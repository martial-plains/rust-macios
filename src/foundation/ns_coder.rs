use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{traits::INSCoder, NSDecodingFailurePolicy};

/// An abstract class that serves as the basis for objects that enable archiving and distribution of other objects.
pub struct NSCoder {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSCoder {
    fn im_class<'a>() -> &'a Class {
        class!(NSCoder)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, isEqual: object] })
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, isKindOfClass: class] })
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, isMemberOfClass: class] })
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, respondsToSelector: selector] })
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, conformsToProtocol: protocol] })
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, isProxy] })
    }
}

impl INSCoder for NSCoder {
    fn ip_allows_keyed_coding(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, allowsKeyedCoding] })
    }

    fn im_contains_value_for_key<K>(&self, key: K) -> bool
    where
        K: Into<NSString>,
    {
        to_bool(unsafe { msg_send![&*self.ptr, containsValueForKey: key.into()] })
    }

    fn ip_decoding_failure_policy(&self) -> NSDecodingFailurePolicy {
        unsafe { msg_send![&*self.ptr, decodingFailurePolicy] }
    }

    fn im_encode_array_of_objc_type_count_at(
        &self,
        type_: *const libc::c_char,
        count: usize,
        array: *const libc::c_void,
    ) {
        unsafe {
            msg_send![
                self.ptr,
                encodeArrayOfObjCType: type_
                count: count
                at: array
            ]
        }
    }

    fn im_encode_bool_for_key<K>(&self, value: bool, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeBool: value forKey: key.into()] }
    }

    fn im_encode_bycopy_object<T>(&self, object: T)
    where
        T: PNSObject + ToId,
    {
        unsafe { msg_send![&*self.ptr, encodeBycopyObject: object.to_id()] }
    }

    fn im_encode_byref_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![&*self.ptr, encodeByrefObject: object] }
    }

    fn im_encode_bytes_length(&self, bytes: *const libc::c_void, length: UInt) {
        unsafe { msg_send![&*self.ptr, encodeBytes: bytes length: length] }
    }

    fn im_encode_bytes_length_for_key<K>(&self, bytes: *const super::UInt8, length: UInt, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeBytes: bytes length: length forKey: key.into()] }
    }

    fn im_encode_conditional_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![&*self.ptr, encodeConditionalObject: object] }
    }

    fn im_encode_conditional_object_for_key<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeConditionalObject: object forKey: key.into()] }
    }

    fn im_encode_data_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![&*self.ptr, encodeDataObject: object] }
    }

    fn im_encode_double_for_key<K>(&self, value: libc::c_double, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeDouble: value forKey: key.into()] }
    }

    fn im_encode_float_for_key<K>(&self, value: libc::c_float, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeFloat: value forKey: key.into()] }
    }

    fn im_encode_int_for_key<K>(&self, value: super::Int, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeInt: value forKey: key.into()] }
    }

    fn im_encode_integer_for_key<K>(&self, value: super::Int, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeInteger: value forKey: key.into()] }
    }

    fn im_encode_int32_for_key<K>(&self, value: super::Int32, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeInt32: value forKey: key.into()] }
    }

    fn im_encode_int64_for_key<K>(&self, value: super::Int64, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeInt64: value forKey: key.into()] }
    }

    fn im_encode_object<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![&*self.ptr, encodeObject: object] }
    }

    fn im_encode_object_for_key<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, encodeObject: object forKey: key.into()] }
    }
}

impl fmt::Debug for NSCoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSCoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSCoder {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSCoder {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
