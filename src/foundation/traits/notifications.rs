use objc::{msg_send, sel, sel_impl};

use crate::objective_c_runtime::{traits::{FromId, PNSObject}, id};

/// A container for information broadcast through a notification center to all registered observers.
pub trait INSNotification: PNSObject {
    /// Initializes an empty notification.
    fn im_init() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let cls: id = msg_send![Self::im_class(), alloc];
            let ptr = msg_send![cls, init];
            FromId::from_id(ptr)
        }
    }
}
