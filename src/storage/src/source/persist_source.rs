// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

//! A source that reads from an a persist shard.

use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

use differential_dataflow::Hashable;
use futures_util::Stream as FuturesStream;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::generic::builder_rc::OperatorBuilder;
use timely::dataflow::operators::{Map, OkErr};
use timely::dataflow::{Scope, Stream};
use timely::progress::Antichain;
use timely::PartialOrder;
use tokio::sync::Mutex;
use tracing::trace;

use mz_ore::cast::CastFrom;
use mz_persist::location::ExternalError;
use mz_persist_client::cache::PersistClientCache;
use mz_persist_client::read::{ListenEvent, ReaderEnrichedHollowBatch};
use mz_repr::{Diff, GlobalId, Row, Timestamp};
use mz_timely_util::async_op;
use mz_timely_util::operators_async_ext::OperatorBuilderExt;

use crate::controller::CollectionMetadata;
use crate::types::errors::DataflowError;
use crate::types::sources::SourceData;

/// Creates a new source that reads from a persist shard, distributing the work
/// of reading data to all timely workers.
///
/// TODO: deprecate this method when fixing issues with `persist_source_sharded`.
///
/// All times emitted will have been [advanced by] the given `as_of` frontier.
///
/// [advanced by]: differential_dataflow::lattice::Lattice::advance_by
pub fn persist_source<G>(
    scope: &G,
    source_id: GlobalId,
    persist_clients: Arc<Mutex<PersistClientCache>>,
    metadata: CollectionMetadata,
    as_of: Antichain<Timestamp>,
) -> (
    Stream<G, (Row, Timestamp, Diff)>,
    Stream<G, (DataflowError, Timestamp, Diff)>,
    Rc<dyn Any>,
)
where
    G: Scope<Timestamp = mz_repr::Timestamp>,
{
    let (stream, token) = persist_source_core(scope, source_id, persist_clients, metadata, as_of);
    let (ok_stream, err_stream) = stream.ok_err(|(d, t, r)| match d {
        Ok(row) => Ok((row, t, r)),
        Err(err) => Err((err, t, r)),
    });
    (ok_stream, err_stream, token)
}

