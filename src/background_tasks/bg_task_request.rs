use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSDate,
    object,
    objective_c_runtime::{
        macros::interface_impl,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// An abstract class for representing task requests.
    unsafe pub struct BGTaskRequest;
}

#[interface_impl(NSObject)]
impl BGTaskRequest {
    /// The earliest date and time at which to run the task.
    #[property]
    pub fn earliest_begin_date(&self) -> NSDate {
        unsafe { NSDate::from_id(msg_send![self.m_self(), earliestBeginDate]) }
    }

    /// Sets the earliest date and time at which to run the task.
    #[property]
    pub fn set_earliest_begin_date(&mut self, date: NSDate) {
        unsafe { msg_send![self.m_self(), setEarliestBeginDate: date] }
    }
}

impl Default for BGTaskRequest {
    fn default() -> Self {
        Self::m_new()
    }
}
