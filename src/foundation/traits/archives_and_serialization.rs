use libc::{c_char, c_double, c_float, c_void};
use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{Int, Int32, Int64, NSDecodingFailurePolicy, UInt, UInt8},
    objective_c_runtime::traits::{PNSObject, ToId},
    utils::to_bool,
};

use super::INSString;

/// An abstract class that serves as the basis for objects that enable archiving and distribution of other objects.
pub trait INSCoder: PNSObject {
    /* Inspecting a Coder
     */

    /// Returns a Boolean value that indicates whether an encoded value is available for a string.
    fn ip_allows_keyed_coding(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), allowsKeyedCoding]) }
    }

    /// Returns a Boolean value that indicates whether an encoded value is available for a string.
    fn im_contains_value_for_key<K>(&self, key: K) -> bool
    where
        K: INSString,
    {
        unsafe { to_bool(msg_send![self.im_self(), containsValueForKey: key]) }
    }

    /// The action the coder should take when decoding fails.
    fn ip_decoding_failure_policy(&self) -> NSDecodingFailurePolicy {
        unsafe { msg_send![self.im_self(), decodingFailurePolicy] }
    }

    /* Encoding General Data
     */

    /// Encodes an array of the given Objective-C type, provided the number of items and a pointer.
    fn im_encode_array_of_objc_type_count_at(
        &self,
        type_: *const c_char,
        count: usize,
        array: *const c_void,
    ) {
        unsafe { msg_send![self.im_self(), encodeArrayOfObjCType: type_ count: count at: array] }
    }

    /// Encodes a Boolean value and associates it with the string key.
    fn im_encode_bool_for_key<K>(&self, value: bool, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeBool: value forKey: key] }
    }

    /// An encoding method for subclasses to override such that it creates a copy, rather than a proxy, when decoded.
    fn im_encode_bycopy_object<T>(&self, object: T)
    where
        T: PNSObject + ToId,
    {
        unsafe { msg_send![self.im_self(), encodeBycopy: object.to_id()] }
    }

    /// An encoding method for subclasses to override such that it creates a proxy, rather than a copy, when decoded.
    fn im_encode_byref_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.im_self(), encodeByref: object] }
    }

    /// Encodes a buffer of data of an unspecified type.
    fn im_encode_bytes_length(&self, bytes: *const c_void, length: UInt) {
        unsafe { msg_send![self.im_self(), encodeBytes: bytes length: length] }
    }

    /// Encodes a buffer of data, given its length and a pointer, and associates it with a string key.
    fn im_encode_bytes_length_for_key<K>(&self, bytes: *const UInt8, length: UInt, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeBytes: bytes length: length forKey: key] }
    }

    /// An encoding method for subclasses to override to conditionally encode an object, preserving common references to it.
    fn im_encode_conditional_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.im_self(), encodeConditionalObject: object] }
    }

    /// An encoding method for subclasses to override to conditionally encode an object, preserving common references to it, only if it has been unconditionally encoded.
    fn im_encode_conditional_object_for_key<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeConditionalObject: object forKey: key] }
    }

    /// Encodes a given data object.
    fn im_encode_data_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.im_self(), encodeDataObject: object] }
    }

    /// Encodes a double-precision floating point value and associates it with the string key.
    fn im_encode_double_for_key<K>(&self, value: c_double, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeDouble: value forKey: key] }
    }

    /// Encodes a floating point value and associates it with the string key.
    fn im_encode_float_for_key<K>(&self, value: c_float, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeFloat: value forKey: key] }
    }

    /// Encodes a C integer value and associates it with the string key.
    fn im_encode_int_for_key<K>(&self, value: Int, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeInt: value forKey: key] }
    }

    /// Encodes an integer value and associates it with the string key.
    fn im_encode_integer_for_key<K>(&self, value: Int, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeInteger: value forKey: key] }
    }

    /// Encodes a 32-bit integer value and associates it with the string key.
    fn im_encode_int32_for_key<K>(&self, value: Int32, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeInt32: value forKey: key] }
    }

    /// Encodes a 64-bit integer value and associates it with the string key.
    fn im_encode_int64_for_key<K>(&self, value: Int64, key: K)
    where
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeInt64: value forKey: key] }
    }

    /// Encodes an object.
    fn im_encode_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.im_self(), encodeObject: object] }
    }

    /// Encodes an object and associates it with the string key.
    fn im_encode_object_for_key<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: INSString,
    {
        unsafe { msg_send![self.im_self(), encodeObject: object forKey: key] }
    }
}
