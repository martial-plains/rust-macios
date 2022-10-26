use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::IBGTaskRequest;

object! {
    /// A request to launch your app in the background to execute a processing task that can take minutes to complete.
    unsafe pub struct BGProcessingTaskRequest;
}

#[interface_impl(BGTaskRequest)]
impl BGProcessingTaskRequest {
    /* Initializing a Processing Task Request
     */

    /// Return a new processing task request for the specified identifier.
    #[method]
    pub fn init_with_identifier(&mut self, identifier: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithIdentifier: identifier]) }
    }

    /* Setting Task Request Options
     */

    /// A Boolean specifying if the processing task requires a device connected to power.
    #[property]
    pub fn requires_external_power() -> bool {
        unsafe { to_bool(msg_send![Self::m_class(), requiresExternalPower]) }
    }

    /// A Boolean specifying if the processing task requires network connectivity.
    #[property]
    pub fn requires_network_connectivity() -> bool {
        unsafe { to_bool(msg_send![Self::m_class(), requiresNetworkConnectivity]) }
    }
}

impl IBGTaskRequest for BGProcessingTaskRequest {}

impl Default for BGProcessingTaskRequest {
    fn default() -> Self {
        Self::m_new()
    }
}
