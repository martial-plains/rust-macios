use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{
    macros::object,
    traits::{FromId, PNSObject},
};

object! {
    /// Returns the singleton instance of NSNull.
    unsafe pub struct NSNull;

}

#[interface_impl(NSObject)]
impl NSNull {
    /* Obtaining an instance
     */

    /// Returns the singleton instance of NSNull.
    #[method]
    pub fn null() -> NSNull {
        unsafe { NSNull::from_id(msg_send![Self::m_class(), null]) }
    }
}
