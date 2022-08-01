use libc::{c_char, c_double, c_float, c_void};
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    core_graphics::{CGPoint, CGRect, CGSize},
    objective_c_runtime::{
        id,
        macros::object,
        traits::{PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{
    Int, Int32, Int64, NSDecodingFailurePolicy, NSPoint, NSRect, NSSize, NSString, UInt, UInt8,
};

object! {
    /// An abstract class that serves as the basis for objects that enable archiving and distribution of other objects.
    unsafe pub struct NSCoder;
}

#[interface_impl(NSObject)]
impl NSCoder {
    /* Inspecting a Coder
     */

    /// Returns a Boolean value that indicates whether an encoded value is available for a string.
    #[property]
    pub fn allows_keyed_coding(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), allowsKeyedCoding]) }
    }

    /// Returns a Boolean value that indicates whether an encoded value is available for a string.
    #[method]
    pub fn contains_value_for_key(&self, key: NSString) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), containsValueForKey: key]) }
    }

    /// The action the coder should take when decoding fails.
    #[property]
    pub fn decoding_failure_policy(&self) -> NSDecodingFailurePolicy {
        unsafe { msg_send![self.m_self(), decodingFailurePolicy] }
    }

    /* Encoding General Data
     */

    /// Encodes an array of the given Objective-C type, provided the number of items and a pointer.
    #[method]
    pub fn encode_array_of_objc_type_count_at(
        &mut self,
        type_: *const c_char,
        count: usize,
        array: *const c_void,
    ) {
        unsafe { msg_send![self.m_self(), encodeArrayOfObjCType: type_ count: count at: array] }
    }

    /// Encodes a Boolean value and associates it with the string key.
    #[method]
    pub fn encode_bool_for_key(&mut self, value: bool, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeBool: value forKey: key] }
    }

    /// An encoding method for subclasses to override such that it creates a copy, rather than a proxy, when decoded.
    #[method]
    pub fn encode_bycopy_object<T>(&mut self, object: T)
    where
        T: PNSObject + ToId,
    {
        unsafe { msg_send![self.m_self(), encodeBycopy: object.to_id()] }
    }

    /// An encoding method for subclasses to override such that it creates a proxy, rather than a copy, when decoded.
    #[method]
    pub fn encode_byref_object<T>(&mut self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.m_self(), encodeByref: object] }
    }

    /// Encodes a buffer of data of an unspecified type.
    #[method]
    pub fn encode_bytes_length(&mut self, bytes: *const c_void, length: UInt) {
        unsafe { msg_send![self.m_self(), encodeBytes: bytes length: length] }
    }

    /// Encodes a buffer of data, given its length and a pointer, and associates it with a string key.
    #[method]
    pub fn encode_bytes_length_for_key(
        &mut self,
        bytes: *const UInt8,
        length: UInt,
        key: NSString,
    ) {
        unsafe { msg_send![self.m_self(), encodeBytes: bytes length: length forKey: key] }
    }

    /// An encoding method for subclasses to override to conditionally encode an object, preserving common references to it.
    #[method]
    pub fn encode_conditional_object<T>(&mut self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.m_self(), encodeConditionalObject: object] }
    }

    /// An encoding method for subclasses to override to conditionally encode an object, preserving common references to it, only if it has been unconditionally encoded.
    #[method]
    pub fn encode_conditional_object_for_key<T>(&mut self, object: T, key: NSString)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.m_self(), encodeConditionalObject: object forKey: key] }
    }

    /// Encodes a given data object.
    #[method]
    pub fn encode_data_object<T>(&mut self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.m_self(), encodeDataObject: object] }
    }

    /// Encodes a double-precision floating point value and associates it with the string key.
    #[method]
    pub fn encode_double_for_key(&mut self, value: c_double, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeDouble: value forKey: key] }
    }

    /// Encodes a floating point value and associates it with the string key.
    #[method]
    pub fn encode_float_for_key(&mut self, value: c_float, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeFloat: value forKey: key] }
    }

    /// Encodes a C integer value and associates it with the string key.
    #[method]
    pub fn encode_int_for_key(&mut self, value: Int, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeInt: value forKey: key] }
    }

    /// Encodes an integer value and associates it with the string key.
    #[method]
    pub fn encode_integer_for_key(&mut self, value: Int, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeInteger: value forKey: key] }
    }

    /// Encodes a 32-bit integer value and associates it with the string key.
    #[method]
    pub fn encode_int32_for_key(&mut self, value: Int32, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeInt32: value forKey: key] }
    }

    /// Encodes a 64-bit integer value and associates it with the string key.
    #[method]
    pub fn encode_int64_for_key(&mut self, value: Int64, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeInt64: value forKey: key] }
    }

    /// Encodes an object.
    #[method]
    pub fn encode_object<T>(&mut self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.m_self(), encodeObject: object] }
    }

    /// Encodes an object and associates it with the string key.
    #[method]
    pub fn encode_object_for_key<T>(&mut self, object: T, key: NSString)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.m_self(), encodeObject: object forKey: key] }
    }

    /// Encodes a point.
    #[method]
    pub fn encode_point(&mut self, point: NSPoint) {
        unsafe { msg_send![self.m_self(), encodePoint: point] }
    }

    /// Encodes a point and associates it with the string key.
    #[method]
    pub fn encode_point_for_key(&mut self, point: NSPoint, key: NSString) {
        unsafe { msg_send![self.m_self(), encodePoint: point forKey: key] }
    }

    /// Encodes a property list.
    #[method]
    pub fn encode_property_list(&mut self, property_list: id) {
        unsafe { msg_send![self.m_self(), encodePropertyList: property_list] }
    }

    /// Encodes a rectangle structure.
    #[method]
    pub fn encode_rect(&mut self, rect: NSRect) {
        unsafe { msg_send![self.m_self(), encodeRect: rect] }
    }

    /// Encodes a rectangle structure and associates it with the string key.
    #[method]
    pub fn encode_rect_for_key(&mut self, rect: NSRect, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeRect: rect forKey: key] }
    }

    /// An encoding method for subclasses to override to encode an interconnected group of objects, starting with the provided root object.
    #[method]
    pub fn encode_root_object(&mut self, root_object: id) {
        unsafe { msg_send![self.m_self(), encodeRootObject: root_object] }
    }

    /// Encodes a size structure.
    #[method]
    pub fn encode_size(&mut self, size: NSSize) {
        unsafe { msg_send![self.m_self(), encodeSize: size] }
    }

    /// Encodes a size structure and associates it with the given string key.
    #[method]
    pub fn encode_size_for_key(&mut self, size: NSSize, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeSize: size forKey: key] }
    }

    /// Encodes a value of the given type at the given address.
    #[method]
    pub fn encode_value_of_objc_type_at(&mut self, _type: *const c_char, addr: *const c_void) {
        unsafe { msg_send![self.m_self(), encodeValueOfObjCType: _type at: addr] }
    }

    /// Encodes a point and associates it with the specified key in the receiver’s archive.
    #[method]
    pub fn encode_cgpoint_for_key(&mut self, point: CGPoint, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeCGPoint: point forKey: key] }
    }

    /// Encodes a rectangle and associates it with the specified key in the receiver’s archive.
    #[method]
    pub fn encode_cgrect_for_key(&mut self, rect: CGRect, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeCGRect: rect forKey: key] }
    }

    /// Encodes size information and associates it with the specified key in the coder’s archive.
    #[method]
    pub fn encode_cgzize_for_key(&mut self, size: CGSize, key: NSString) {
        unsafe { msg_send![self.m_self(), encodeCGSize: size forKey: key] }
    }
}