/// Creates a new source that reads from a persist shard, distributing the work
/// of reading data to all timely workers.
///
/// Differs from `persist_source` by returning one stream including both values
/// and errors.
///
/// TODO: deprecate this method when fixing issues with `persist_source_sharded`.
///
/// All times emitted will have been [advanced by] the given `as_of` frontier.
///
/// [advanced by]: differential_dataflow::lattice::Lattice::advance_by
pub fn persist_source_core<G>(
    scope: &G,
    source_id: GlobalId,
    persist_clients: Arc<Mutex<PersistClientCache>>,
    metadata: CollectionMetadata,
    as_of: Antichain<Timestamp>,
) -> (
    Stream<G, (Result<Row, DataflowError>, Timestamp, Diff)>,
    Rc<dyn Any>,
)
where
    G: Scope<Timestamp = mz_repr::Timestamp>,
{
    let worker_index = scope.index();
    let peers = scope.peers();
    let chosen_worker = usize::cast_from(source_id.hashed()) % peers;

    // This source is split into two parts: a first part that sets up `async_stream` and a timely
    // source operator that the continuously reads from that stream.
    //
    // It is split that way because there is currently no easy way of setting up an async source
    // operator in materialize/timely.

    // This is a generator that sets up an async `Stream` that can be continously polled to get the
    // values that are `yield`-ed from it's body.
    let async_stream = async_stream::try_stream!({
        // Only one worker is responsible for distributing batches
        if worker_index != chosen_worker {
            trace!(
                "We are not the chosen worker ({}), exiting...",
                chosen_worker
            );
            return;
        }

        let read = persist_clients
            .lock()
            .await
            .open(metadata.persist_location)
            .await
            .expect("could not open persist client")
            .open_reader::<SourceData, (), mz_repr::Timestamp, mz_repr::Diff>(metadata.data_shard)
            .await
            .expect("could not open persist shard");

        let mut subscription = read
            .subscribe(as_of)
            .await
            .expect("cannot serve requested as_of");

        loop {
            for event in subscription.next_listen_events().await {
                yield event;
            }
        }
    });

    let mut pinned_stream = Box::pin(async_stream);

    let (timely_stream, token) =
        crate::source::util::source(scope, "persist_source".to_string(), move |info| {
            let waker_activator = Arc::new(scope.sync_activator_for(&info.address[..]));
            let waker = futures_util::task::waker(waker_activator);

            move |cap_set, output| {
                let mut context = Context::from_waker(&waker);

                while let Poll::Ready(item) = pinned_stream.as_mut().poll_next(&mut context) {
                    match item {
                        Some(Ok(ListenEvent::Progress(upper))) => {
                            cap_set.downgrade(upper.iter());

                            if upper.is_empty() {
                                // Return early because we're done now.
                                return;
                            }
                        }
                        Some(Ok(ListenEvent::Updates(mut updates))) => {
                            // This operator guarantees that its output has been advanced by `as_of.
                            // The persist SnapshotIter already has this contract, so nothing to do
                            // here.

                            if updates.is_empty() {
                                continue;
                            }

                            // Swing through all the capabilities we have and
                            // peel off updates that we can emit with it. For
                            // the case of totally ordered times, we have at
                            // most on capability and will peel off all updates
                            // in one go.
                            //
                            // NOTE: We use this seemingly complicated approach
                            // such that the code is ready to deal with
                            // partially ordered times.
                            for cap in cap_set.iter() {
                                // NOTE: The nightly `drain_filter()` would use
                                // less allocations than this. Should switch to
                                // it once it's available in stable rust.
                                let (mut to_emit, remaining_updates) =
                                    updates.into_iter().partition(|(_update, ts, _diff)| {
                                        PartialOrder::less_equal(cap.time(), ts)
                                    });
                                updates = remaining_updates;

                                let mut session = output.session(&cap);
                                session.give_vec(&mut to_emit);
                            }

                            assert!(
                                updates.is_empty(),
                                "did not have matching Capability for updates: {:?}",
                                updates
                            );
                        }
                        Some(Err::<_, ExternalError>(e)) => {
                            // TODO(petrosagg): error handling
                            panic!("unexpected error from persist {e}");
                        }
                        None => {
                            // Empty out the `CapabilitySet` to indicate that we're done.
                            cap_set.downgrade(&[]);
                            return;
                        }
                    }
                }
            }
        });

    let stream = timely_stream.map(|x| match x {
        ((Ok(SourceData(Ok(row))), Ok(())), ts, diff) => (Ok(row), ts, diff),
        ((Ok(SourceData(Err(err))), Ok(())), ts, diff) => (Err(err), ts, diff),
        // TODO(petrosagg): error handling
        _ => panic!("decoding failed"),
    });

    let token = Rc::new(token);

    (stream, token)
}

