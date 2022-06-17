use objc::{msg_send, sel, sel_impl};

use crate::{
    appkit::NSStatusItem,
    core_graphics::CGFloat,
    objective_c_runtime::traits::{FromId, PNSObject},
};

/// An object that manages a collection of status items displayed within the system-wide menu bar.
pub trait INSStatusBar: PNSObject {
    /* Getting the System-Wide Instance
     */

    /// Returns the system-wide status bar located in the menu bar.
    fn tp_system_status_bar() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), systemStatusBar]) }
    }

    /* Managing Status items
     */

    /// Returns a newly created status item that has been allotted a specified space within the status bar.
    fn im_status_item_with_length(&self, length: CGFloat) -> NSStatusItem {
        unsafe { NSStatusItem::from_id(msg_send![self.im_self(), statusItemWithLength: length]) }
    }
}
