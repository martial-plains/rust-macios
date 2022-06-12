use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{ns_coder::NSCoder, traits::INSCharacterSet, unichar, NSData, NSString, UInt, UInt8};

/// A character set containing the characters in Unicode General Categories L*, M*, and N*.
pub struct NSCharacterSet {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSCharacterSet {
    fn im_class<'a>() -> &'a Class {
        class!(NSCharacterSet)
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

impl INSCharacterSet for NSCharacterSet {
    fn im_init_with_coder(self, coder: NSCoder) -> Self {
        unsafe {
            let ptr: id = msg_send![&*self.ptr, initWithCoder: coder];
            Self::from_id(ptr)
        }
    }

    fn ip_bitmap_representation(&self) -> NSData {
        unsafe { NSData::from_id(msg_send![&*self.ptr, bitmapRepresentation]) }
    }

    fn ip_inverted_set(&self) -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![&*self.ptr, invertedSet]) }
    }

    fn im_character_is_member(&self, character: unichar) -> bool {
        unsafe { msg_send![&*self.ptr, characterIsMember: character] }
    }

    fn im_has_member_in_plane(&self, plane: UInt8) -> bool {
        unsafe { msg_send![&*self.ptr, hasMemberInPlane: plane] }
    }

    fn im_is_superset_of_set(&self, other: NSCharacterSet) -> bool {
        unsafe { msg_send![&*self.ptr, isSupersetOfSet: other] }
    }

    fn im_long_character_is_member(&self, long_char: u32) -> bool {
        unsafe { msg_send![&*self.ptr, longCharacterIsMember: long_char] }
    }
}

impl ToId for NSCharacterSet {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSCharacterSet {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}

impl fmt::Debug for NSCharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSCharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
