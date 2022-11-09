use objc::{msg_send, sel, sel_impl};

use crate::{
    core_graphics::CGFloat,
    object,
    objective_c_runtime::traits::{FromId, PNSObject},
};

use super::{interface_impl, NSStatusItem};

object! {
     /// An individual element displayed in the system menu bar.
    unsafe pub struct NSStatusBar;
}

#[interface_impl(NSObject)]
impl NSStatusBar {
    /* Getting the System-Wide Instance
     */

    /// Returns the system-wide status bar located in the menu bar.
    #[property]
    pub fn system_status_bar() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), systemStatusBar]) }
    }

    /* Managing Status items
     */

    /// Returns a newly created status item that has been allotted a specified space within the status bar.
    #[method]
    pub fn status_item_with_length(&self, length: CGFloat) -> NSStatusItem {
        unsafe { NSStatusItem::from_id(msg_send![self.m_self(), statusItemWithLength: length]) }
    }
}
