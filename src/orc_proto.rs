// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct IntegerStatistics {
    // message fields
    minimum: ::std::option::Option<i64>,
    maximum: ::std::option::Option<i64>,
    sum: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IntegerStatistics {}

impl IntegerStatistics {
    pub fn new() -> IntegerStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IntegerStatistics {
        static mut instance: ::protobuf::lazy::Lazy<IntegerStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IntegerStatistics,
        };
        unsafe {
            instance.get(IntegerStatistics::new)
        }
    }

    // optional sint64 minimum = 1;

    pub fn clear_minimum(&mut self) {
        self.minimum = ::std::option::Option::None;
    }

    pub fn has_minimum(&self) -> bool {
        self.minimum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum(&mut self, v: i64) {
        self.minimum = ::std::option::Option::Some(v);
    }

    pub fn get_minimum(&self) -> i64 {
        self.minimum.unwrap_or(0)
    }

    fn get_minimum_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.minimum
    }

    fn mut_minimum_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.minimum
    }

    // optional sint64 maximum = 2;

    pub fn clear_maximum(&mut self) {
        self.maximum = ::std::option::Option::None;
    }

    pub fn has_maximum(&self) -> bool {
        self.maximum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum(&mut self, v: i64) {
        self.maximum = ::std::option::Option::Some(v);
    }

    pub fn get_maximum(&self) -> i64 {
        self.maximum.unwrap_or(0)
    }

    fn get_maximum_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.maximum
    }

    fn mut_maximum_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.maximum
    }

    // optional sint64 sum = 3;

    pub fn clear_sum(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_sum(&self) -> bool {
        self.sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: i64) {
        self.sum = ::std::option::Option::Some(v);
    }

    pub fn get_sum(&self) -> i64 {
        self.sum.unwrap_or(0)
    }

    fn get_sum_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.sum
    }
}

