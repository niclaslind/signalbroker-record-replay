// This file is generated by rust-protobuf 2.17.0. Do not edit
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
//! Generated file from `functional_api.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
pub struct SenderInfo {
    // message fields
    pub clientId: ::protobuf::SingularPtrField<super::common::ClientId>,
    pub value: ::protobuf::SingularPtrField<Value>,
    pub frequency: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SenderInfo {
    fn default() -> &'a SenderInfo {
        <SenderInfo as ::protobuf::Message>::default_instance()
    }
}

impl SenderInfo {
    pub fn new() -> SenderInfo {
        ::std::default::Default::default()
    }

    // .base.ClientId clientId = 1;


    pub fn get_clientId(&self) -> &super::common::ClientId {
        self.clientId.as_ref().unwrap_or_else(|| <super::common::ClientId as ::protobuf::Message>::default_instance())
    }
    pub fn clear_clientId(&mut self) {
        self.clientId.clear();
    }

    pub fn has_clientId(&self) -> bool {
        self.clientId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientId(&mut self, v: super::common::ClientId) {
        self.clientId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientId(&mut self) -> &mut super::common::ClientId {
        if self.clientId.is_none() {
            self.clientId.set_default();
        }
        self.clientId.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientId(&mut self) -> super::common::ClientId {
        self.clientId.take().unwrap_or_else(|| super::common::ClientId::new())
    }

    // .base.Value value = 2;


    pub fn get_value(&self) -> &Value {
        self.value.as_ref().unwrap_or_else(|| <Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: Value) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut Value {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    // int32 frequency = 3;


    pub fn get_frequency(&self) -> i32 {
        self.frequency
    }
    pub fn clear_frequency(&mut self) {
        self.frequency = 0;
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: i32) {
        self.frequency = v;
    }
}

impl ::protobuf::Message for SenderInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.clientId {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.value {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clientId)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.frequency = tmp;
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
        if let Some(ref v) = self.clientId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.frequency != 0 {
            my_size += ::protobuf::rt::value_size(3, self.frequency, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.clientId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.frequency != 0 {
            os.write_int32(3, self.frequency)?;
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

    fn new() -> SenderInfo {
        SenderInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ClientId>>(
                "clientId",
                |m: &SenderInfo| { &m.clientId },
                |m: &mut SenderInfo| { &mut m.clientId },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                "value",
                |m: &SenderInfo| { &m.value },
                |m: &mut SenderInfo| { &mut m.value },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "frequency",
                |m: &SenderInfo| { &m.frequency },
                |m: &mut SenderInfo| { &mut m.frequency },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SenderInfo>(
                "SenderInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SenderInfo {
        static instance: ::protobuf::rt::LazyV2<SenderInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SenderInfo::new)
    }
}

impl ::protobuf::Clear for SenderInfo {
    fn clear(&mut self) {
        self.clientId.clear();
        self.value.clear();
        self.frequency = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SenderInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SenderInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubscriberRequest {
    // message fields
    pub clientId: ::protobuf::SingularPtrField<super::common::ClientId>,
    pub onChange: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SubscriberRequest {
    fn default() -> &'a SubscriberRequest {
        <SubscriberRequest as ::protobuf::Message>::default_instance()
    }
}

impl SubscriberRequest {
    pub fn new() -> SubscriberRequest {
        ::std::default::Default::default()
    }

    // .base.ClientId clientId = 1;


    pub fn get_clientId(&self) -> &super::common::ClientId {
        self.clientId.as_ref().unwrap_or_else(|| <super::common::ClientId as ::protobuf::Message>::default_instance())
    }
    pub fn clear_clientId(&mut self) {
        self.clientId.clear();
    }

    pub fn has_clientId(&self) -> bool {
        self.clientId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientId(&mut self, v: super::common::ClientId) {
        self.clientId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientId(&mut self) -> &mut super::common::ClientId {
        if self.clientId.is_none() {
            self.clientId.set_default();
        }
        self.clientId.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientId(&mut self) -> super::common::ClientId {
        self.clientId.take().unwrap_or_else(|| super::common::ClientId::new())
    }

    // bool onChange = 2;


    pub fn get_onChange(&self) -> bool {
        self.onChange
    }
    pub fn clear_onChange(&mut self) {
        self.onChange = false;
    }

    // Param is passed by value, moved
    pub fn set_onChange(&mut self, v: bool) {
        self.onChange = v;
    }
}

impl ::protobuf::Message for SubscriberRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.clientId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clientId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.onChange = tmp;
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
        if let Some(ref v) = self.clientId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.onChange != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.clientId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.onChange != false {
            os.write_bool(2, self.onChange)?;
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

    fn new() -> SubscriberRequest {
        SubscriberRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ClientId>>(
                "clientId",
                |m: &SubscriberRequest| { &m.clientId },
                |m: &mut SubscriberRequest| { &mut m.clientId },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "onChange",
                |m: &SubscriberRequest| { &m.onChange },
                |m: &mut SubscriberRequest| { &mut m.onChange },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SubscriberRequest>(
                "SubscriberRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SubscriberRequest {
        static instance: ::protobuf::rt::LazyV2<SubscriberRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SubscriberRequest::new)
    }
}

impl ::protobuf::Clear for SubscriberRequest {
    fn clear(&mut self) {
        self.clientId.clear();
        self.onChange = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscriberRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscriberRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Value {
    // message fields
    pub payload: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Value {
    fn default() -> &'a Value {
        <Value as ::protobuf::Message>::default_instance()
    }
}

impl Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    // int32 payload = 1;


    pub fn get_payload(&self) -> i32 {
        self.payload
    }
    pub fn clear_payload(&mut self) {
        self.payload = 0;
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: i32) {
        self.payload = v;
    }
}

impl ::protobuf::Message for Value {
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
                    let tmp = is.read_int32()?;
                    self.payload = tmp;
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
        if self.payload != 0 {
            my_size += ::protobuf::rt::value_size(1, self.payload, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.payload != 0 {
            os.write_int32(1, self.payload)?;
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

    fn new() -> Value {
        Value::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "payload",
                |m: &Value| { &m.payload },
                |m: &mut Value| { &mut m.payload },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Value>(
                "Value",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Value {
        static instance: ::protobuf::rt::LazyV2<Value> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Value::new)
    }
}

impl ::protobuf::Clear for Value {
    fn clear(&mut self) {
        self.payload = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Value {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14functional_api.proto\x12\x04base\x1a\x0ccommon.proto\"y\n\nSenderI\
    nfo\x12*\n\x08clientId\x18\x01\x20\x01(\x0b2\x0e.base.ClientIdR\x08clien\
    tId\x12!\n\x05value\x18\x02\x20\x01(\x0b2\x0b.base.ValueR\x05value\x12\
    \x1c\n\tfrequency\x18\x03\x20\x01(\x05R\tfrequency\"[\n\x11SubscriberReq\
    uest\x12*\n\x08clientId\x18\x01\x20\x01(\x0b2\x0e.base.ClientIdR\x08clie\
    ntId\x12\x1a\n\x08onChange\x18\x02\x20\x01(\x08R\x08onChange\"!\n\x05Val\
    ue\x12\x18\n\x07payload\x18\x01\x20\x01(\x05R\x07payload2\xe7\x01\n\x11F\
    unctionalService\x12/\n\x0eOpenPassWindow\x12\x0e.base.ClientId\x1a\x0b.\
    base.Empty\"\0\x120\n\x0fClosePassWindow\x12\x0e.base.ClientId\x1a\x0b.b\
    ase.Empty\"\0\x12.\n\x0bSetFanSpeed\x12\x10.base.SenderInfo\x1a\x0b.base\
    .Empty\"\0\x12?\n\x13SubscribeToFanSpeed\x12\x17.base.SubscriberRequest\
    \x1a\x0b.base.Value\"\00\x01b\x06proto3\
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
