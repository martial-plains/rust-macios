use objc::{msg_send, sel, sel_impl, Encode};
use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{
    id,
    macros::object,
    traits::{FromId, PNSObject},
};

object! {
    /// A container for information broadcast through a notification center to all registered observers.
    unsafe pub struct NSNotification;
}

unsafe impl Encode for NSNotification {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

#[interface_impl(NSObject)]
impl NSNotification {
    /// Initializes an empty notification.
    #[method]
    pub fn init() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let cls: id = msg_send![Self::m_class(), alloc];
            let ptr = msg_send![cls, init];
            FromId::from_id(ptr)
        }
    }
}