/// Creates a new source that reads from a persist shard, distributing the work
/// of reading data to all timely workers.
///
/// TODO: fix issue of using multiple `ReadHandles`.
///
/// All times emitted will have been [advanced by] the given `as_of` frontier.
///
/// [advanced by]: differential_dataflow::lattice::Lattice::advance_by
pub fn persist_source_sharded<G>(
    scope: &G,
    source_id: GlobalId,
    persist_clients: Arc<Mutex<PersistClientCache>>,
    metadata: CollectionMetadata,
    as_of: Antichain<Timestamp>,
) -> (
    Stream<G, (Row, Timestamp, Diff)>,
    Stream<G, (DataflowError, Timestamp, Diff)>,
    Rc<dyn Any>,
)
where
    G: Scope<Timestamp = mz_repr::Timestamp>,
{
    let worker_index = scope.index();
    let peers = scope.peers();
    let chosen_worker = usize::cast_from(source_id.hashed()) % peers;

    let persist_clients_stream = Arc::<Mutex<PersistClientCache>>::clone(&persist_clients);
    let persist_location_stream = metadata.persist_location.clone();
    let data_shard = metadata.data_shard.clone();
    let as_of_stream = as_of;

    // This source is split as such:
    // 1. Sets up `async_stream`, which only yields data (hollow batches) on one
    //    worker.
    // 2. A timely source operator which continuously reads from that stream,
    //    and distributes the data among workers.
    // 3. A timely operator which downloads the batch's contents from S3, and
    //    outputs them to a timely stream.

    // This is a generator that sets up an async `Stream` that can be continously polled to get the
    // values that are `yield`-ed from it's body.
    let async_stream = async_stream::try_stream!({
        // Only one worker is responsible for distributing batches
        if worker_index != chosen_worker {
            trace!(
                "We are not the chosen worker ({}), exiting...",
                chosen_worker
            );
            return;
        }

        let read = persist_clients_stream
            .lock()
            .await
            .open(persist_location_stream)
            .await
            .expect("could not open persist client")
            .open_reader::<SourceData, (), mz_repr::Timestamp, mz_repr::Diff>(data_shard.clone())
            .await
            .expect("could not open persist shard");

        let mut subscription = read
            .subscribe(as_of_stream)
            .await
            .expect("cannot serve requested as_of");

        loop {
            yield subscription.next().await;
        }
    });

    let mut pinned_stream = Box::pin(async_stream);

    let (inner, token) =
        crate::source::util::source(scope, "persist_source".to_string(), move |info| {
            let waker_activator = Arc::new(scope.sync_activator_for(&info.address[..]));
            let waker = futures_util::task::waker(waker_activator);

            let mut current_ts = 0;

            // `i` gets used to round-robin distribution of hollow batches. We
            // start at a different worker for each source, so as to prevent
            // sources started at the same time from distributing sources in
            // lock step with one another.
            let mut i = usize::cast_from(source_id.hashed()) % peers;

            move |cap_set, output| {
                let mut context = Context::from_waker(&waker);

                while let Poll::Ready(item) = pinned_stream.as_mut().poll_next(&mut context) {
                    match item {
                        Some(Ok(batch)) => {
                            let session_cap = cap_set.delayed(&current_ts);
                            let mut session = output.session(&session_cap);
                            let progress = batch.generate_progress();

                            session.give((i, batch));

                            // Round robin
                            i = (i + 1) % peers;
                            if let Some(frontier) = progress {
                                cap_set.downgrade(frontier.iter());
                                match frontier.into_option() {
                                    Some(ts) => {
                                        current_ts = ts;
                                    }
                                    None => {
                                        cap_set.downgrade(&[]);
                                        return;
                                    }
                                }
                            }
                        }
                        Some(Err::<_, ExternalError>(e)) => {
                            panic!("unexpected error from persist {e}")
                        }
                        // We never expect any further output from
                        // `pinned_stream`, so propagate that information
                        // downstream.
                        None => {
                            cap_set.downgrade(&[]);
                            return;
                        }
                    }
                }
            }
        });

    let mut builder = OperatorBuilder::new(
        format!(
            "persist_source: sharded reader {} of {:?}",
            worker_index, source_id
        ),
        scope.clone(),
    );
    let dist = |&(i, _): &(usize, ReaderEnrichedHollowBatch<Timestamp>)| u64::cast_from(i);
    let mut input = builder.new_input(&inner, Exchange::new(dist));
    let (mut output, output_stream) = builder.new_output();

    builder.build_async(
        scope.clone(),
        async_op!(|initial_capabilities, _frontiers| {
            let mut read = persist_clients
                .lock()
                .await
                .open(metadata.persist_location.clone())
                .await
                .expect("could not open persist client")
                .open_reader::<SourceData, (), mz_repr::Timestamp, mz_repr::Diff>(
                    data_shard.clone(),
                )
                .await
                .expect("could not open persist shard");

            initial_capabilities.clear();

            let mut output_handle = output.activate();

            while let Some((cap, data)) = input.next() {
                let cap = cap.retain();
                for (_idx, batch) in data.iter() {
                    let mut updates = read
                        .fetch_batch(batch.clone())
                        .await
                        .expect("shard_id generated for sources must match across all workers");

                    let mut session = output_handle.session(&cap);
                    session.give_vec(&mut updates);
                }
            }
            false
        }),
    );

    let (ok_stream, err_stream) = output_stream.ok_err(|x| match x {
        ((Ok(SourceData(Ok(row))), Ok(())), ts, diff) => Ok((row, ts, diff)),
        ((Ok(SourceData(Err(err))), Ok(())), ts, diff) => Err((err, ts, diff)),
        // TODO(petrosagg): error handling
        _ => panic!("decoding failed"),
    });

    let token = Rc::new(token);

    (ok_stream, err_stream, token)
}
