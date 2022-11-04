use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    objective_c_runtime::{macros::object, traits::PNSObject},
    utils::to_bool,
};

object! {
    /// The common behavior for subclasses that trigger the delivery of a local or remote notification.
    unsafe pub struct UNNotificationTrigger;
}

#[interface_impl(NSObject)]
impl UNNotificationTrigger {
    /// A Boolean value indicating whether the system reschedules the notification after it’s delivered.
    #[property]
    pub fn repeats(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), repeats]) }
    }
}
