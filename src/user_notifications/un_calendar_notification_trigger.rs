use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSDate, NSDateComponents},
    object,
    objective_c_runtime::traits::FromId,
    utils::to_optional,
};

use super::IUNNotificationTrigger;

object! {
    unsafe pub struct UNCalendarNotificationTrigger;
}

impl IUNNotificationTrigger for UNCalendarNotificationTrigger {}

#[interface_impl(UNNotificationTrigger)]
impl UNCalendarNotificationTrigger {
    /* Creating a Calendar Trigger
     */

    /// Creates a calendar trigger using the date components parameter.
    #[method]
    pub fn trigger_with_date_matching_components_repeats(
        date_components: NSDateComponents,
        repeats: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), triggerWithDateMatchingComponents: date_components repeats: repeats],
            )
        }
    }

    /* Getting the Trigger Information
     */

    /// The next date at which the trigger conditions are met.
    #[method]
    pub fn next_trigger_date(&self) -> Option<NSDate> {
        unsafe { to_optional(msg_send![self.m_self(), nextTriggerDate]) }
    }

    /// The date components to construct this object.
    #[property]
    pub fn date_components(&self) -> NSDateComponents {
        unsafe { NSDateComponents::from_id(msg_send![self.m_self(), dateComponents]) }
    }
}
