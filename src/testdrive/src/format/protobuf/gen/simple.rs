// This file is generated by rust-protobuf 2.16.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `src/format/protobuf/simple.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_16_2;

#[derive(PartialEq,Clone,Default)]
pub struct Struct {
    // message fields
    pub int: i32,
    pub bad_int: i32,
    pub bin: Binary,
    pub st: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Struct {
    fn default() -> &'a Struct {
        <Struct as ::protobuf::Message>::default_instance()
    }
}

impl Struct {
    pub fn new() -> Struct {
        ::std::default::Default::default()
    }

    // sint32 int = 1;


    pub fn get_int(&self) -> i32 {
        self.int
    }
    pub fn clear_int(&mut self) {
        self.int = 0;
    }

    // Param is passed by value, moved
    pub fn set_int(&mut self, v: i32) {
        self.int = v;
    }

    // int32 bad_int = 2;


    pub fn get_bad_int(&self) -> i32 {
        self.bad_int
    }
    pub fn clear_bad_int(&mut self) {
        self.bad_int = 0;
    }

    // Param is passed by value, moved
    pub fn set_bad_int(&mut self, v: i32) {
        self.bad_int = v;
    }

    // .Binary bin = 3;


    pub fn get_bin(&self) -> Binary {
        self.bin
    }
    pub fn clear_bin(&mut self) {
        self.bin = Binary::ZERO;
    }

    // Param is passed by value, moved
    pub fn set_bin(&mut self, v: Binary) {
        self.bin = v;
    }

    // string st = 4;


    pub fn get_st(&self) -> &str {
        &self.st
    }
    pub fn clear_st(&mut self) {
        self.st.clear();
    }

