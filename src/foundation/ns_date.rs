use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
};

use super::{traits::INSDate, NSComparisonResult, NSTimeInterval};

/// A representation of a specific point in time, independent of any calendar or time zone.
pub struct NSDate {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSDate {
    fn im_class<'a>() -> &'a Class {
        class!(NSDate)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![&*self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isKindOfClass: class] }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isMemberOfClass: class] }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send![&*self.ptr, respondsToSelector: selector] }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { msg_send![&*self.ptr, conformsToProtocol: protocol] }
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
        unsafe { msg_send![&*self.ptr, isProxy] }
    }
}

impl INSDate for NSDate {
    fn im_is_equal_to_date(&self, date: NSDate) -> bool {
        unsafe { msg_send![&*self.ptr, isEqualToDate: date] }
    }

    fn im_earlier_date(&self, date: NSDate) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, earlierDate: date]) }
    }

    fn im_later_date(&self, date: NSDate) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, laterDate: date]) }
    }

    fn im_compare(&self, date: NSDate) -> NSComparisonResult {
        unsafe { msg_send![&*self.ptr, compare: date] }
    }

    fn im_time_interval_since_date(&self, date: NSDate) -> NSTimeInterval {
        unsafe { msg_send![&*self.ptr, timeIntervalSinceDate: date] }
    }

    fn ip_time_interval_since_now(&self) -> NSTimeInterval {
        unsafe { msg_send![&*self.ptr, timeIntervalSinceNow] }
    }

    fn im_date_by_adding_time_interval(&self, secs_to_be_added: NSTimeInterval) -> Self {
        unsafe {
            Self::from_id(msg_send![
                &*self.ptr,
                dateByAddingTimeInterval: secs_to_be_added
            ])
        }
    }
}

impl ToId for NSDate {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSDate {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl fmt::Debug for NSDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_description())
    }
}

impl Clone for NSDate {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, retain]) }
    }
}
