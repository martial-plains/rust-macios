use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::NSBundle,
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
};

use super::{
    traits::{INSResponder, INSViewController},
    NSNibName,
};

/// A controller that manages a view, typically loaded from a nib file.
pub struct NSViewController {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl NSViewController {
    /// Creates a new view controller.
    pub fn new() -> Self {
        unsafe {
            let obj = msg_send![class!(NSViewController), new];
            Self {
                ptr: Id::from_ptr(obj),
            }
        }
    }
}

impl Default for NSViewController {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSViewController {
    fn im_class<'a>() -> &'a Class {
        class!(NSViewController)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSResponder for NSViewController {}

impl INSViewController for NSViewController {
    fn im_init_with_nib_name_bundle(nib_name: NSNibName, bundle: NSBundle) -> Self {
        unsafe {
            let ptr: id = msg_send![class!(NSViewController), alloc];
            let ptr: id = msg_send![ptr, initWithNibName: nib_name bundle: bundle];
            Self::from_id(ptr)
        }
    }

    fn im_load_view(&self) {
        unsafe { msg_send![&*self.ptr, loadView] }
    }

    fn ip_represented_object(&self) -> id {
        unsafe { msg_send![&*self.ptr, representedObject] }
    }
}

impl fmt::Debug for NSViewController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSViewController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSViewController {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}
impl FromId for NSViewController {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
