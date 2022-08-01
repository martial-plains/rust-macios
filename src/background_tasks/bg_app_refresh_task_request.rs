use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    background_tasks::IBGTaskRequest,
    foundation::NSString,
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// A request to launch your app in the background to execute a short refresh task.
    unsafe pub struct BGAppRefreshTaskRequest;
}

#[interface_impl(BGTaskRequest)]
impl BGAppRefreshTaskRequest {
    /// Return a new refresh task request for the specified identifier.
    #[method]
    pub fn init_with_identifier(&mut self, identifier: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithIdentifier: identifier]) }
    }
}

impl Default for BGAppRefreshTaskRequest {
    fn default() -> Self {
        Self::m_new()
    }
}

impl IBGTaskRequest for BGAppRefreshTaskRequest {}
