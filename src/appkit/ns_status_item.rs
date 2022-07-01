use std::ops::{Deref, DerefMut};

use objc::{msg_send, runtime::Object, sel, sel_impl};

use crate::{
    core_graphics::CGFloat,
    objective_c_runtime::traits::{FromId, PNSObject},
};

use super::{object, traits::INSStatusItem};

object! {
    /// An individual element displayed in the system menu bar.
    unsafe pub struct NSStatusItem;
}

impl NSStatusItem {
    /// A status item length that is equal to the status bar’s thickness.
    pub const NSSQUARE_STATUS_ITEM_LENGTH: CGFloat = -2.0;

    /// A status item length that dynamically adjusts to the width of its contents.
    pub const NSVARIABLE_STATUS_ITEM_LENGTH: CGFloat = -1.0;

    /// Creates a new status item.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![NSStatusItem::im_class(), alloc]) }
    }
}

impl Default for NSStatusItem {
    fn default() -> Self {
        Self::new()
    }
}

impl INSStatusItem for NSStatusItem {}

impl Deref for NSStatusItem {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        unsafe { &*self.im_self() }
    }
}

impl DerefMut for NSStatusItem {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.im_self() }
    }
}

impl Clone for NSStatusItem {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![&*self.im_self(), retain]) }
    }
}