impl ::protobuf::Message for IntegerStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.minimum = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.maximum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.sum = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.minimum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(v) = self.maximum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        }
        if let Some(v) = self.sum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.minimum {
            os.write_sint64(1, v)?;
        }
        if let Some(v) = self.maximum {
            os.write_sint64(2, v)?;
        }
        if let Some(v) = self.sum {
            os.write_sint64(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IntegerStatistics {
    fn new() -> IntegerStatistics {
        IntegerStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<IntegerStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "minimum",
                    IntegerStatistics::get_minimum_for_reflect,
                    IntegerStatistics::mut_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "maximum",
                    IntegerStatistics::get_maximum_for_reflect,
                    IntegerStatistics::mut_maximum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "sum",
                    IntegerStatistics::get_sum_for_reflect,
                    IntegerStatistics::mut_sum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IntegerStatistics>(
                    "IntegerStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IntegerStatistics {
    fn clear(&mut self) {
        self.clear_minimum();
        self.clear_maximum();
        self.clear_sum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IntegerStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IntegerStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoubleStatistics {
    // message fields
    minimum: ::std::option::Option<f64>,
    maximum: ::std::option::Option<f64>,
    sum: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoubleStatistics {}

impl DoubleStatistics {
    pub fn new() -> DoubleStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoubleStatistics {
        static mut instance: ::protobuf::lazy::Lazy<DoubleStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoubleStatistics,
        };
        unsafe {
            instance.get(DoubleStatistics::new)
        }
    }

    // optional double minimum = 1;

    pub fn clear_minimum(&mut self) {
        self.minimum = ::std::option::Option::None;
    }

    pub fn has_minimum(&self) -> bool {
        self.minimum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum(&mut self, v: f64) {
        self.minimum = ::std::option::Option::Some(v);
    }

    pub fn get_minimum(&self) -> f64 {
        self.minimum.unwrap_or(0.)
    }

    fn get_minimum_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.minimum
    }

    fn mut_minimum_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.minimum
    }

    // optional double maximum = 2;

    pub fn clear_maximum(&mut self) {
        self.maximum = ::std::option::Option::None;
    }

    pub fn has_maximum(&self) -> bool {
        self.maximum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum(&mut self, v: f64) {
        self.maximum = ::std::option::Option::Some(v);
    }

    pub fn get_maximum(&self) -> f64 {
        self.maximum.unwrap_or(0.)
    }

    fn get_maximum_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.maximum
    }

    fn mut_maximum_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.maximum
    }

    // optional double sum = 3;

    pub fn clear_sum(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_sum(&self) -> bool {
        self.sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: f64) {
        self.sum = ::std::option::Option::Some(v);
    }

    pub fn get_sum(&self) -> f64 {
        self.sum.unwrap_or(0.)
    }

    fn get_sum_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.sum
    }
}

impl ::protobuf::Message for DoubleStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.minimum = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.maximum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sum = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.minimum {
            my_size += 9;
        }
        if let Some(v) = self.maximum {
            my_size += 9;
        }
        if let Some(v) = self.sum {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.minimum {
            os.write_double(1, v)?;
        }
        if let Some(v) = self.maximum {
            os.write_double(2, v)?;
        }
        if let Some(v) = self.sum {
            os.write_double(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DoubleStatistics {
    fn new() -> DoubleStatistics {
        DoubleStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoubleStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "minimum",
                    DoubleStatistics::get_minimum_for_reflect,
                    DoubleStatistics::mut_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "maximum",
                    DoubleStatistics::get_maximum_for_reflect,
                    DoubleStatistics::mut_maximum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "sum",
                    DoubleStatistics::get_sum_for_reflect,
                    DoubleStatistics::mut_sum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoubleStatistics>(
                    "DoubleStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoubleStatistics {
    fn clear(&mut self) {
        self.clear_minimum();
        self.clear_maximum();
        self.clear_sum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoubleStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoubleStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StringStatistics {
    // message fields
    minimum: ::protobuf::SingularField<::std::string::String>,
    maximum: ::protobuf::SingularField<::std::string::String>,
    sum: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StringStatistics {}

impl StringStatistics {
    pub fn new() -> StringStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringStatistics {
        static mut instance: ::protobuf::lazy::Lazy<StringStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringStatistics,
        };
        unsafe {
            instance.get(StringStatistics::new)
        }
    }

    // optional string minimum = 1;

    pub fn clear_minimum(&mut self) {
        self.minimum.clear();
    }

    pub fn has_minimum(&self) -> bool {
        self.minimum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum(&mut self, v: ::std::string::String) {
        self.minimum = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_minimum(&mut self) -> &mut ::std::string::String {
        if self.minimum.is_none() {
            self.minimum.set_default();
        }
        self.minimum.as_mut().unwrap()
    }

    // Take field
    pub fn take_minimum(&mut self) -> ::std::string::String {
        self.minimum.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_minimum(&self) -> &str {
        match self.minimum.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_minimum_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.minimum
    }

    fn mut_minimum_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.minimum
    }

    // optional string maximum = 2;

    pub fn clear_maximum(&mut self) {
        self.maximum.clear();
    }

    pub fn has_maximum(&self) -> bool {
        self.maximum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum(&mut self, v: ::std::string::String) {
        self.maximum = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maximum(&mut self) -> &mut ::std::string::String {
        if self.maximum.is_none() {
            self.maximum.set_default();
        }
        self.maximum.as_mut().unwrap()
    }

    // Take field
    pub fn take_maximum(&mut self) -> ::std::string::String {
        self.maximum.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_maximum(&self) -> &str {
        match self.maximum.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_maximum_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.maximum
    }

    fn mut_maximum_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.maximum
    }

    // optional sint64 sum = 3;

    pub fn clear_sum(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_sum(&self) -> bool {
        self.sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: i64) {
        self.sum = ::std::option::Option::Some(v);
    }

    pub fn get_sum(&self) -> i64 {
        self.sum.unwrap_or(0)
    }

    fn get_sum_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.sum
    }
}

impl ::protobuf::Message for StringStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.minimum)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.maximum)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.sum = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.minimum.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.maximum.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.sum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.minimum.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.maximum.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.sum {
            os.write_sint64(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StringStatistics {
    fn new() -> StringStatistics {
        StringStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<StringStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "minimum",
                    StringStatistics::get_minimum_for_reflect,
                    StringStatistics::mut_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "maximum",
                    StringStatistics::get_maximum_for_reflect,
                    StringStatistics::mut_maximum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "sum",
                    StringStatistics::get_sum_for_reflect,
                    StringStatistics::mut_sum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StringStatistics>(
                    "StringStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StringStatistics {
    fn clear(&mut self) {
        self.clear_minimum();
        self.clear_maximum();
        self.clear_sum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StringStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StringStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BucketStatistics {
    // message fields
    count: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BucketStatistics {}

impl BucketStatistics {
    pub fn new() -> BucketStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BucketStatistics {
        static mut instance: ::protobuf::lazy::Lazy<BucketStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BucketStatistics,
        };
        unsafe {
            instance.get(BucketStatistics::new)
        }
    }

    // repeated uint64 count = 1;

    pub fn clear_count(&mut self) {
        self.count.clear();
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: ::std::vec::Vec<u64>) {
        self.count = v;
    }

    // Mutable pointer to the field.
    pub fn mut_count(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.count
    }

    // Take field
    pub fn take_count(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.count, ::std::vec::Vec::new())
    }

    pub fn get_count(&self) -> &[u64] {
        &self.count
    }

    fn get_count_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.count
    }
}

impl ::protobuf::Message for BucketStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.count)?;
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
        if !self.count.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.count);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.count.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.count))?;
            for v in &self.count {
                os.write_uint64_no_tag(*v)?;
            };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BucketStatistics {
    fn new() -> BucketStatistics {
        BucketStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<BucketStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    BucketStatistics::get_count_for_reflect,
                    BucketStatistics::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BucketStatistics>(
                    "BucketStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BucketStatistics {
    fn clear(&mut self) {
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BucketStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BucketStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DecimalStatistics {
    // message fields
    minimum: ::protobuf::SingularField<::std::string::String>,
    maximum: ::protobuf::SingularField<::std::string::String>,
    sum: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DecimalStatistics {}

impl DecimalStatistics {
    pub fn new() -> DecimalStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DecimalStatistics {
        static mut instance: ::protobuf::lazy::Lazy<DecimalStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DecimalStatistics,
        };
        unsafe {
            instance.get(DecimalStatistics::new)
        }
    }

    // optional string minimum = 1;

    pub fn clear_minimum(&mut self) {
        self.minimum.clear();
    }

    pub fn has_minimum(&self) -> bool {
        self.minimum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum(&mut self, v: ::std::string::String) {
        self.minimum = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_minimum(&mut self) -> &mut ::std::string::String {
        if self.minimum.is_none() {
            self.minimum.set_default();
        }
        self.minimum.as_mut().unwrap()
    }

    // Take field
    pub fn take_minimum(&mut self) -> ::std::string::String {
        self.minimum.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_minimum(&self) -> &str {
        match self.minimum.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_minimum_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.minimum
    }

    fn mut_minimum_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.minimum
    }

    // optional string maximum = 2;

    pub fn clear_maximum(&mut self) {
        self.maximum.clear();
    }

    pub fn has_maximum(&self) -> bool {
        self.maximum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum(&mut self, v: ::std::string::String) {
        self.maximum = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maximum(&mut self) -> &mut ::std::string::String {
        if self.maximum.is_none() {
            self.maximum.set_default();
        }
        self.maximum.as_mut().unwrap()
    }

    // Take field
    pub fn take_maximum(&mut self) -> ::std::string::String {
        self.maximum.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_maximum(&self) -> &str {
        match self.maximum.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_maximum_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.maximum
    }

    fn mut_maximum_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.maximum
    }

    // optional string sum = 3;

    pub fn clear_sum(&mut self) {
        self.sum.clear();
    }

    pub fn has_sum(&self) -> bool {
        self.sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: ::std::string::String) {
        self.sum = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sum(&mut self) -> &mut ::std::string::String {
        if self.sum.is_none() {
            self.sum.set_default();
        }
        self.sum.as_mut().unwrap()
    }

    // Take field
    pub fn take_sum(&mut self) -> ::std::string::String {
        self.sum.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sum(&self) -> &str {
        match self.sum.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sum_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sum
    }
}

impl ::protobuf::Message for DecimalStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.minimum)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.maximum)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sum)?;
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
        if let Some(ref v) = self.minimum.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.maximum.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.sum.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.minimum.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.maximum.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.sum.as_ref() {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DecimalStatistics {
    fn new() -> DecimalStatistics {
        DecimalStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<DecimalStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "minimum",
                    DecimalStatistics::get_minimum_for_reflect,
                    DecimalStatistics::mut_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "maximum",
                    DecimalStatistics::get_maximum_for_reflect,
                    DecimalStatistics::mut_maximum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sum",
                    DecimalStatistics::get_sum_for_reflect,
                    DecimalStatistics::mut_sum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DecimalStatistics>(
                    "DecimalStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DecimalStatistics {
    fn clear(&mut self) {
        self.clear_minimum();
        self.clear_maximum();
        self.clear_sum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DecimalStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DecimalStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DateStatistics {
    // message fields
    minimum: ::std::option::Option<i32>,
    maximum: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DateStatistics {}

impl DateStatistics {
    pub fn new() -> DateStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DateStatistics {
        static mut instance: ::protobuf::lazy::Lazy<DateStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DateStatistics,
        };
        unsafe {
            instance.get(DateStatistics::new)
        }
    }

    // optional sint32 minimum = 1;

    pub fn clear_minimum(&mut self) {
        self.minimum = ::std::option::Option::None;
    }

    pub fn has_minimum(&self) -> bool {
        self.minimum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum(&mut self, v: i32) {
        self.minimum = ::std::option::Option::Some(v);
    }

    pub fn get_minimum(&self) -> i32 {
        self.minimum.unwrap_or(0)
    }

    fn get_minimum_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.minimum
    }

    fn mut_minimum_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.minimum
    }

    // optional sint32 maximum = 2;

    pub fn clear_maximum(&mut self) {
        self.maximum = ::std::option::Option::None;
    }

    pub fn has_maximum(&self) -> bool {
        self.maximum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum(&mut self, v: i32) {
        self.maximum = ::std::option::Option::Some(v);
    }

    pub fn get_maximum(&self) -> i32 {
        self.maximum.unwrap_or(0)
    }

    fn get_maximum_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.maximum
    }

    fn mut_maximum_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.maximum
    }
}

impl ::protobuf::Message for DateStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.minimum = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.maximum = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.minimum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(v) = self.maximum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.minimum {
            os.write_sint32(1, v)?;
        }
        if let Some(v) = self.maximum {
            os.write_sint32(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DateStatistics {
    fn new() -> DateStatistics {
        DateStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<DateStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "minimum",
                    DateStatistics::get_minimum_for_reflect,
                    DateStatistics::mut_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "maximum",
                    DateStatistics::get_maximum_for_reflect,
                    DateStatistics::mut_maximum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DateStatistics>(
                    "DateStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DateStatistics {
    fn clear(&mut self) {
        self.clear_minimum();
        self.clear_maximum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DateStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DateStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TimestampStatistics {
    // message fields
    minimum: ::std::option::Option<i64>,
    maximum: ::std::option::Option<i64>,
    minimumUtc: ::std::option::Option<i64>,
    maximumUtc: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TimestampStatistics {}

impl TimestampStatistics {
    pub fn new() -> TimestampStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TimestampStatistics {
        static mut instance: ::protobuf::lazy::Lazy<TimestampStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TimestampStatistics,
        };
        unsafe {
            instance.get(TimestampStatistics::new)
        }
    }

    // optional sint64 minimum = 1;

    pub fn clear_minimum(&mut self) {
        self.minimum = ::std::option::Option::None;
    }

    pub fn has_minimum(&self) -> bool {
        self.minimum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum(&mut self, v: i64) {
        self.minimum = ::std::option::Option::Some(v);
    }

    pub fn get_minimum(&self) -> i64 {
        self.minimum.unwrap_or(0)
    }

    fn get_minimum_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.minimum
    }

    fn mut_minimum_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.minimum
    }

    // optional sint64 maximum = 2;

    pub fn clear_maximum(&mut self) {
        self.maximum = ::std::option::Option::None;
    }

    pub fn has_maximum(&self) -> bool {
        self.maximum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum(&mut self, v: i64) {
        self.maximum = ::std::option::Option::Some(v);
    }

    pub fn get_maximum(&self) -> i64 {
        self.maximum.unwrap_or(0)
    }

    fn get_maximum_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.maximum
    }

    fn mut_maximum_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.maximum
    }

    // optional sint64 minimumUtc = 3;

    pub fn clear_minimumUtc(&mut self) {
        self.minimumUtc = ::std::option::Option::None;
    }

    pub fn has_minimumUtc(&self) -> bool {
        self.minimumUtc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimumUtc(&mut self, v: i64) {
        self.minimumUtc = ::std::option::Option::Some(v);
    }

    pub fn get_minimumUtc(&self) -> i64 {
        self.minimumUtc.unwrap_or(0)
    }

    fn get_minimumUtc_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.minimumUtc
    }

    fn mut_minimumUtc_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.minimumUtc
    }

    // optional sint64 maximumUtc = 4;

    pub fn clear_maximumUtc(&mut self) {
        self.maximumUtc = ::std::option::Option::None;
    }

    pub fn has_maximumUtc(&self) -> bool {
        self.maximumUtc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximumUtc(&mut self, v: i64) {
        self.maximumUtc = ::std::option::Option::Some(v);
    }

    pub fn get_maximumUtc(&self) -> i64 {
        self.maximumUtc.unwrap_or(0)
    }

    fn get_maximumUtc_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.maximumUtc
    }

    fn mut_maximumUtc_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.maximumUtc
    }
}

impl ::protobuf::Message for TimestampStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.minimum = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.maximum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.minimumUtc = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.maximumUtc = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.minimum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(v) = self.maximum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        }
        if let Some(v) = self.minimumUtc {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        if let Some(v) = self.maximumUtc {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.minimum {
            os.write_sint64(1, v)?;
        }
        if let Some(v) = self.maximum {
            os.write_sint64(2, v)?;
        }
        if let Some(v) = self.minimumUtc {
            os.write_sint64(3, v)?;
        }
        if let Some(v) = self.maximumUtc {
            os.write_sint64(4, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TimestampStatistics {
    fn new() -> TimestampStatistics {
        TimestampStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<TimestampStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "minimum",
                    TimestampStatistics::get_minimum_for_reflect,
                    TimestampStatistics::mut_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "maximum",
                    TimestampStatistics::get_maximum_for_reflect,
                    TimestampStatistics::mut_maximum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "minimumUtc",
                    TimestampStatistics::get_minimumUtc_for_reflect,
                    TimestampStatistics::mut_minimumUtc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "maximumUtc",
                    TimestampStatistics::get_maximumUtc_for_reflect,
                    TimestampStatistics::mut_maximumUtc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TimestampStatistics>(
                    "TimestampStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TimestampStatistics {
    fn clear(&mut self) {
        self.clear_minimum();
        self.clear_maximum();
        self.clear_minimumUtc();
        self.clear_maximumUtc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TimestampStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TimestampStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BinaryStatistics {
    // message fields
    sum: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BinaryStatistics {}

impl BinaryStatistics {
    pub fn new() -> BinaryStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BinaryStatistics {
        static mut instance: ::protobuf::lazy::Lazy<BinaryStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BinaryStatistics,
        };
        unsafe {
            instance.get(BinaryStatistics::new)
        }
    }

    // optional sint64 sum = 1;

    pub fn clear_sum(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_sum(&self) -> bool {
        self.sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: i64) {
        self.sum = ::std::option::Option::Some(v);
    }

    pub fn get_sum(&self) -> i64 {
        self.sum.unwrap_or(0)
    }

    fn get_sum_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.sum
    }
}

impl ::protobuf::Message for BinaryStatistics {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.sum = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.sum {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sum {
            os.write_sint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BinaryStatistics {
    fn new() -> BinaryStatistics {
        BinaryStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<BinaryStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "sum",
                    BinaryStatistics::get_sum_for_reflect,
                    BinaryStatistics::mut_sum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BinaryStatistics>(
                    "BinaryStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BinaryStatistics {
    fn clear(&mut self) {
        self.clear_sum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BinaryStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BinaryStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ColumnStatistics {
    // message fields
    numberOfValues: ::std::option::Option<u64>,
    intStatistics: ::protobuf::SingularPtrField<IntegerStatistics>,
    doubleStatistics: ::protobuf::SingularPtrField<DoubleStatistics>,
    stringStatistics: ::protobuf::SingularPtrField<StringStatistics>,
    bucketStatistics: ::protobuf::SingularPtrField<BucketStatistics>,
    decimalStatistics: ::protobuf::SingularPtrField<DecimalStatistics>,
    dateStatistics: ::protobuf::SingularPtrField<DateStatistics>,
    binaryStatistics: ::protobuf::SingularPtrField<BinaryStatistics>,
    timestampStatistics: ::protobuf::SingularPtrField<TimestampStatistics>,
    hasNull: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnStatistics {}

impl ColumnStatistics {
    pub fn new() -> ColumnStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnStatistics {
        static mut instance: ::protobuf::lazy::Lazy<ColumnStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnStatistics,
        };
        unsafe {
            instance.get(ColumnStatistics::new)
        }
    }

    // optional uint64 numberOfValues = 1;

    pub fn clear_numberOfValues(&mut self) {
        self.numberOfValues = ::std::option::Option::None;
    }

    pub fn has_numberOfValues(&self) -> bool {
        self.numberOfValues.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberOfValues(&mut self, v: u64) {
        self.numberOfValues = ::std::option::Option::Some(v);
    }

    pub fn get_numberOfValues(&self) -> u64 {
        self.numberOfValues.unwrap_or(0)
    }

    fn get_numberOfValues_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numberOfValues
    }

    fn mut_numberOfValues_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numberOfValues
    }

    // optional .orc.proto.IntegerStatistics intStatistics = 2;

    pub fn clear_intStatistics(&mut self) {
        self.intStatistics.clear();
    }

    pub fn has_intStatistics(&self) -> bool {
        self.intStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_intStatistics(&mut self, v: IntegerStatistics) {
        self.intStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_intStatistics(&mut self) -> &mut IntegerStatistics {
        if self.intStatistics.is_none() {
            self.intStatistics.set_default();
        }
        self.intStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_intStatistics(&mut self) -> IntegerStatistics {
        self.intStatistics.take().unwrap_or_else(|| IntegerStatistics::new())
    }

    pub fn get_intStatistics(&self) -> &IntegerStatistics {
        self.intStatistics.as_ref().unwrap_or_else(|| IntegerStatistics::default_instance())
    }

    fn get_intStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<IntegerStatistics> {
        &self.intStatistics
    }

    fn mut_intStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<IntegerStatistics> {
        &mut self.intStatistics
    }

    // optional .orc.proto.DoubleStatistics doubleStatistics = 3;

    pub fn clear_doubleStatistics(&mut self) {
        self.doubleStatistics.clear();
    }

    pub fn has_doubleStatistics(&self) -> bool {
        self.doubleStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_doubleStatistics(&mut self, v: DoubleStatistics) {
        self.doubleStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doubleStatistics(&mut self) -> &mut DoubleStatistics {
        if self.doubleStatistics.is_none() {
            self.doubleStatistics.set_default();
        }
        self.doubleStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_doubleStatistics(&mut self) -> DoubleStatistics {
        self.doubleStatistics.take().unwrap_or_else(|| DoubleStatistics::new())
    }

    pub fn get_doubleStatistics(&self) -> &DoubleStatistics {
        self.doubleStatistics.as_ref().unwrap_or_else(|| DoubleStatistics::default_instance())
    }

    fn get_doubleStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<DoubleStatistics> {
        &self.doubleStatistics
    }

    fn mut_doubleStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DoubleStatistics> {
        &mut self.doubleStatistics
    }

    // optional .orc.proto.StringStatistics stringStatistics = 4;

    pub fn clear_stringStatistics(&mut self) {
        self.stringStatistics.clear();
    }

    pub fn has_stringStatistics(&self) -> bool {
        self.stringStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stringStatistics(&mut self, v: StringStatistics) {
        self.stringStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stringStatistics(&mut self) -> &mut StringStatistics {
        if self.stringStatistics.is_none() {
            self.stringStatistics.set_default();
        }
        self.stringStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_stringStatistics(&mut self) -> StringStatistics {
        self.stringStatistics.take().unwrap_or_else(|| StringStatistics::new())
    }

    pub fn get_stringStatistics(&self) -> &StringStatistics {
        self.stringStatistics.as_ref().unwrap_or_else(|| StringStatistics::default_instance())
    }

    fn get_stringStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<StringStatistics> {
        &self.stringStatistics
    }

    fn mut_stringStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StringStatistics> {
        &mut self.stringStatistics
    }

    // optional .orc.proto.BucketStatistics bucketStatistics = 5;

    pub fn clear_bucketStatistics(&mut self) {
        self.bucketStatistics.clear();
    }

    pub fn has_bucketStatistics(&self) -> bool {
        self.bucketStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucketStatistics(&mut self, v: BucketStatistics) {
        self.bucketStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucketStatistics(&mut self) -> &mut BucketStatistics {
        if self.bucketStatistics.is_none() {
            self.bucketStatistics.set_default();
        }
        self.bucketStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucketStatistics(&mut self) -> BucketStatistics {
        self.bucketStatistics.take().unwrap_or_else(|| BucketStatistics::new())
    }

    pub fn get_bucketStatistics(&self) -> &BucketStatistics {
        self.bucketStatistics.as_ref().unwrap_or_else(|| BucketStatistics::default_instance())
    }

    fn get_bucketStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<BucketStatistics> {
        &self.bucketStatistics
    }

    fn mut_bucketStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BucketStatistics> {
        &mut self.bucketStatistics
    }

    // optional .orc.proto.DecimalStatistics decimalStatistics = 6;

    pub fn clear_decimalStatistics(&mut self) {
        self.decimalStatistics.clear();
    }

    pub fn has_decimalStatistics(&self) -> bool {
        self.decimalStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decimalStatistics(&mut self, v: DecimalStatistics) {
        self.decimalStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_decimalStatistics(&mut self) -> &mut DecimalStatistics {
        if self.decimalStatistics.is_none() {
            self.decimalStatistics.set_default();
        }
        self.decimalStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_decimalStatistics(&mut self) -> DecimalStatistics {
        self.decimalStatistics.take().unwrap_or_else(|| DecimalStatistics::new())
    }

    pub fn get_decimalStatistics(&self) -> &DecimalStatistics {
        self.decimalStatistics.as_ref().unwrap_or_else(|| DecimalStatistics::default_instance())
    }

    fn get_decimalStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<DecimalStatistics> {
        &self.decimalStatistics
    }

    fn mut_decimalStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DecimalStatistics> {
        &mut self.decimalStatistics
    }

    // optional .orc.proto.DateStatistics dateStatistics = 7;

    pub fn clear_dateStatistics(&mut self) {
        self.dateStatistics.clear();
    }

    pub fn has_dateStatistics(&self) -> bool {
        self.dateStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dateStatistics(&mut self, v: DateStatistics) {
        self.dateStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dateStatistics(&mut self) -> &mut DateStatistics {
        if self.dateStatistics.is_none() {
            self.dateStatistics.set_default();
        }
        self.dateStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_dateStatistics(&mut self) -> DateStatistics {
        self.dateStatistics.take().unwrap_or_else(|| DateStatistics::new())
    }

    pub fn get_dateStatistics(&self) -> &DateStatistics {
        self.dateStatistics.as_ref().unwrap_or_else(|| DateStatistics::default_instance())
    }

    fn get_dateStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<DateStatistics> {
        &self.dateStatistics
    }

    fn mut_dateStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DateStatistics> {
        &mut self.dateStatistics
    }

    // optional .orc.proto.BinaryStatistics binaryStatistics = 8;

    pub fn clear_binaryStatistics(&mut self) {
        self.binaryStatistics.clear();
    }

    pub fn has_binaryStatistics(&self) -> bool {
        self.binaryStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_binaryStatistics(&mut self, v: BinaryStatistics) {
        self.binaryStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_binaryStatistics(&mut self) -> &mut BinaryStatistics {
        if self.binaryStatistics.is_none() {
            self.binaryStatistics.set_default();
        }
        self.binaryStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_binaryStatistics(&mut self) -> BinaryStatistics {
        self.binaryStatistics.take().unwrap_or_else(|| BinaryStatistics::new())
    }

    pub fn get_binaryStatistics(&self) -> &BinaryStatistics {
        self.binaryStatistics.as_ref().unwrap_or_else(|| BinaryStatistics::default_instance())
    }

    fn get_binaryStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<BinaryStatistics> {
        &self.binaryStatistics
    }

    fn mut_binaryStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BinaryStatistics> {
        &mut self.binaryStatistics
    }

    // optional .orc.proto.TimestampStatistics timestampStatistics = 9;

    pub fn clear_timestampStatistics(&mut self) {
        self.timestampStatistics.clear();
    }

    pub fn has_timestampStatistics(&self) -> bool {
        self.timestampStatistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestampStatistics(&mut self, v: TimestampStatistics) {
        self.timestampStatistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestampStatistics(&mut self) -> &mut TimestampStatistics {
        if self.timestampStatistics.is_none() {
            self.timestampStatistics.set_default();
        }
        self.timestampStatistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_timestampStatistics(&mut self) -> TimestampStatistics {
        self.timestampStatistics.take().unwrap_or_else(|| TimestampStatistics::new())
    }

    pub fn get_timestampStatistics(&self) -> &TimestampStatistics {
        self.timestampStatistics.as_ref().unwrap_or_else(|| TimestampStatistics::default_instance())
    }

    fn get_timestampStatistics_for_reflect(&self) -> &::protobuf::SingularPtrField<TimestampStatistics> {
        &self.timestampStatistics
    }

    fn mut_timestampStatistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TimestampStatistics> {
        &mut self.timestampStatistics
    }

    // optional bool hasNull = 10;

    pub fn clear_hasNull(&mut self) {
        self.hasNull = ::std::option::Option::None;
    }

    pub fn has_hasNull(&self) -> bool {
        self.hasNull.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hasNull(&mut self, v: bool) {
        self.hasNull = ::std::option::Option::Some(v);
    }

    pub fn get_hasNull(&self) -> bool {
        self.hasNull.unwrap_or(false)
    }

    fn get_hasNull_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hasNull
    }

    fn mut_hasNull_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hasNull
    }
}

impl ::protobuf::Message for ColumnStatistics {
    fn is_initialized(&self) -> bool {
        for v in &self.intStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.doubleStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.stringStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.bucketStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.decimalStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.dateStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.binaryStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.timestampStatistics {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numberOfValues = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.intStatistics)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.doubleStatistics)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stringStatistics)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.bucketStatistics)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.decimalStatistics)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dateStatistics)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.binaryStatistics)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timestampStatistics)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hasNull = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.numberOfValues {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.intStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.doubleStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.stringStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.bucketStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.decimalStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.dateStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.binaryStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.timestampStatistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.hasNull {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.numberOfValues {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.intStatistics.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.doubleStatistics.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.stringStatistics.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.bucketStatistics.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.decimalStatistics.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.dateStatistics.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.binaryStatistics.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.timestampStatistics.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.hasNull {
            os.write_bool(10, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ColumnStatistics {
    fn new() -> ColumnStatistics {
        ColumnStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numberOfValues",
                    ColumnStatistics::get_numberOfValues_for_reflect,
                    ColumnStatistics::mut_numberOfValues_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IntegerStatistics>>(
                    "intStatistics",
                    ColumnStatistics::get_intStatistics_for_reflect,
                    ColumnStatistics::mut_intStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DoubleStatistics>>(
                    "doubleStatistics",
                    ColumnStatistics::get_doubleStatistics_for_reflect,
                    ColumnStatistics::mut_doubleStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StringStatistics>>(
                    "stringStatistics",
                    ColumnStatistics::get_stringStatistics_for_reflect,
                    ColumnStatistics::mut_stringStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BucketStatistics>>(
                    "bucketStatistics",
                    ColumnStatistics::get_bucketStatistics_for_reflect,
                    ColumnStatistics::mut_bucketStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DecimalStatistics>>(
                    "decimalStatistics",
                    ColumnStatistics::get_decimalStatistics_for_reflect,
                    ColumnStatistics::mut_decimalStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DateStatistics>>(
                    "dateStatistics",
                    ColumnStatistics::get_dateStatistics_for_reflect,
                    ColumnStatistics::mut_dateStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BinaryStatistics>>(
                    "binaryStatistics",
                    ColumnStatistics::get_binaryStatistics_for_reflect,
                    ColumnStatistics::mut_binaryStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TimestampStatistics>>(
                    "timestampStatistics",
                    ColumnStatistics::get_timestampStatistics_for_reflect,
                    ColumnStatistics::mut_timestampStatistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hasNull",
                    ColumnStatistics::get_hasNull_for_reflect,
                    ColumnStatistics::mut_hasNull_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnStatistics>(
                    "ColumnStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnStatistics {
    fn clear(&mut self) {
        self.clear_numberOfValues();
        self.clear_intStatistics();
        self.clear_doubleStatistics();
        self.clear_stringStatistics();
        self.clear_bucketStatistics();
        self.clear_decimalStatistics();
        self.clear_dateStatistics();
        self.clear_binaryStatistics();
        self.clear_timestampStatistics();
        self.clear_hasNull();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColumnStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RowIndexEntry {
    // message fields
    positions: ::std::vec::Vec<u64>,
    statistics: ::protobuf::SingularPtrField<ColumnStatistics>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RowIndexEntry {}

impl RowIndexEntry {
    pub fn new() -> RowIndexEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RowIndexEntry {
        static mut instance: ::protobuf::lazy::Lazy<RowIndexEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RowIndexEntry,
        };
        unsafe {
            instance.get(RowIndexEntry::new)
        }
    }

    // repeated uint64 positions = 1;

    pub fn clear_positions(&mut self) {
        self.positions.clear();
    }

    // Param is passed by value, moved
    pub fn set_positions(&mut self, v: ::std::vec::Vec<u64>) {
        self.positions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_positions(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.positions
    }

    // Take field
    pub fn take_positions(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.positions, ::std::vec::Vec::new())
    }

    pub fn get_positions(&self) -> &[u64] {
        &self.positions
    }

    fn get_positions_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.positions
    }

    fn mut_positions_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.positions
    }

    // optional .orc.proto.ColumnStatistics statistics = 2;

    pub fn clear_statistics(&mut self) {
        self.statistics.clear();
    }

    pub fn has_statistics(&self) -> bool {
        self.statistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_statistics(&mut self, v: ColumnStatistics) {
        self.statistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_statistics(&mut self) -> &mut ColumnStatistics {
        if self.statistics.is_none() {
            self.statistics.set_default();
        }
        self.statistics.as_mut().unwrap()
    }

    // Take field
    pub fn take_statistics(&mut self) -> ColumnStatistics {
        self.statistics.take().unwrap_or_else(|| ColumnStatistics::new())
    }

    pub fn get_statistics(&self) -> &ColumnStatistics {
        self.statistics.as_ref().unwrap_or_else(|| ColumnStatistics::default_instance())
    }

    fn get_statistics_for_reflect(&self) -> &::protobuf::SingularPtrField<ColumnStatistics> {
        &self.statistics
    }

    fn mut_statistics_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ColumnStatistics> {
        &mut self.statistics
    }
}

impl ::protobuf::Message for RowIndexEntry {
    fn is_initialized(&self) -> bool {
        for v in &self.statistics {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.positions)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.statistics)?;
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
        if !self.positions.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.positions);
        }
        if let Some(ref v) = self.statistics.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.positions.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.positions))?;
            for v in &self.positions {
                os.write_uint64_no_tag(*v)?;
            };
        }
        if let Some(ref v) = self.statistics.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RowIndexEntry {
    fn new() -> RowIndexEntry {
        RowIndexEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<RowIndexEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "positions",
                    RowIndexEntry::get_positions_for_reflect,
                    RowIndexEntry::mut_positions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ColumnStatistics>>(
                    "statistics",
                    RowIndexEntry::get_statistics_for_reflect,
                    RowIndexEntry::mut_statistics_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RowIndexEntry>(
                    "RowIndexEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RowIndexEntry {
    fn clear(&mut self) {
        self.clear_positions();
        self.clear_statistics();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RowIndexEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RowIndexEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RowIndex {
    // message fields
    entry: ::protobuf::RepeatedField<RowIndexEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RowIndex {}

impl RowIndex {
    pub fn new() -> RowIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RowIndex {
        static mut instance: ::protobuf::lazy::Lazy<RowIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RowIndex,
        };
        unsafe {
            instance.get(RowIndex::new)
        }
    }

    // repeated .orc.proto.RowIndexEntry entry = 1;

    pub fn clear_entry(&mut self) {
        self.entry.clear();
    }

    // Param is passed by value, moved
    pub fn set_entry(&mut self, v: ::protobuf::RepeatedField<RowIndexEntry>) {
        self.entry = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entry(&mut self) -> &mut ::protobuf::RepeatedField<RowIndexEntry> {
        &mut self.entry
    }

    // Take field
    pub fn take_entry(&mut self) -> ::protobuf::RepeatedField<RowIndexEntry> {
        ::std::mem::replace(&mut self.entry, ::protobuf::RepeatedField::new())
    }

    pub fn get_entry(&self) -> &[RowIndexEntry] {
        &self.entry
    }

    fn get_entry_for_reflect(&self) -> &::protobuf::RepeatedField<RowIndexEntry> {
        &self.entry
    }

    fn mut_entry_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RowIndexEntry> {
        &mut self.entry
    }
}

impl ::protobuf::Message for RowIndex {
    fn is_initialized(&self) -> bool {
        for v in &self.entry {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entry)?;
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
        for value in &self.entry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entry {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RowIndex {
    fn new() -> RowIndex {
        RowIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<RowIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RowIndexEntry>>(
                    "entry",
                    RowIndex::get_entry_for_reflect,
                    RowIndex::mut_entry_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RowIndex>(
                    "RowIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RowIndex {
    fn clear(&mut self) {
        self.clear_entry();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RowIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RowIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BloomFilter {
    // message fields
    numHashFunctions: ::std::option::Option<u32>,
    bitset: ::std::vec::Vec<u64>,
    utf8bitset: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BloomFilter {}

impl BloomFilter {
    pub fn new() -> BloomFilter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BloomFilter {
        static mut instance: ::protobuf::lazy::Lazy<BloomFilter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BloomFilter,
        };
        unsafe {
            instance.get(BloomFilter::new)
        }
    }

    // optional uint32 numHashFunctions = 1;

    pub fn clear_numHashFunctions(&mut self) {
        self.numHashFunctions = ::std::option::Option::None;
    }

    pub fn has_numHashFunctions(&self) -> bool {
        self.numHashFunctions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numHashFunctions(&mut self, v: u32) {
        self.numHashFunctions = ::std::option::Option::Some(v);
    }

    pub fn get_numHashFunctions(&self) -> u32 {
        self.numHashFunctions.unwrap_or(0)
    }

    fn get_numHashFunctions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numHashFunctions
    }

    fn mut_numHashFunctions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numHashFunctions
    }

    // repeated fixed64 bitset = 2;

    pub fn clear_bitset(&mut self) {
        self.bitset.clear();
    }

    // Param is passed by value, moved
    pub fn set_bitset(&mut self, v: ::std::vec::Vec<u64>) {
        self.bitset = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bitset(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.bitset
    }

    // Take field
    pub fn take_bitset(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.bitset, ::std::vec::Vec::new())
    }

    pub fn get_bitset(&self) -> &[u64] {
        &self.bitset
    }

    fn get_bitset_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.bitset
    }

    fn mut_bitset_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.bitset
    }

    // optional bytes utf8bitset = 3;

    pub fn clear_utf8bitset(&mut self) {
        self.utf8bitset.clear();
    }

    pub fn has_utf8bitset(&self) -> bool {
        self.utf8bitset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_utf8bitset(&mut self, v: ::std::vec::Vec<u8>) {
        self.utf8bitset = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_utf8bitset(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.utf8bitset.is_none() {
            self.utf8bitset.set_default();
        }
        self.utf8bitset.as_mut().unwrap()
    }

    // Take field
    pub fn take_utf8bitset(&mut self) -> ::std::vec::Vec<u8> {
        self.utf8bitset.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_utf8bitset(&self) -> &[u8] {
        match self.utf8bitset.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_utf8bitset_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.utf8bitset
    }

    fn mut_utf8bitset_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.utf8bitset
    }
}

impl ::protobuf::Message for BloomFilter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numHashFunctions = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.bitset)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.utf8bitset)?;
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
        if let Some(v) = self.numHashFunctions {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += 9 * self.bitset.len() as u32;
        if let Some(ref v) = self.utf8bitset.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.numHashFunctions {
            os.write_uint32(1, v)?;
        }
        for v in &self.bitset {
            os.write_fixed64(2, *v)?;
        };
        if let Some(ref v) = self.utf8bitset.as_ref() {
            os.write_bytes(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BloomFilter {
    fn new() -> BloomFilter {
        BloomFilter::new()
    }

    fn descriptor_static(_: ::std::option::Option<BloomFilter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numHashFunctions",
                    BloomFilter::get_numHashFunctions_for_reflect,
                    BloomFilter::mut_numHashFunctions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "bitset",
                    BloomFilter::get_bitset_for_reflect,
                    BloomFilter::mut_bitset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "utf8bitset",
                    BloomFilter::get_utf8bitset_for_reflect,
                    BloomFilter::mut_utf8bitset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BloomFilter>(
                    "BloomFilter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BloomFilter {
    fn clear(&mut self) {
        self.clear_numHashFunctions();
        self.clear_bitset();
        self.clear_utf8bitset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BloomFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BloomFilter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BloomFilterIndex {
    // message fields
    bloomFilter: ::protobuf::RepeatedField<BloomFilter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BloomFilterIndex {}

impl BloomFilterIndex {
    pub fn new() -> BloomFilterIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BloomFilterIndex {
        static mut instance: ::protobuf::lazy::Lazy<BloomFilterIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BloomFilterIndex,
        };
        unsafe {
            instance.get(BloomFilterIndex::new)
        }
    }

    // repeated .orc.proto.BloomFilter bloomFilter = 1;

    pub fn clear_bloomFilter(&mut self) {
        self.bloomFilter.clear();
    }

    // Param is passed by value, moved
    pub fn set_bloomFilter(&mut self, v: ::protobuf::RepeatedField<BloomFilter>) {
        self.bloomFilter = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bloomFilter(&mut self) -> &mut ::protobuf::RepeatedField<BloomFilter> {
        &mut self.bloomFilter
    }

    // Take field
    pub fn take_bloomFilter(&mut self) -> ::protobuf::RepeatedField<BloomFilter> {
        ::std::mem::replace(&mut self.bloomFilter, ::protobuf::RepeatedField::new())
    }

    pub fn get_bloomFilter(&self) -> &[BloomFilter] {
        &self.bloomFilter
    }

    fn get_bloomFilter_for_reflect(&self) -> &::protobuf::RepeatedField<BloomFilter> {
        &self.bloomFilter
    }

    fn mut_bloomFilter_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BloomFilter> {
        &mut self.bloomFilter
    }
}

impl ::protobuf::Message for BloomFilterIndex {
    fn is_initialized(&self) -> bool {
        for v in &self.bloomFilter {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.bloomFilter)?;
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
        for value in &self.bloomFilter {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.bloomFilter {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BloomFilterIndex {
    fn new() -> BloomFilterIndex {
        BloomFilterIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<BloomFilterIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BloomFilter>>(
                    "bloomFilter",
                    BloomFilterIndex::get_bloomFilter_for_reflect,
                    BloomFilterIndex::mut_bloomFilter_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BloomFilterIndex>(
                    "BloomFilterIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BloomFilterIndex {
    fn clear(&mut self) {
        self.clear_bloomFilter();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BloomFilterIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BloomFilterIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Stream {
    // message fields
    kind: ::std::option::Option<Stream_Kind>,
    column: ::std::option::Option<u32>,
    length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Stream {}

impl Stream {
    pub fn new() -> Stream {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Stream {
        static mut instance: ::protobuf::lazy::Lazy<Stream> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Stream,
        };
        unsafe {
            instance.get(Stream::new)
        }
    }

    // optional .orc.proto.Stream.Kind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: Stream_Kind) {
        self.kind = ::std::option::Option::Some(v);
    }

    pub fn get_kind(&self) -> Stream_Kind {
        self.kind.unwrap_or(Stream_Kind::PRESENT)
    }

    fn get_kind_for_reflect(&self) -> &::std::option::Option<Stream_Kind> {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut ::std::option::Option<Stream_Kind> {
        &mut self.kind
    }

    // optional uint32 column = 2;

    pub fn clear_column(&mut self) {
        self.column = ::std::option::Option::None;
    }

    pub fn has_column(&self) -> bool {
        self.column.is_some()
    }

    // Param is passed by value, moved
    pub fn set_column(&mut self, v: u32) {
        self.column = ::std::option::Option::Some(v);
    }

    pub fn get_column(&self) -> u32 {
        self.column.unwrap_or(0)
    }

    fn get_column_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.column
    }

    fn mut_column_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.column
    }

    // optional uint64 length = 3;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }
}

impl ::protobuf::Message for Stream {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.kind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.column = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.kind {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.column {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.kind {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.column {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Stream {
    fn new() -> Stream {
        Stream::new()
    }

    fn descriptor_static(_: ::std::option::Option<Stream>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Stream_Kind>>(
                    "kind",
                    Stream::get_kind_for_reflect,
                    Stream::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "column",
                    Stream::get_column_for_reflect,
                    Stream::mut_column_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    Stream::get_length_for_reflect,
                    Stream::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Stream>(
                    "Stream",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Stream {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_column();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Stream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Stream {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Stream_Kind {
    PRESENT = 0,
    DATA = 1,
    LENGTH = 2,
    DICTIONARY_DATA = 3,
    DICTIONARY_COUNT = 4,
    SECONDARY = 5,
    ROW_INDEX = 6,
    BLOOM_FILTER = 7,
    BLOOM_FILTER_UTF8 = 8,
}

impl ::protobuf::ProtobufEnum for Stream_Kind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Stream_Kind> {
        match value {
            0 => ::std::option::Option::Some(Stream_Kind::PRESENT),
            1 => ::std::option::Option::Some(Stream_Kind::DATA),
            2 => ::std::option::Option::Some(Stream_Kind::LENGTH),
            3 => ::std::option::Option::Some(Stream_Kind::DICTIONARY_DATA),
            4 => ::std::option::Option::Some(Stream_Kind::DICTIONARY_COUNT),
            5 => ::std::option::Option::Some(Stream_Kind::SECONDARY),
            6 => ::std::option::Option::Some(Stream_Kind::ROW_INDEX),
            7 => ::std::option::Option::Some(Stream_Kind::BLOOM_FILTER),
            8 => ::std::option::Option::Some(Stream_Kind::BLOOM_FILTER_UTF8),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Stream_Kind] = &[
            Stream_Kind::PRESENT,
            Stream_Kind::DATA,
            Stream_Kind::LENGTH,
            Stream_Kind::DICTIONARY_DATA,
            Stream_Kind::DICTIONARY_COUNT,
            Stream_Kind::SECONDARY,
            Stream_Kind::ROW_INDEX,
            Stream_Kind::BLOOM_FILTER,
            Stream_Kind::BLOOM_FILTER_UTF8,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Stream_Kind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Stream_Kind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Stream_Kind {
}

impl ::protobuf::reflect::ProtobufValue for Stream_Kind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ColumnEncoding {
    // message fields
    kind: ::std::option::Option<ColumnEncoding_Kind>,
    dictionarySize: ::std::option::Option<u32>,
    bloomEncoding: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ColumnEncoding {}

impl ColumnEncoding {
    pub fn new() -> ColumnEncoding {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ColumnEncoding {
        static mut instance: ::protobuf::lazy::Lazy<ColumnEncoding> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ColumnEncoding,
        };
        unsafe {
            instance.get(ColumnEncoding::new)
        }
    }

    // optional .orc.proto.ColumnEncoding.Kind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: ColumnEncoding_Kind) {
        self.kind = ::std::option::Option::Some(v);
    }

    pub fn get_kind(&self) -> ColumnEncoding_Kind {
        self.kind.unwrap_or(ColumnEncoding_Kind::DIRECT)
    }

    fn get_kind_for_reflect(&self) -> &::std::option::Option<ColumnEncoding_Kind> {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut ::std::option::Option<ColumnEncoding_Kind> {
        &mut self.kind
    }

    // optional uint32 dictionarySize = 2;

    pub fn clear_dictionarySize(&mut self) {
        self.dictionarySize = ::std::option::Option::None;
    }

    pub fn has_dictionarySize(&self) -> bool {
        self.dictionarySize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dictionarySize(&mut self, v: u32) {
        self.dictionarySize = ::std::option::Option::Some(v);
    }

    pub fn get_dictionarySize(&self) -> u32 {
        self.dictionarySize.unwrap_or(0)
    }

    fn get_dictionarySize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dictionarySize
    }

    fn mut_dictionarySize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dictionarySize
    }

    // optional uint32 bloomEncoding = 3;

    pub fn clear_bloomEncoding(&mut self) {
        self.bloomEncoding = ::std::option::Option::None;
    }

    pub fn has_bloomEncoding(&self) -> bool {
        self.bloomEncoding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bloomEncoding(&mut self, v: u32) {
        self.bloomEncoding = ::std::option::Option::Some(v);
    }

    pub fn get_bloomEncoding(&self) -> u32 {
        self.bloomEncoding.unwrap_or(0)
    }

    fn get_bloomEncoding_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bloomEncoding
    }

    fn mut_bloomEncoding_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bloomEncoding
    }
}

impl ::protobuf::Message for ColumnEncoding {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.kind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.dictionarySize = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bloomEncoding = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.kind {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.dictionarySize {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bloomEncoding {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.kind {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.dictionarySize {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.bloomEncoding {
            os.write_uint32(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ColumnEncoding {
    fn new() -> ColumnEncoding {
        ColumnEncoding::new()
    }

    fn descriptor_static(_: ::std::option::Option<ColumnEncoding>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ColumnEncoding_Kind>>(
                    "kind",
                    ColumnEncoding::get_kind_for_reflect,
                    ColumnEncoding::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dictionarySize",
                    ColumnEncoding::get_dictionarySize_for_reflect,
                    ColumnEncoding::mut_dictionarySize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bloomEncoding",
                    ColumnEncoding::get_bloomEncoding_for_reflect,
                    ColumnEncoding::mut_bloomEncoding_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ColumnEncoding>(
                    "ColumnEncoding",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ColumnEncoding {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_dictionarySize();
        self.clear_bloomEncoding();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColumnEncoding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColumnEncoding {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ColumnEncoding_Kind {
    DIRECT = 0,
    DICTIONARY = 1,
    DIRECT_V2 = 2,
    DICTIONARY_V2 = 3,
}

impl ::protobuf::ProtobufEnum for ColumnEncoding_Kind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ColumnEncoding_Kind> {
        match value {
            0 => ::std::option::Option::Some(ColumnEncoding_Kind::DIRECT),
            1 => ::std::option::Option::Some(ColumnEncoding_Kind::DICTIONARY),
            2 => ::std::option::Option::Some(ColumnEncoding_Kind::DIRECT_V2),
            3 => ::std::option::Option::Some(ColumnEncoding_Kind::DICTIONARY_V2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ColumnEncoding_Kind] = &[
            ColumnEncoding_Kind::DIRECT,
            ColumnEncoding_Kind::DICTIONARY,
            ColumnEncoding_Kind::DIRECT_V2,
            ColumnEncoding_Kind::DICTIONARY_V2,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ColumnEncoding_Kind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ColumnEncoding_Kind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ColumnEncoding_Kind {
}

impl ::protobuf::reflect::ProtobufValue for ColumnEncoding_Kind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StripeFooter {
    // message fields
    streams: ::protobuf::RepeatedField<Stream>,
    columns: ::protobuf::RepeatedField<ColumnEncoding>,
    writerTimezone: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StripeFooter {}

impl StripeFooter {
    pub fn new() -> StripeFooter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StripeFooter {
        static mut instance: ::protobuf::lazy::Lazy<StripeFooter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StripeFooter,
        };
        unsafe {
            instance.get(StripeFooter::new)
        }
    }

    // repeated .orc.proto.Stream streams = 1;

    pub fn clear_streams(&mut self) {
        self.streams.clear();
    }

    // Param is passed by value, moved
    pub fn set_streams(&mut self, v: ::protobuf::RepeatedField<Stream>) {
        self.streams = v;
    }

    // Mutable pointer to the field.
    pub fn mut_streams(&mut self) -> &mut ::protobuf::RepeatedField<Stream> {
        &mut self.streams
    }

    // Take field
    pub fn take_streams(&mut self) -> ::protobuf::RepeatedField<Stream> {
        ::std::mem::replace(&mut self.streams, ::protobuf::RepeatedField::new())
    }

    pub fn get_streams(&self) -> &[Stream] {
        &self.streams
    }

    fn get_streams_for_reflect(&self) -> &::protobuf::RepeatedField<Stream> {
        &self.streams
    }

    fn mut_streams_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Stream> {
        &mut self.streams
    }

    // repeated .orc.proto.ColumnEncoding columns = 2;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<ColumnEncoding>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<ColumnEncoding> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<ColumnEncoding> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[ColumnEncoding] {
        &self.columns
    }

    fn get_columns_for_reflect(&self) -> &::protobuf::RepeatedField<ColumnEncoding> {
        &self.columns
    }

    fn mut_columns_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ColumnEncoding> {
        &mut self.columns
    }

    // optional string writerTimezone = 3;

    pub fn clear_writerTimezone(&mut self) {
        self.writerTimezone.clear();
    }

    pub fn has_writerTimezone(&self) -> bool {
        self.writerTimezone.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writerTimezone(&mut self, v: ::std::string::String) {
        self.writerTimezone = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_writerTimezone(&mut self) -> &mut ::std::string::String {
        if self.writerTimezone.is_none() {
            self.writerTimezone.set_default();
        }
        self.writerTimezone.as_mut().unwrap()
    }

    // Take field
    pub fn take_writerTimezone(&mut self) -> ::std::string::String {
        self.writerTimezone.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_writerTimezone(&self) -> &str {
        match self.writerTimezone.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_writerTimezone_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.writerTimezone
    }

    fn mut_writerTimezone_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.writerTimezone
    }
}

impl ::protobuf::Message for StripeFooter {
    fn is_initialized(&self) -> bool {
        for v in &self.streams {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.columns {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.streams)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.writerTimezone)?;
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
        for value in &self.streams {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.writerTimezone.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.streams {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.columns {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.writerTimezone.as_ref() {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StripeFooter {
    fn new() -> StripeFooter {
        StripeFooter::new()
    }

    fn descriptor_static(_: ::std::option::Option<StripeFooter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Stream>>(
                    "streams",
                    StripeFooter::get_streams_for_reflect,
                    StripeFooter::mut_streams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ColumnEncoding>>(
                    "columns",
                    StripeFooter::get_columns_for_reflect,
                    StripeFooter::mut_columns_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "writerTimezone",
                    StripeFooter::get_writerTimezone_for_reflect,
                    StripeFooter::mut_writerTimezone_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StripeFooter>(
                    "StripeFooter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StripeFooter {
    fn clear(&mut self) {
        self.clear_streams();
        self.clear_columns();
        self.clear_writerTimezone();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StripeFooter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StripeFooter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Type {
    // message fields
    kind: ::std::option::Option<Type_Kind>,
    subtypes: ::std::vec::Vec<u32>,
    fieldNames: ::protobuf::RepeatedField<::std::string::String>,
    maximumLength: ::std::option::Option<u32>,
    precision: ::std::option::Option<u32>,
    scale: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Type {}

impl Type {
    pub fn new() -> Type {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Type {
        static mut instance: ::protobuf::lazy::Lazy<Type> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Type,
        };
        unsafe {
            instance.get(Type::new)
        }
    }

    // optional .orc.proto.Type.Kind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: Type_Kind) {
        self.kind = ::std::option::Option::Some(v);
    }

    pub fn get_kind(&self) -> Type_Kind {
        self.kind.unwrap_or(Type_Kind::BOOLEAN)
    }

    fn get_kind_for_reflect(&self) -> &::std::option::Option<Type_Kind> {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut ::std::option::Option<Type_Kind> {
        &mut self.kind
    }

    // repeated uint32 subtypes = 2;

    pub fn clear_subtypes(&mut self) {
        self.subtypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_subtypes(&mut self, v: ::std::vec::Vec<u32>) {
        self.subtypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subtypes(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.subtypes
    }

    // Take field
    pub fn take_subtypes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.subtypes, ::std::vec::Vec::new())
    }

    pub fn get_subtypes(&self) -> &[u32] {
        &self.subtypes
    }

    fn get_subtypes_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.subtypes
    }

    fn mut_subtypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.subtypes
    }

    // repeated string fieldNames = 3;

    pub fn clear_fieldNames(&mut self) {
        self.fieldNames.clear();
    }

    // Param is passed by value, moved
    pub fn set_fieldNames(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.fieldNames = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fieldNames(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.fieldNames
    }

    // Take field
    pub fn take_fieldNames(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.fieldNames, ::protobuf::RepeatedField::new())
    }

    pub fn get_fieldNames(&self) -> &[::std::string::String] {
        &self.fieldNames
    }

    fn get_fieldNames_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.fieldNames
    }

    fn mut_fieldNames_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.fieldNames
    }

    // optional uint32 maximumLength = 4;

    pub fn clear_maximumLength(&mut self) {
        self.maximumLength = ::std::option::Option::None;
    }

    pub fn has_maximumLength(&self) -> bool {
        self.maximumLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximumLength(&mut self, v: u32) {
        self.maximumLength = ::std::option::Option::Some(v);
    }

    pub fn get_maximumLength(&self) -> u32 {
        self.maximumLength.unwrap_or(0)
    }

    fn get_maximumLength_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.maximumLength
    }

    fn mut_maximumLength_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.maximumLength
    }

    // optional uint32 precision = 5;

    pub fn clear_precision(&mut self) {
        self.precision = ::std::option::Option::None;
    }

    pub fn has_precision(&self) -> bool {
        self.precision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_precision(&mut self, v: u32) {
        self.precision = ::std::option::Option::Some(v);
    }

    pub fn get_precision(&self) -> u32 {
        self.precision.unwrap_or(0)
    }

    fn get_precision_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.precision
    }

    fn mut_precision_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.precision
    }

    // optional uint32 scale = 6;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: u32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> u32 {
        self.scale.unwrap_or(0)
    }

    fn get_scale_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.scale
    }

    fn mut_scale_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.scale
    }
}

impl ::protobuf::Message for Type {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.kind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.subtypes)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.fieldNames)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.maximumLength = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.precision = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.scale = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.kind {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if !self.subtypes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.subtypes);
        }
        for value in &self.fieldNames {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if let Some(v) = self.maximumLength {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.precision {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.scale {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.kind {
            os.write_enum(1, v.value())?;
        }
        if !self.subtypes.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.subtypes))?;
            for v in &self.subtypes {
                os.write_uint32_no_tag(*v)?;
            };
        }
        for v in &self.fieldNames {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.maximumLength {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.precision {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.scale {
            os.write_uint32(6, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Type {
    fn new() -> Type {
        Type::new()
    }

    fn descriptor_static(_: ::std::option::Option<Type>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Type_Kind>>(
                    "kind",
                    Type::get_kind_for_reflect,
                    Type::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "subtypes",
                    Type::get_subtypes_for_reflect,
                    Type::mut_subtypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fieldNames",
                    Type::get_fieldNames_for_reflect,
                    Type::mut_fieldNames_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "maximumLength",
                    Type::get_maximumLength_for_reflect,
                    Type::mut_maximumLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "precision",
                    Type::get_precision_for_reflect,
                    Type::mut_precision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "scale",
                    Type::get_scale_for_reflect,
                    Type::mut_scale_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Type>(
                    "Type",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Type {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_subtypes();
        self.clear_fieldNames();
        self.clear_maximumLength();
        self.clear_precision();
        self.clear_scale();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Type_Kind {
    BOOLEAN = 0,
    BYTE = 1,
    SHORT = 2,
    INT = 3,
    LONG = 4,
    FLOAT = 5,
    DOUBLE = 6,
    STRING = 7,
    BINARY = 8,
    TIMESTAMP = 9,
    LIST = 10,
    MAP = 11,
    STRUCT = 12,
    UNION = 13,
    DECIMAL = 14,
    DATE = 15,
    VARCHAR = 16,
    CHAR = 17,
}

impl ::protobuf::ProtobufEnum for Type_Kind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Type_Kind> {
        match value {
            0 => ::std::option::Option::Some(Type_Kind::BOOLEAN),
            1 => ::std::option::Option::Some(Type_Kind::BYTE),
            2 => ::std::option::Option::Some(Type_Kind::SHORT),
            3 => ::std::option::Option::Some(Type_Kind::INT),
            4 => ::std::option::Option::Some(Type_Kind::LONG),
            5 => ::std::option::Option::Some(Type_Kind::FLOAT),
            6 => ::std::option::Option::Some(Type_Kind::DOUBLE),
            7 => ::std::option::Option::Some(Type_Kind::STRING),
            8 => ::std::option::Option::Some(Type_Kind::BINARY),
            9 => ::std::option::Option::Some(Type_Kind::TIMESTAMP),
            10 => ::std::option::Option::Some(Type_Kind::LIST),
            11 => ::std::option::Option::Some(Type_Kind::MAP),
            12 => ::std::option::Option::Some(Type_Kind::STRUCT),
            13 => ::std::option::Option::Some(Type_Kind::UNION),
            14 => ::std::option::Option::Some(Type_Kind::DECIMAL),
            15 => ::std::option::Option::Some(Type_Kind::DATE),
            16 => ::std::option::Option::Some(Type_Kind::VARCHAR),
            17 => ::std::option::Option::Some(Type_Kind::CHAR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Type_Kind] = &[
            Type_Kind::BOOLEAN,
            Type_Kind::BYTE,
            Type_Kind::SHORT,
            Type_Kind::INT,
            Type_Kind::LONG,
            Type_Kind::FLOAT,
            Type_Kind::DOUBLE,
            Type_Kind::STRING,
            Type_Kind::BINARY,
            Type_Kind::TIMESTAMP,
            Type_Kind::LIST,
            Type_Kind::MAP,
            Type_Kind::STRUCT,
            Type_Kind::UNION,
            Type_Kind::DECIMAL,
            Type_Kind::DATE,
            Type_Kind::VARCHAR,
            Type_Kind::CHAR,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Type_Kind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Type_Kind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Type_Kind {
}

impl ::protobuf::reflect::ProtobufValue for Type_Kind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StripeInformation {
    // message fields
    offset: ::std::option::Option<u64>,
    indexLength: ::std::option::Option<u64>,
    dataLength: ::std::option::Option<u64>,
    footerLength: ::std::option::Option<u64>,
    numberOfRows: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StripeInformation {}

impl StripeInformation {
    pub fn new() -> StripeInformation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StripeInformation {
        static mut instance: ::protobuf::lazy::Lazy<StripeInformation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StripeInformation,
        };
        unsafe {
            instance.get(StripeInformation::new)
        }
    }

    // optional uint64 offset = 1;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }

    fn get_offset_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.offset
    }

    fn mut_offset_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.offset
    }

    // optional uint64 indexLength = 2;

    pub fn clear_indexLength(&mut self) {
        self.indexLength = ::std::option::Option::None;
    }

    pub fn has_indexLength(&self) -> bool {
        self.indexLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_indexLength(&mut self, v: u64) {
        self.indexLength = ::std::option::Option::Some(v);
    }

    pub fn get_indexLength(&self) -> u64 {
        self.indexLength.unwrap_or(0)
    }

    fn get_indexLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.indexLength
    }

    fn mut_indexLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.indexLength
    }

    // optional uint64 dataLength = 3;

    pub fn clear_dataLength(&mut self) {
        self.dataLength = ::std::option::Option::None;
    }

    pub fn has_dataLength(&self) -> bool {
        self.dataLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dataLength(&mut self, v: u64) {
        self.dataLength = ::std::option::Option::Some(v);
    }

    pub fn get_dataLength(&self) -> u64 {
        self.dataLength.unwrap_or(0)
    }

    fn get_dataLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.dataLength
    }

    fn mut_dataLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.dataLength
    }

    // optional uint64 footerLength = 4;

    pub fn clear_footerLength(&mut self) {
        self.footerLength = ::std::option::Option::None;
    }

    pub fn has_footerLength(&self) -> bool {
        self.footerLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_footerLength(&mut self, v: u64) {
        self.footerLength = ::std::option::Option::Some(v);
    }

    pub fn get_footerLength(&self) -> u64 {
        self.footerLength.unwrap_or(0)
    }

    fn get_footerLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.footerLength
    }

    fn mut_footerLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.footerLength
    }

    // optional uint64 numberOfRows = 5;

    pub fn clear_numberOfRows(&mut self) {
        self.numberOfRows = ::std::option::Option::None;
    }

    pub fn has_numberOfRows(&self) -> bool {
        self.numberOfRows.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberOfRows(&mut self, v: u64) {
        self.numberOfRows = ::std::option::Option::Some(v);
    }

    pub fn get_numberOfRows(&self) -> u64 {
        self.numberOfRows.unwrap_or(0)
    }

    fn get_numberOfRows_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numberOfRows
    }

    fn mut_numberOfRows_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numberOfRows
    }
}

impl ::protobuf::Message for StripeInformation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.indexLength = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.dataLength = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.footerLength = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numberOfRows = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.indexLength {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dataLength {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.footerLength {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numberOfRows {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offset {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.indexLength {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.dataLength {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.footerLength {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.numberOfRows {
            os.write_uint64(5, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StripeInformation {
    fn new() -> StripeInformation {
        StripeInformation::new()
    }

    fn descriptor_static(_: ::std::option::Option<StripeInformation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "offset",
                    StripeInformation::get_offset_for_reflect,
                    StripeInformation::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "indexLength",
                    StripeInformation::get_indexLength_for_reflect,
                    StripeInformation::mut_indexLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "dataLength",
                    StripeInformation::get_dataLength_for_reflect,
                    StripeInformation::mut_dataLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "footerLength",
                    StripeInformation::get_footerLength_for_reflect,
                    StripeInformation::mut_footerLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numberOfRows",
                    StripeInformation::get_numberOfRows_for_reflect,
                    StripeInformation::mut_numberOfRows_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StripeInformation>(
                    "StripeInformation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StripeInformation {
    fn clear(&mut self) {
        self.clear_offset();
        self.clear_indexLength();
        self.clear_dataLength();
        self.clear_footerLength();
        self.clear_numberOfRows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StripeInformation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StripeInformation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserMetadataItem {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserMetadataItem {}

impl UserMetadataItem {
    pub fn new() -> UserMetadataItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserMetadataItem {
        static mut instance: ::protobuf::lazy::Lazy<UserMetadataItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserMetadataItem,
        };
        unsafe {
            instance.get(UserMetadataItem::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for UserMetadataItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserMetadataItem {
    fn new() -> UserMetadataItem {
        UserMetadataItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserMetadataItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    UserMetadataItem::get_name_for_reflect,
                    UserMetadataItem::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    UserMetadataItem::get_value_for_reflect,
                    UserMetadataItem::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserMetadataItem>(
                    "UserMetadataItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserMetadataItem {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserMetadataItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserMetadataItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StripeStatistics {
    // message fields
    colStats: ::protobuf::RepeatedField<ColumnStatistics>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StripeStatistics {}

impl StripeStatistics {
    pub fn new() -> StripeStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StripeStatistics {
        static mut instance: ::protobuf::lazy::Lazy<StripeStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StripeStatistics,
        };
        unsafe {
            instance.get(StripeStatistics::new)
        }
    }

    // repeated .orc.proto.ColumnStatistics colStats = 1;

    pub fn clear_colStats(&mut self) {
        self.colStats.clear();
    }

    // Param is passed by value, moved
    pub fn set_colStats(&mut self, v: ::protobuf::RepeatedField<ColumnStatistics>) {
        self.colStats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_colStats(&mut self) -> &mut ::protobuf::RepeatedField<ColumnStatistics> {
        &mut self.colStats
    }

    // Take field
    pub fn take_colStats(&mut self) -> ::protobuf::RepeatedField<ColumnStatistics> {
        ::std::mem::replace(&mut self.colStats, ::protobuf::RepeatedField::new())
    }

    pub fn get_colStats(&self) -> &[ColumnStatistics] {
        &self.colStats
    }

    fn get_colStats_for_reflect(&self) -> &::protobuf::RepeatedField<ColumnStatistics> {
        &self.colStats
    }

    fn mut_colStats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ColumnStatistics> {
        &mut self.colStats
    }
}

impl ::protobuf::Message for StripeStatistics {
    fn is_initialized(&self) -> bool {
        for v in &self.colStats {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.colStats)?;
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
        for value in &self.colStats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.colStats {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StripeStatistics {
    fn new() -> StripeStatistics {
        StripeStatistics::new()
    }

    fn descriptor_static(_: ::std::option::Option<StripeStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ColumnStatistics>>(
                    "colStats",
                    StripeStatistics::get_colStats_for_reflect,
                    StripeStatistics::mut_colStats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StripeStatistics>(
                    "StripeStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StripeStatistics {
    fn clear(&mut self) {
        self.clear_colStats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StripeStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StripeStatistics {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    stripeStats: ::protobuf::RepeatedField<StripeStatistics>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe {
            instance.get(Metadata::new)
        }
    }

    // repeated .orc.proto.StripeStatistics stripeStats = 1;

    pub fn clear_stripeStats(&mut self) {
        self.stripeStats.clear();
    }

    // Param is passed by value, moved
    pub fn set_stripeStats(&mut self, v: ::protobuf::RepeatedField<StripeStatistics>) {
        self.stripeStats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stripeStats(&mut self) -> &mut ::protobuf::RepeatedField<StripeStatistics> {
        &mut self.stripeStats
    }

    // Take field
    pub fn take_stripeStats(&mut self) -> ::protobuf::RepeatedField<StripeStatistics> {
        ::std::mem::replace(&mut self.stripeStats, ::protobuf::RepeatedField::new())
    }

    pub fn get_stripeStats(&self) -> &[StripeStatistics] {
        &self.stripeStats
    }

    fn get_stripeStats_for_reflect(&self) -> &::protobuf::RepeatedField<StripeStatistics> {
        &self.stripeStats
    }

    fn mut_stripeStats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StripeStatistics> {
        &mut self.stripeStats
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        for v in &self.stripeStats {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stripeStats)?;
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
        for value in &self.stripeStats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.stripeStats {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Metadata {
    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StripeStatistics>>(
                    "stripeStats",
                    Metadata::get_stripeStats_for_reflect,
                    Metadata::mut_stripeStats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.clear_stripeStats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Footer {
    // message fields
    headerLength: ::std::option::Option<u64>,
    contentLength: ::std::option::Option<u64>,
    stripes: ::protobuf::RepeatedField<StripeInformation>,
    types: ::protobuf::RepeatedField<Type>,
    metadata: ::protobuf::RepeatedField<UserMetadataItem>,
    numberOfRows: ::std::option::Option<u64>,
    statistics: ::protobuf::RepeatedField<ColumnStatistics>,
    rowIndexStride: ::std::option::Option<u32>,
    writer: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Footer {}

impl Footer {
    pub fn new() -> Footer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Footer {
        static mut instance: ::protobuf::lazy::Lazy<Footer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Footer,
        };
        unsafe {
            instance.get(Footer::new)
        }
    }

    // optional uint64 headerLength = 1;

    pub fn clear_headerLength(&mut self) {
        self.headerLength = ::std::option::Option::None;
    }

    pub fn has_headerLength(&self) -> bool {
        self.headerLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_headerLength(&mut self, v: u64) {
        self.headerLength = ::std::option::Option::Some(v);
    }

    pub fn get_headerLength(&self) -> u64 {
        self.headerLength.unwrap_or(0)
    }

    fn get_headerLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.headerLength
    }

    fn mut_headerLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.headerLength
    }

    // optional uint64 contentLength = 2;

    pub fn clear_contentLength(&mut self) {
        self.contentLength = ::std::option::Option::None;
    }

    pub fn has_contentLength(&self) -> bool {
        self.contentLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contentLength(&mut self, v: u64) {
        self.contentLength = ::std::option::Option::Some(v);
    }

    pub fn get_contentLength(&self) -> u64 {
        self.contentLength.unwrap_or(0)
    }

    fn get_contentLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.contentLength
    }

    fn mut_contentLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.contentLength
    }

    // repeated .orc.proto.StripeInformation stripes = 3;

    pub fn clear_stripes(&mut self) {
        self.stripes.clear();
    }

    // Param is passed by value, moved
    pub fn set_stripes(&mut self, v: ::protobuf::RepeatedField<StripeInformation>) {
        self.stripes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stripes(&mut self) -> &mut ::protobuf::RepeatedField<StripeInformation> {
        &mut self.stripes
    }

    // Take field
    pub fn take_stripes(&mut self) -> ::protobuf::RepeatedField<StripeInformation> {
        ::std::mem::replace(&mut self.stripes, ::protobuf::RepeatedField::new())
    }

    pub fn get_stripes(&self) -> &[StripeInformation] {
        &self.stripes
    }

    fn get_stripes_for_reflect(&self) -> &::protobuf::RepeatedField<StripeInformation> {
        &self.stripes
    }

    fn mut_stripes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StripeInformation> {
        &mut self.stripes
    }

    // repeated .orc.proto.Type types = 4;

    pub fn clear_types(&mut self) {
        self.types.clear();
    }

    // Param is passed by value, moved
    pub fn set_types(&mut self, v: ::protobuf::RepeatedField<Type>) {
        self.types = v;
    }

    // Mutable pointer to the field.
    pub fn mut_types(&mut self) -> &mut ::protobuf::RepeatedField<Type> {
        &mut self.types
    }

    // Take field
    pub fn take_types(&mut self) -> ::protobuf::RepeatedField<Type> {
        ::std::mem::replace(&mut self.types, ::protobuf::RepeatedField::new())
    }

    pub fn get_types(&self) -> &[Type] {
        &self.types
    }

    fn get_types_for_reflect(&self) -> &::protobuf::RepeatedField<Type> {
        &self.types
    }

    fn mut_types_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Type> {
        &mut self.types
    }

    // repeated .orc.proto.UserMetadataItem metadata = 5;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::protobuf::RepeatedField<UserMetadataItem>) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut ::protobuf::RepeatedField<UserMetadataItem> {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::protobuf::RepeatedField<UserMetadataItem> {
        ::std::mem::replace(&mut self.metadata, ::protobuf::RepeatedField::new())
    }

    pub fn get_metadata(&self) -> &[UserMetadataItem] {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::RepeatedField<UserMetadataItem> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UserMetadataItem> {
        &mut self.metadata
    }

    // optional uint64 numberOfRows = 6;

    pub fn clear_numberOfRows(&mut self) {
        self.numberOfRows = ::std::option::Option::None;
    }

    pub fn has_numberOfRows(&self) -> bool {
        self.numberOfRows.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberOfRows(&mut self, v: u64) {
        self.numberOfRows = ::std::option::Option::Some(v);
    }

    pub fn get_numberOfRows(&self) -> u64 {
        self.numberOfRows.unwrap_or(0)
    }

    fn get_numberOfRows_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numberOfRows
    }

    fn mut_numberOfRows_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numberOfRows
    }

    // repeated .orc.proto.ColumnStatistics statistics = 7;

    pub fn clear_statistics(&mut self) {
        self.statistics.clear();
    }

    // Param is passed by value, moved
    pub fn set_statistics(&mut self, v: ::protobuf::RepeatedField<ColumnStatistics>) {
        self.statistics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_statistics(&mut self) -> &mut ::protobuf::RepeatedField<ColumnStatistics> {
        &mut self.statistics
    }

    // Take field
    pub fn take_statistics(&mut self) -> ::protobuf::RepeatedField<ColumnStatistics> {
        ::std::mem::replace(&mut self.statistics, ::protobuf::RepeatedField::new())
    }

    pub fn get_statistics(&self) -> &[ColumnStatistics] {
        &self.statistics
    }

    fn get_statistics_for_reflect(&self) -> &::protobuf::RepeatedField<ColumnStatistics> {
        &self.statistics
    }

    fn mut_statistics_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ColumnStatistics> {
        &mut self.statistics
    }

    // optional uint32 rowIndexStride = 8;

    pub fn clear_rowIndexStride(&mut self) {
        self.rowIndexStride = ::std::option::Option::None;
    }

    pub fn has_rowIndexStride(&self) -> bool {
        self.rowIndexStride.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rowIndexStride(&mut self, v: u32) {
        self.rowIndexStride = ::std::option::Option::Some(v);
    }

    pub fn get_rowIndexStride(&self) -> u32 {
        self.rowIndexStride.unwrap_or(0)
    }

    fn get_rowIndexStride_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rowIndexStride
    }

    fn mut_rowIndexStride_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rowIndexStride
    }

    // optional uint32 writer = 9;

    pub fn clear_writer(&mut self) {
        self.writer = ::std::option::Option::None;
    }

    pub fn has_writer(&self) -> bool {
        self.writer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writer(&mut self, v: u32) {
        self.writer = ::std::option::Option::Some(v);
    }

    pub fn get_writer(&self) -> u32 {
        self.writer.unwrap_or(0)
    }

    fn get_writer_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.writer
    }

    fn mut_writer_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.writer
    }
}

impl ::protobuf::Message for Footer {
    fn is_initialized(&self) -> bool {
        for v in &self.stripes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.types {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.statistics {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.headerLength = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.contentLength = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stripes)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.types)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metadata)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numberOfRows = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.statistics)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.rowIndexStride = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.writer = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.headerLength {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.contentLength {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.stripes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.types {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.metadata {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.numberOfRows {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.statistics {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.rowIndexStride {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.writer {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.headerLength {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.contentLength {
            os.write_uint64(2, v)?;
        }
        for v in &self.stripes {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.types {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.metadata {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.numberOfRows {
            os.write_uint64(6, v)?;
        }
        for v in &self.statistics {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.rowIndexStride {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.writer {
            os.write_uint32(9, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Footer {
    fn new() -> Footer {
        Footer::new()
    }

    fn descriptor_static(_: ::std::option::Option<Footer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "headerLength",
                    Footer::get_headerLength_for_reflect,
                    Footer::mut_headerLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "contentLength",
                    Footer::get_contentLength_for_reflect,
                    Footer::mut_contentLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StripeInformation>>(
                    "stripes",
                    Footer::get_stripes_for_reflect,
                    Footer::mut_stripes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Type>>(
                    "types",
                    Footer::get_types_for_reflect,
                    Footer::mut_types_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UserMetadataItem>>(
                    "metadata",
                    Footer::get_metadata_for_reflect,
                    Footer::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numberOfRows",
                    Footer::get_numberOfRows_for_reflect,
                    Footer::mut_numberOfRows_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ColumnStatistics>>(
                    "statistics",
                    Footer::get_statistics_for_reflect,
                    Footer::mut_statistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rowIndexStride",
                    Footer::get_rowIndexStride_for_reflect,
                    Footer::mut_rowIndexStride_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "writer",
                    Footer::get_writer_for_reflect,
                    Footer::mut_writer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Footer>(
                    "Footer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Footer {
    fn clear(&mut self) {
        self.clear_headerLength();
        self.clear_contentLength();
        self.clear_stripes();
        self.clear_types();
        self.clear_metadata();
        self.clear_numberOfRows();
        self.clear_statistics();
        self.clear_rowIndexStride();
        self.clear_writer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Footer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Footer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PostScript {
    // message fields
    footerLength: ::std::option::Option<u64>,
    compression: ::std::option::Option<CompressionKind>,
    compressionBlockSize: ::std::option::Option<u64>,
    version: ::std::vec::Vec<u32>,
    metadataLength: ::std::option::Option<u64>,
    writerVersion: ::std::option::Option<u32>,
    magic: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PostScript {}

impl PostScript {
    pub fn new() -> PostScript {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PostScript {
        static mut instance: ::protobuf::lazy::Lazy<PostScript> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PostScript,
        };
        unsafe {
            instance.get(PostScript::new)
        }
    }

    // optional uint64 footerLength = 1;

    pub fn clear_footerLength(&mut self) {
        self.footerLength = ::std::option::Option::None;
    }

    pub fn has_footerLength(&self) -> bool {
        self.footerLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_footerLength(&mut self, v: u64) {
        self.footerLength = ::std::option::Option::Some(v);
    }

    pub fn get_footerLength(&self) -> u64 {
        self.footerLength.unwrap_or(0)
    }

    fn get_footerLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.footerLength
    }

    fn mut_footerLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.footerLength
    }

    // optional .orc.proto.CompressionKind compression = 2;

    pub fn clear_compression(&mut self) {
        self.compression = ::std::option::Option::None;
    }

    pub fn has_compression(&self) -> bool {
        self.compression.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compression(&mut self, v: CompressionKind) {
        self.compression = ::std::option::Option::Some(v);
    }

    pub fn get_compression(&self) -> CompressionKind {
        self.compression.unwrap_or(CompressionKind::NONE)
    }

    fn get_compression_for_reflect(&self) -> &::std::option::Option<CompressionKind> {
        &self.compression
    }

    fn mut_compression_for_reflect(&mut self) -> &mut ::std::option::Option<CompressionKind> {
        &mut self.compression
    }

    // optional uint64 compressionBlockSize = 3;

    pub fn clear_compressionBlockSize(&mut self) {
        self.compressionBlockSize = ::std::option::Option::None;
    }

    pub fn has_compressionBlockSize(&self) -> bool {
        self.compressionBlockSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compressionBlockSize(&mut self, v: u64) {
        self.compressionBlockSize = ::std::option::Option::Some(v);
    }

    pub fn get_compressionBlockSize(&self) -> u64 {
        self.compressionBlockSize.unwrap_or(0)
    }

    fn get_compressionBlockSize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.compressionBlockSize
    }

    fn mut_compressionBlockSize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.compressionBlockSize
    }

    // repeated uint32 version = 4;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::vec::Vec<u32>) {
        self.version = v;
    }

    // Mutable pointer to the field.
    pub fn mut_version(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.version, ::std::vec::Vec::new())
    }

    pub fn get_version(&self) -> &[u32] {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.version
    }

    // optional uint64 metadataLength = 5;

    pub fn clear_metadataLength(&mut self) {
        self.metadataLength = ::std::option::Option::None;
    }

    pub fn has_metadataLength(&self) -> bool {
        self.metadataLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadataLength(&mut self, v: u64) {
        self.metadataLength = ::std::option::Option::Some(v);
    }

    pub fn get_metadataLength(&self) -> u64 {
        self.metadataLength.unwrap_or(0)
    }

    fn get_metadataLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.metadataLength
    }

    fn mut_metadataLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.metadataLength
    }

    // optional uint32 writerVersion = 6;

    pub fn clear_writerVersion(&mut self) {
        self.writerVersion = ::std::option::Option::None;
    }

    pub fn has_writerVersion(&self) -> bool {
        self.writerVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writerVersion(&mut self, v: u32) {
        self.writerVersion = ::std::option::Option::Some(v);
    }

    pub fn get_writerVersion(&self) -> u32 {
        self.writerVersion.unwrap_or(0)
    }

    fn get_writerVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.writerVersion
    }

    fn mut_writerVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.writerVersion
    }

    // optional string magic = 8000;

    pub fn clear_magic(&mut self) {
        self.magic.clear();
    }

    pub fn has_magic(&self) -> bool {
        self.magic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magic(&mut self, v: ::std::string::String) {
        self.magic = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_magic(&mut self) -> &mut ::std::string::String {
        if self.magic.is_none() {
            self.magic.set_default();
        }
        self.magic.as_mut().unwrap()
    }

    // Take field
    pub fn take_magic(&mut self) -> ::std::string::String {
        self.magic.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_magic(&self) -> &str {
        match self.magic.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_magic_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.magic
    }

    fn mut_magic_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.magic
    }
}

impl ::protobuf::Message for PostScript {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.footerLength = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.compression = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.compressionBlockSize = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.version)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.metadataLength = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.writerVersion = ::std::option::Option::Some(tmp);
                },
                8000 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.magic)?;
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
        if let Some(v) = self.footerLength {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.compression {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.compressionBlockSize {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, &self.version);
        }
        if let Some(v) = self.metadataLength {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.writerVersion {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.magic.as_ref() {
            my_size += ::protobuf::rt::string_size(8000, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.footerLength {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.compression {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.compressionBlockSize {
            os.write_uint64(3, v)?;
        }
        if !self.version.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.version))?;
            for v in &self.version {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if let Some(v) = self.metadataLength {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.writerVersion {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.magic.as_ref() {
            os.write_string(8000, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PostScript {
    fn new() -> PostScript {
        PostScript::new()
    }

    fn descriptor_static(_: ::std::option::Option<PostScript>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "footerLength",
                    PostScript::get_footerLength_for_reflect,
                    PostScript::mut_footerLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CompressionKind>>(
                    "compression",
                    PostScript::get_compression_for_reflect,
                    PostScript::mut_compression_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "compressionBlockSize",
                    PostScript::get_compressionBlockSize_for_reflect,
                    PostScript::mut_compressionBlockSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    PostScript::get_version_for_reflect,
                    PostScript::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "metadataLength",
                    PostScript::get_metadataLength_for_reflect,
                    PostScript::mut_metadataLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "writerVersion",
                    PostScript::get_writerVersion_for_reflect,
                    PostScript::mut_writerVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "magic",
                    PostScript::get_magic_for_reflect,
                    PostScript::mut_magic_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PostScript>(
                    "PostScript",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PostScript {
    fn clear(&mut self) {
        self.clear_footerLength();
        self.clear_compression();
        self.clear_compressionBlockSize();
        self.clear_version();
        self.clear_metadataLength();
        self.clear_writerVersion();
        self.clear_magic();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PostScript {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PostScript {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileTail {
    // message fields
    postscript: ::protobuf::SingularPtrField<PostScript>,
    footer: ::protobuf::SingularPtrField<Footer>,
    fileLength: ::std::option::Option<u64>,
    postscriptLength: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileTail {}

impl FileTail {
    pub fn new() -> FileTail {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileTail {
        static mut instance: ::protobuf::lazy::Lazy<FileTail> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileTail,
        };
        unsafe {
            instance.get(FileTail::new)
        }
    }

    // optional .orc.proto.PostScript postscript = 1;

    pub fn clear_postscript(&mut self) {
        self.postscript.clear();
    }

    pub fn has_postscript(&self) -> bool {
        self.postscript.is_some()
    }

    // Param is passed by value, moved
    pub fn set_postscript(&mut self, v: PostScript) {
        self.postscript = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_postscript(&mut self) -> &mut PostScript {
        if self.postscript.is_none() {
            self.postscript.set_default();
        }
        self.postscript.as_mut().unwrap()
    }

    // Take field
    pub fn take_postscript(&mut self) -> PostScript {
        self.postscript.take().unwrap_or_else(|| PostScript::new())
    }

    pub fn get_postscript(&self) -> &PostScript {
        self.postscript.as_ref().unwrap_or_else(|| PostScript::default_instance())
    }

    fn get_postscript_for_reflect(&self) -> &::protobuf::SingularPtrField<PostScript> {
        &self.postscript
    }

    fn mut_postscript_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PostScript> {
        &mut self.postscript
    }

    // optional .orc.proto.Footer footer = 2;

    pub fn clear_footer(&mut self) {
        self.footer.clear();
    }

    pub fn has_footer(&self) -> bool {
        self.footer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_footer(&mut self, v: Footer) {
        self.footer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_footer(&mut self) -> &mut Footer {
        if self.footer.is_none() {
            self.footer.set_default();
        }
        self.footer.as_mut().unwrap()
    }

    // Take field
    pub fn take_footer(&mut self) -> Footer {
        self.footer.take().unwrap_or_else(|| Footer::new())
    }

    pub fn get_footer(&self) -> &Footer {
        self.footer.as_ref().unwrap_or_else(|| Footer::default_instance())
    }

    fn get_footer_for_reflect(&self) -> &::protobuf::SingularPtrField<Footer> {
        &self.footer
    }

    fn mut_footer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Footer> {
        &mut self.footer
    }

    // optional uint64 fileLength = 3;

    pub fn clear_fileLength(&mut self) {
        self.fileLength = ::std::option::Option::None;
    }

    pub fn has_fileLength(&self) -> bool {
        self.fileLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileLength(&mut self, v: u64) {
        self.fileLength = ::std::option::Option::Some(v);
    }

    pub fn get_fileLength(&self) -> u64 {
        self.fileLength.unwrap_or(0)
    }

    fn get_fileLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileLength
    }

    fn mut_fileLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileLength
    }

    // optional uint64 postscriptLength = 4;

    pub fn clear_postscriptLength(&mut self) {
        self.postscriptLength = ::std::option::Option::None;
    }

    pub fn has_postscriptLength(&self) -> bool {
        self.postscriptLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_postscriptLength(&mut self, v: u64) {
        self.postscriptLength = ::std::option::Option::Some(v);
    }

    pub fn get_postscriptLength(&self) -> u64 {
        self.postscriptLength.unwrap_or(0)
    }

    fn get_postscriptLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.postscriptLength
    }

    fn mut_postscriptLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.postscriptLength
    }
}

impl ::protobuf::Message for FileTail {
    fn is_initialized(&self) -> bool {
        for v in &self.postscript {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.footer {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.postscript)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.footer)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.fileLength = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.postscriptLength = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.postscript.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.footer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.fileLength {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.postscriptLength {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.postscript.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.footer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.fileLength {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.postscriptLength {
            os.write_uint64(4, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileTail {
    fn new() -> FileTail {
        FileTail::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileTail>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PostScript>>(
                    "postscript",
                    FileTail::get_postscript_for_reflect,
                    FileTail::mut_postscript_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Footer>>(
                    "footer",
                    FileTail::get_footer_for_reflect,
                    FileTail::mut_footer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileLength",
                    FileTail::get_fileLength_for_reflect,
                    FileTail::mut_fileLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "postscriptLength",
                    FileTail::get_postscriptLength_for_reflect,
                    FileTail::mut_postscriptLength_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileTail>(
                    "FileTail",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileTail {
    fn clear(&mut self) {
        self.clear_postscript();
        self.clear_footer();
        self.clear_fileLength();
        self.clear_postscriptLength();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileTail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileTail {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CompressionKind {
    NONE = 0,
    ZLIB = 1,
    SNAPPY = 2,
    LZO = 3,
    LZ4 = 4,
    ZSTD = 5,
}

impl ::protobuf::ProtobufEnum for CompressionKind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CompressionKind> {
        match value {
            0 => ::std::option::Option::Some(CompressionKind::NONE),
            1 => ::std::option::Option::Some(CompressionKind::ZLIB),
            2 => ::std::option::Option::Some(CompressionKind::SNAPPY),
            3 => ::std::option::Option::Some(CompressionKind::LZO),
            4 => ::std::option::Option::Some(CompressionKind::LZ4),
            5 => ::std::option::Option::Some(CompressionKind::ZSTD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CompressionKind] = &[
            CompressionKind::NONE,
            CompressionKind::ZLIB,
            CompressionKind::SNAPPY,
            CompressionKind::LZO,
            CompressionKind::LZ4,
            CompressionKind::ZSTD,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CompressionKind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CompressionKind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CompressionKind {
}

impl ::protobuf::reflect::ProtobufValue for CompressionKind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0forc_proto.proto\x12\torc.proto\"Y\n\x11IntegerStatistics\x12\x18\n\
    \x07minimum\x18\x01\x20\x01(\x12R\x07minimum\x12\x18\n\x07maximum\x18\
    \x02\x20\x01(\x12R\x07maximum\x12\x10\n\x03sum\x18\x03\x20\x01(\x12R\x03\
    sum\"X\n\x10DoubleStatistics\x12\x18\n\x07minimum\x18\x01\x20\x01(\x01R\
    \x07minimum\x12\x18\n\x07maximum\x18\x02\x20\x01(\x01R\x07maximum\x12\
    \x10\n\x03sum\x18\x03\x20\x01(\x01R\x03sum\"X\n\x10StringStatistics\x12\
    \x18\n\x07minimum\x18\x01\x20\x01(\tR\x07minimum\x12\x18\n\x07maximum\
    \x18\x02\x20\x01(\tR\x07maximum\x12\x10\n\x03sum\x18\x03\x20\x01(\x12R\
    \x03sum\",\n\x10BucketStatistics\x12\x18\n\x05count\x18\x01\x20\x03(\x04\
    R\x05countB\x02\x10\x01\"Y\n\x11DecimalStatistics\x12\x18\n\x07minimum\
    \x18\x01\x20\x01(\tR\x07minimum\x12\x18\n\x07maximum\x18\x02\x20\x01(\tR\
    \x07maximum\x12\x10\n\x03sum\x18\x03\x20\x01(\tR\x03sum\"D\n\x0eDateStat\
    istics\x12\x18\n\x07minimum\x18\x01\x20\x01(\x11R\x07minimum\x12\x18\n\
    \x07maximum\x18\x02\x20\x01(\x11R\x07maximum\"\x89\x01\n\x13TimestampSta\
    tistics\x12\x18\n\x07minimum\x18\x01\x20\x01(\x12R\x07minimum\x12\x18\n\
    \x07maximum\x18\x02\x20\x01(\x12R\x07maximum\x12\x1e\n\nminimumUtc\x18\
    \x03\x20\x01(\x12R\nminimumUtc\x12\x1e\n\nmaximumUtc\x18\x04\x20\x01(\
    \x12R\nmaximumUtc\"$\n\x10BinaryStatistics\x12\x10\n\x03sum\x18\x01\x20\
    \x01(\x12R\x03sum\"\x9d\x05\n\x10ColumnStatistics\x12&\n\x0enumberOfValu\
    es\x18\x01\x20\x01(\x04R\x0enumberOfValues\x12B\n\rintStatistics\x18\x02\
    \x20\x01(\x0b2\x1c.orc.proto.IntegerStatisticsR\rintStatistics\x12G\n\
    \x10doubleStatistics\x18\x03\x20\x01(\x0b2\x1b.orc.proto.DoubleStatistic\
    sR\x10doubleStatistics\x12G\n\x10stringStatistics\x18\x04\x20\x01(\x0b2\
    \x1b.orc.proto.StringStatisticsR\x10stringStatistics\x12G\n\x10bucketSta\
    tistics\x18\x05\x20\x01(\x0b2\x1b.orc.proto.BucketStatisticsR\x10bucketS\
    tatistics\x12J\n\x11decimalStatistics\x18\x06\x20\x01(\x0b2\x1c.orc.prot\
    o.DecimalStatisticsR\x11decimalStatistics\x12A\n\x0edateStatistics\x18\
    \x07\x20\x01(\x0b2\x19.orc.proto.DateStatisticsR\x0edateStatistics\x12G\
    \n\x10binaryStatistics\x18\x08\x20\x01(\x0b2\x1b.orc.proto.BinaryStatist\
    icsR\x10binaryStatistics\x12P\n\x13timestampStatistics\x18\t\x20\x01(\
    \x0b2\x1e.orc.proto.TimestampStatisticsR\x13timestampStatistics\x12\x18\
    \n\x07hasNull\x18\n\x20\x01(\x08R\x07hasNull\"n\n\rRowIndexEntry\x12\x20\
    \n\tpositions\x18\x01\x20\x03(\x04R\tpositionsB\x02\x10\x01\x12;\n\nstat\
    istics\x18\x02\x20\x01(\x0b2\x1b.orc.proto.ColumnStatisticsR\nstatistics\
    \":\n\x08RowIndex\x12.\n\x05entry\x18\x01\x20\x03(\x0b2\x18.orc.proto.Ro\
    wIndexEntryR\x05entry\"q\n\x0bBloomFilter\x12*\n\x10numHashFunctions\x18\
    \x01\x20\x01(\rR\x10numHashFunctions\x12\x16\n\x06bitset\x18\x02\x20\x03\
    (\x06R\x06bitset\x12\x1e\n\nutf8bitset\x18\x03\x20\x01(\x0cR\nutf8bitset\
    \"L\n\x10BloomFilterIndex\x128\n\x0bbloomFilter\x18\x01\x20\x03(\x0b2\
    \x16.orc.proto.BloomFilterR\x0bbloomFilter\"\x82\x02\n\x06Stream\x12*\n\
    \x04kind\x18\x01\x20\x01(\x0e2\x16.orc.proto.Stream.KindR\x04kind\x12\
    \x16\n\x06column\x18\x02\x20\x01(\rR\x06column\x12\x16\n\x06length\x18\
    \x03\x20\x01(\x04R\x06length\"\x9b\x01\n\x04Kind\x12\x0b\n\x07PRESENT\
    \x10\0\x12\x08\n\x04DATA\x10\x01\x12\n\n\x06LENGTH\x10\x02\x12\x13\n\x0f\
    DICTIONARY_DATA\x10\x03\x12\x14\n\x10DICTIONARY_COUNT\x10\x04\x12\r\n\tS\
    ECONDARY\x10\x05\x12\r\n\tROW_INDEX\x10\x06\x12\x10\n\x0cBLOOM_FILTER\
    \x10\x07\x12\x15\n\x11BLOOM_FILTER_UTF8\x10\x08\"\xd8\x01\n\x0eColumnEnc\
    oding\x122\n\x04kind\x18\x01\x20\x01(\x0e2\x1e.orc.proto.ColumnEncoding.\
    KindR\x04kind\x12&\n\x0edictionarySize\x18\x02\x20\x01(\rR\x0edictionary\
    Size\x12$\n\rbloomEncoding\x18\x03\x20\x01(\rR\rbloomEncoding\"D\n\x04Ki\
    nd\x12\n\n\x06DIRECT\x10\0\x12\x0e\n\nDICTIONARY\x10\x01\x12\r\n\tDIRECT\
    _V2\x10\x02\x12\x11\n\rDICTIONARY_V2\x10\x03\"\x98\x01\n\x0cStripeFooter\
    \x12+\n\x07streams\x18\x01\x20\x03(\x0b2\x11.orc.proto.StreamR\x07stream\
    s\x123\n\x07columns\x18\x02\x20\x03(\x0b2\x19.orc.proto.ColumnEncodingR\
    \x07columns\x12&\n\x0ewriterTimezone\x18\x03\x20\x01(\tR\x0ewriterTimezo\
    ne\"\x9e\x03\n\x04Type\x12(\n\x04kind\x18\x01\x20\x01(\x0e2\x14.orc.prot\
    o.Type.KindR\x04kind\x12\x1e\n\x08subtypes\x18\x02\x20\x03(\rR\x08subtyp\
    esB\x02\x10\x01\x12\x1e\n\nfieldNames\x18\x03\x20\x03(\tR\nfieldNames\
    \x12$\n\rmaximumLength\x18\x04\x20\x01(\rR\rmaximumLength\x12\x1c\n\tpre\
    cision\x18\x05\x20\x01(\rR\tprecision\x12\x14\n\x05scale\x18\x06\x20\x01\
    (\rR\x05scale\"\xd1\x01\n\x04Kind\x12\x0b\n\x07BOOLEAN\x10\0\x12\x08\n\
    \x04BYTE\x10\x01\x12\t\n\x05SHORT\x10\x02\x12\x07\n\x03INT\x10\x03\x12\
    \x08\n\x04LONG\x10\x04\x12\t\n\x05FLOAT\x10\x05\x12\n\n\x06DOUBLE\x10\
    \x06\x12\n\n\x06STRING\x10\x07\x12\n\n\x06BINARY\x10\x08\x12\r\n\tTIMEST\
    AMP\x10\t\x12\x08\n\x04LIST\x10\n\x12\x07\n\x03MAP\x10\x0b\x12\n\n\x06ST\
    RUCT\x10\x0c\x12\t\n\x05UNION\x10\r\x12\x0b\n\x07DECIMAL\x10\x0e\x12\x08\
    \n\x04DATE\x10\x0f\x12\x0b\n\x07VARCHAR\x10\x10\x12\x08\n\x04CHAR\x10\
    \x11\"\xb5\x01\n\x11StripeInformation\x12\x16\n\x06offset\x18\x01\x20\
    \x01(\x04R\x06offset\x12\x20\n\x0bindexLength\x18\x02\x20\x01(\x04R\x0bi\
    ndexLength\x12\x1e\n\ndataLength\x18\x03\x20\x01(\x04R\ndataLength\x12\"\
    \n\x0cfooterLength\x18\x04\x20\x01(\x04R\x0cfooterLength\x12\"\n\x0cnumb\
    erOfRows\x18\x05\x20\x01(\x04R\x0cnumberOfRows\"<\n\x10UserMetadataItem\
    \x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\
    \x02\x20\x01(\x0cR\x05value\"K\n\x10StripeStatistics\x127\n\x08colStats\
    \x18\x01\x20\x03(\x0b2\x1b.orc.proto.ColumnStatisticsR\x08colStats\"I\n\
    \x08Metadata\x12=\n\x0bstripeStats\x18\x01\x20\x03(\x0b2\x1b.orc.proto.S\
    tripeStatisticsR\x0bstripeStats\"\x8b\x03\n\x06Footer\x12\"\n\x0cheaderL\
    ength\x18\x01\x20\x01(\x04R\x0cheaderLength\x12$\n\rcontentLength\x18\
    \x02\x20\x01(\x04R\rcontentLength\x126\n\x07stripes\x18\x03\x20\x03(\x0b\
    2\x1c.orc.proto.StripeInformationR\x07stripes\x12%\n\x05types\x18\x04\
    \x20\x03(\x0b2\x0f.orc.proto.TypeR\x05types\x127\n\x08metadata\x18\x05\
    \x20\x03(\x0b2\x1b.orc.proto.UserMetadataItemR\x08metadata\x12\"\n\x0cnu\
    mberOfRows\x18\x06\x20\x01(\x04R\x0cnumberOfRows\x12;\n\nstatistics\x18\
    \x07\x20\x03(\x0b2\x1b.orc.proto.ColumnStatisticsR\nstatistics\x12&\n\
    \x0erowIndexStride\x18\x08\x20\x01(\rR\x0erowIndexStride\x12\x16\n\x06wr\
    iter\x18\t\x20\x01(\rR\x06writer\"\xa5\x02\n\nPostScript\x12\"\n\x0cfoot\
    erLength\x18\x01\x20\x01(\x04R\x0cfooterLength\x12<\n\x0bcompression\x18\
    \x02\x20\x01(\x0e2\x1a.orc.proto.CompressionKindR\x0bcompression\x122\n\
    \x14compressionBlockSize\x18\x03\x20\x01(\x04R\x14compressionBlockSize\
    \x12\x1c\n\x07version\x18\x04\x20\x03(\rR\x07versionB\x02\x10\x01\x12&\n\
    \x0emetadataLength\x18\x05\x20\x01(\x04R\x0emetadataLength\x12$\n\rwrite\
    rVersion\x18\x06\x20\x01(\rR\rwriterVersion\x12\x15\n\x05magic\x18\xc0>\
    \x20\x01(\tR\x05magic\"\xb8\x01\n\x08FileTail\x125\n\npostscript\x18\x01\
    \x20\x01(\x0b2\x15.orc.proto.PostScriptR\npostscript\x12)\n\x06footer\
    \x18\x02\x20\x01(\x0b2\x11.orc.proto.FooterR\x06footer\x12\x1e\n\nfileLe\
    ngth\x18\x03\x20\x01(\x04R\nfileLength\x12*\n\x10postscriptLength\x18\
    \x04\x20\x01(\x04R\x10postscriptLength*M\n\x0fCompressionKind\x12\x08\n\
    \x04NONE\x10\0\x12\x08\n\x04ZLIB\x10\x01\x12\n\n\x06SNAPPY\x10\x02\x12\
    \x07\n\x03LZO\x10\x03\x12\x07\n\x03LZ4\x10\x04\x12\x08\n\x04ZSTD\x10\x05\
    B\x10\n\x0eorg.apache.orc\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
