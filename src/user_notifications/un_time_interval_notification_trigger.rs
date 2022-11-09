use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSDate, NSTimeInterval},
    object,
    objective_c_runtime::traits::FromId,
    utils::to_optional,
};

use super::IUNNotificationTrigger;

object! {
    /// A trigger condition that causes the system to deliver a notification after the amount of time you specify elapses.
    unsafe pub struct UNTimeIntervalNotificationTrigger;
}

impl IUNNotificationTrigger for UNTimeIntervalNotificationTrigger {}

#[interface_impl(UNNotificationTrigger)]
impl UNTimeIntervalNotificationTrigger {
    /* Creating a Time Interval Trigger
     */

    /// Creates a time interval trigger using the time value parameter.
    #[method]
    pub fn trigger_with_time_interval_repeats(time_interval: NSTimeInterval, repeats: bool) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                triggerWithTimeInterval: time_interval
                repeats: repeats
            ])
        }
    }

    /* Getting the Trigger Information
     */

    /// The next date at which the trigger conditions are met.
    #[method]
    pub fn next_trigger_date(&self) -> Option<NSDate> {
        unsafe { to_optional(msg_send![self.m_self(), nextTriggerDate]) }
    }

    /// The time interval to create the trigger.
    #[property]
    pub fn time_interval(&self) -> NSTimeInterval {
        unsafe { msg_send![self.m_self(), timeInterval] }
    }
}