    // Param is passed by value, moved
    pub fn set_st(&mut self, v: ::std::string::String) {
        self.st = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_st(&mut self) -> &mut ::std::string::String {
        &mut self.st
    }

    // Take field
    pub fn take_st(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.st, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Struct {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.int = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.bad_int = tmp;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.bin, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.st)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.int != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, self.int);
        }
        if self.bad_int != 0 {
            my_size += ::protobuf::rt::value_size(2, self.bad_int, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bin != Binary::ZERO {
            my_size += ::protobuf::rt::enum_size(3, self.bin);
        }
        if !self.st.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.st);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.int != 0 {
            os.write_sint32(1, self.int)?;
        }
        if self.bad_int != 0 {
            os.write_int32(2, self.bad_int)?;
        }
        if self.bin != Binary::ZERO {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.bin))?;
        }
        if !self.st.is_empty() {
            os.write_string(4, &self.st)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Struct {
        Struct::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                "int",
                |m: &Struct| { &m.int },
                |m: &mut Struct| { &mut m.int },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "bad_int",
                |m: &Struct| { &m.bad_int },
                |m: &mut Struct| { &mut m.bad_int },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Binary>>(
                "bin",
                |m: &Struct| { &m.bin },
                |m: &mut Struct| { &mut m.bin },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "st",
                |m: &Struct| { &m.st },
                |m: &mut Struct| { &mut m.st },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Struct>(
                "Struct",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Struct {
        static instance: ::protobuf::rt::LazyV2<Struct> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Struct::new)
    }
}

impl ::protobuf::Clear for Struct {
    fn clear(&mut self) {
        self.int = 0;
        self.bad_int = 0;
        self.bin = Binary::ZERO;
        self.st.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Struct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Struct {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RepeatedSimple {
    // message fields
    pub ints: ::std::vec::Vec<i64>,
    pub strings: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RepeatedSimple {
    fn default() -> &'a RepeatedSimple {
        <RepeatedSimple as ::protobuf::Message>::default_instance()
    }
}

impl RepeatedSimple {
    pub fn new() -> RepeatedSimple {
        ::std::default::Default::default()
    }

    // repeated sint64 ints = 1;


    pub fn get_ints(&self) -> &[i64] {
        &self.ints
    }
    pub fn clear_ints(&mut self) {
        self.ints.clear();
    }

    // Param is passed by value, moved
    pub fn set_ints(&mut self, v: ::std::vec::Vec<i64>) {
        self.ints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ints(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.ints
    }

    // Take field
    pub fn take_ints(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.ints, ::std::vec::Vec::new())
    }

    // repeated string strings = 2;


    pub fn get_strings(&self) -> &[::std::string::String] {
        &self.strings
    }
    pub fn clear_strings(&mut self) {
        self.strings.clear();
    }

    // Param is passed by value, moved
    pub fn set_strings(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.strings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_strings(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.strings
    }

    // Take field
    pub fn take_strings(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.strings, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for RepeatedSimple {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.ints)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.strings)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ints {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        for value in &self.strings {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ints {
            os.write_sint64(1, *v)?;
        };
        for v in &self.strings {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RepeatedSimple {
        RepeatedSimple::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                "ints",
                |m: &RepeatedSimple| { &m.ints },
                |m: &mut RepeatedSimple| { &mut m.ints },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "strings",
                |m: &RepeatedSimple| { &m.strings },
                |m: &mut RepeatedSimple| { &mut m.strings },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RepeatedSimple>(
                "RepeatedSimple",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RepeatedSimple {
        static instance: ::protobuf::rt::LazyV2<RepeatedSimple> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RepeatedSimple::new)
    }
}

impl ::protobuf::Clear for RepeatedSimple {
    fn clear(&mut self) {
        self.ints.clear();
        self.strings.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RepeatedSimple {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RepeatedSimple {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RepeatedStruct {
    // message fields
    pub struct_field: ::protobuf::RepeatedField<Struct>,
    pub st_repeated: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RepeatedStruct {
    fn default() -> &'a RepeatedStruct {
        <RepeatedStruct as ::protobuf::Message>::default_instance()
    }
}

impl RepeatedStruct {
    pub fn new() -> RepeatedStruct {
        ::std::default::Default::default()
    }

    // repeated .Struct struct_field = 1;


    pub fn get_struct_field(&self) -> &[Struct] {
        &self.struct_field
    }
    pub fn clear_struct_field(&mut self) {
        self.struct_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_struct_field(&mut self, v: ::protobuf::RepeatedField<Struct>) {
        self.struct_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_struct_field(&mut self) -> &mut ::protobuf::RepeatedField<Struct> {
        &mut self.struct_field
    }

    // Take field
    pub fn take_struct_field(&mut self) -> ::protobuf::RepeatedField<Struct> {
        ::std::mem::replace(&mut self.struct_field, ::protobuf::RepeatedField::new())
    }

    // repeated string st_repeated = 2;


    pub fn get_st_repeated(&self) -> &[::std::string::String] {
        &self.st_repeated
    }
    pub fn clear_st_repeated(&mut self) {
        self.st_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_st_repeated(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.st_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_st_repeated(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.st_repeated
    }

    // Take field
    pub fn take_st_repeated(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.st_repeated, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for RepeatedStruct {
    fn is_initialized(&self) -> bool {
        for v in &self.struct_field {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.struct_field)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.st_repeated)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.struct_field {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.st_repeated {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.struct_field {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.st_repeated {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RepeatedStruct {
        RepeatedStruct::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Struct>>(
                "struct_field",
                |m: &RepeatedStruct| { &m.struct_field },
                |m: &mut RepeatedStruct| { &mut m.struct_field },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "st_repeated",
                |m: &RepeatedStruct| { &m.st_repeated },
                |m: &mut RepeatedStruct| { &mut m.st_repeated },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RepeatedStruct>(
                "RepeatedStruct",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RepeatedStruct {
        static instance: ::protobuf::rt::LazyV2<RepeatedStruct> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RepeatedStruct::new)
    }
}

impl ::protobuf::Clear for RepeatedStruct {
    fn clear(&mut self) {
        self.struct_field.clear();
        self.st_repeated.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RepeatedStruct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RepeatedStruct {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Binary {
    ZERO = 0,
    ONE = 1,
}

impl ::protobuf::ProtobufEnum for Binary {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Binary> {
        match value {
            0 => ::std::option::Option::Some(Binary::ZERO),
            1 => ::std::option::Option::Some(Binary::ONE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Binary] = &[
            Binary::ZERO,
            Binary::ONE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Binary>("Binary", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Binary {
}

impl ::std::default::Default for Binary {
    fn default() -> Self {
        Binary::ZERO
    }
}

impl ::protobuf::reflect::ProtobufValue for Binary {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20src/format/protobuf/simple.proto\"^\n\x06Struct\x12\x10\n\x03int\
    \x18\x01\x20\x01(\x11R\x03int\x12\x17\n\x07bad_int\x18\x02\x20\x01(\x05R\
    \x06badInt\x12\x19\n\x03bin\x18\x03\x20\x01(\x0e2\x07.BinaryR\x03bin\x12\
    \x0e\n\x02st\x18\x04\x20\x01(\tR\x02st\">\n\x0eRepeatedSimple\x12\x12\n\
    \x04ints\x18\x01\x20\x03(\x12R\x04ints\x12\x18\n\x07strings\x18\x02\x20\
    \x03(\tR\x07strings\"]\n\x0eRepeatedStruct\x12*\n\x0cstruct_field\x18\
    \x01\x20\x03(\x0b2\x07.StructR\x0bstructField\x12\x1f\n\x0bst_repeated\
    \x18\x02\x20\x03(\tR\nstRepeated*\x1b\n\x06Binary\x12\x08\n\x04ZERO\x10\
    \0\x12\x07\n\x03ONE\x10\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
