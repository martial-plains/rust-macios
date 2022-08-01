use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// An abstract class representing a task that’s run while the app is in the background.
    unsafe pub struct BGTask;
}

#[interface_impl(NSObject)]
impl BGTask {
    /// The identifier of the task.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /// A handler called shortly before the task’s background time expires.
    #[property]
    pub fn expiration_handler(&self) {
        unsafe { msg_send![self.m_self(), expirationHandler] }
    }

    /// Informs the background task scheduler that the task is complete.
    ///
    /// # Arguments
    ///
    /// * success - A [`bool`] indicating if the task completed successfully or not.
    #[method]
    pub fn set_task_completed_with_success(&mut self, success: bool) {
        unsafe { msg_send![self.m_self(), setTaskCompletedWithSuccess: success] }
    }
}

impl Default for BGTask {
    fn default() -> Self {
        Self::m_new()
    }
}
