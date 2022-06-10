use std::{fmt, ops::Range};

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
        to_bool(unsafe { msg_send![self.ptr, isEqual: object] })
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isKindOfClass: class] })
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isMemberOfClass: class] })
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        to_bool(unsafe { msg_send![self.ptr, respondsToSelector: selector] })
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        to_bool(unsafe { msg_send![self.ptr, conformsToProtocol: protocol] })
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isProxy] })
    }
}

impl INSCharacterSet for NSCharacterSet {
    fn tp_alphanumeric_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), alphanumericCharacterSet])
        }
    }

    fn tp_capitalized_letter_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                capitalizedLetterCharacterSet
            ])
        }
    }

    fn tp_control_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), controlCharacterSet]) }
    }

    fn tp_decimal_digit_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), decimalDigitCharacterSet])
        }
    }

    fn tp_decomposable_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), decomposableCharacterSet])
        }
    }

    fn tp_illegal_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), illegalCharacterSet]) }
    }

    fn tp_letter_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), letterCharacterSet]) }
    }

    fn tp_lowercase_letter_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                lowercaseLetterCharacterSet
            ])
        }
    }

    fn tp_newline_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), newlineCharacterSet]) }
    }

    fn tp_non_base_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), nonBaseCharacterSet]) }
    }

    fn tp_punctuation_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), punctuationCharacterSet])
        }
    }

    fn tp_symbol_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), symbolCharacterSet]) }
    }

    fn tp_uppercase_letter_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                uppercaseLetterCharacterSet
            ])
        }
    }

    fn tp_whitespace_and_newline_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                whitespaceAndNewlineCharacterSet
            ])
        }
    }

    fn tp_whitespace_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), whitespaceCharacterSet])
        }
    }

    fn tp_urlfragment_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLFragmentAllowedCharacterSet
            ])
        }
    }

    fn tp_urlhost_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLHostAllowedCharacterSet
            ])
        }
    }

    fn tp_urlpassword_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLPasswordAllowedCharacterSet
            ])
        }
    }

    fn tp_urlpath_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLPathAllowedCharacterSet
            ])
        }
    }

    fn tp_urlquery_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLQueryAllowedCharacterSet
            ])
        }
    }

    fn tp_urluser_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLUserAllowedCharacterSet
            ])
        }
    }

    fn im_init_with_coder(self, coder: NSCoder) -> Self {
        unsafe {
            let ptr: id = msg_send![self.ptr, initWithCoder: coder];
            Self::from_id(ptr)
        }
    }

    fn tm_character_set_with_characters_in_string(string: NSString) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithCharactersInString: string
            ])
        }
    }

    fn tm_character_set_with_range(range: Range<UInt>) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithRange: range
            ])
        }
    }

    fn tm_character_set_with_bitmap_representation(data: NSData) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithBitmapRepresentation: data
            ])
        }
    }

    fn tm_character_set_with_contents_of_file(path: NSString) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithContentsOfFile: path
            ])
        }
    }

    fn ip_bitmap_representation(&self) -> NSData {
        unsafe { NSData::from_id(msg_send![self.ptr, bitmapRepresentation]) }
    }

    fn ip_inverted_set(&self) -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![self.ptr, invertedSet]) }
    }

    fn im_character_is_member(&self, character: unichar) -> bool {
        unsafe { msg_send![self.ptr, characterIsMember: character] }
    }

    fn im_has_member_in_plane(&self, plane: UInt8) -> bool {
        unsafe { msg_send![self.ptr, hasMemberInPlane: plane] }
    }

    fn im_is_superset_of_set(&self, other: NSCharacterSet) -> bool {
        unsafe { msg_send![self.ptr, isSupersetOfSet: other] }
    }

    fn im_long_character_is_member(&self, long_char: u32) -> bool {
        unsafe { msg_send![self.ptr, longCharacterIsMember: long_char] }
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
