use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::macros::object;

use super::IUNNotificationTrigger;

object! {
    /// A trigger condition that causes the system to deliver a notification when the user’s device enters or exits a geographic region you specify.
    unsafe pub struct UNLocationNotificationTrigger;
}

impl IUNNotificationTrigger for UNLocationNotificationTrigger {}

#[interface_impl(UNNotificationTrigger)]
impl UNLocationNotificationTrigger {
    /* Creating a Location Trigger
     */

    /// Creates a location trigger using the region parameter.
    ///
    /// Required features: `"core_location"`
    #[method]
    fn trigger_with_region_repeats(
        region: crate::core_location::CLRegion,
        repeats: objc::runtime::BOOL,
    ) -> Self
    where
        Self: Sized + crate::objective_c_runtime::traits::FromId,
    {
        use objc::{msg_send, sel, sel_impl};

        unsafe {
            Self::from_id(msg_send![Self::m_class(), triggerWithRegion: region repeats: repeats])
        }
    }

    /// The region used to determine when the system sends the notification.
    ///
    /// Required features: `"core_location"`
    #[property]
    fn region(&self) -> crate::core_location::CLRegion {
        use crate::objective_c_runtime::traits::FromId;
        use objc::{msg_send, sel, sel_impl};

        unsafe { crate::core_location::CLRegion::from_id(msg_send![self.m_self(), region]) }
    }
}
