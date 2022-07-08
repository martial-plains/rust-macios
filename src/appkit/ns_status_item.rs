use objc::{msg_send, sel, sel_impl};

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
