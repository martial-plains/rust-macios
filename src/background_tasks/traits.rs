use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSDate, NSString},
    objective_c_runtime::traits::{FromId, PNSObject},
    utils::to_bool,
};

/// A class for scheduling task requests that launch your app in the background.
pub trait IBGTaskScheduler: PNSObject {
    /* Getting the Shared Task Scheduler
     */

    /// The shared background task scheduler instance.
    fn tp_shared_scheduler() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), sharedScheduler]) }
    }
}

/// An abstract class for representing task requests.
pub trait IBGTaskRequest: PNSObject {
    /// The earliest date and time at which to run the task.
    fn ip_earliest_begin_date() -> NSDate {
        unsafe { NSDate::from_id(msg_send![Self::im_class(), earliestBeginDate]) }
    }
}

/// A request to launch your app in the background to execute a processing task that can take minutes to complete.
pub trait IBGProcessingTaskRequest: IBGTaskRequest {
    /* Initializing a Processing Task Request
     */

    /// Return a new processing task request for the specified identifier.
    fn im_init_with_identifier(identifier: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), initWithIdentifier: identifier]) }
    }

    /* Setting Task Request Options
     */

    /// A Boolean specifying if the processing task requires a device connected to power.
    fn ip_requires_external_power() -> bool {
        unsafe { to_bool(msg_send![Self::im_class(), requiresExternalPower]) }
    }

    /// A Boolean specifying if the processing task requires network connectivity.
    fn ip_requires_network_connectivity() -> bool {
        unsafe { to_bool(msg_send![Self::im_class(), requiresNetworkConnectivity]) }
    }
}

/// A request to launch your app in the background to execute a short refresh task.
pub trait BGAppRefreshTaskRequest : IBGTaskRequest {
    /// Return a new refresh task request for the specified identifier.
    fn im_init_with_identifier(identifier: NSString) -> Self where Self: Sized + FromId {
        unsafe { Self::from_id(msg_send![Self::im_class(), initWithIdentifier: identifier]) }
    }
}

/// An abstract class representing a task that’s run while the app is in the background.
pub trait IBGTask: PNSObject {
    /// The identifier of the task.
    fn ip_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), identifier]) }
    }

    /// A handler called shortly before the task’s background time expires.
    fn ip_expiration_handler(&self) {
        unsafe { msg_send![self.im_self(), expirationHandler] }
    }

    /// Informs the background task scheduler that the task is complete.
    fn im_set_task_completed_with_success(&self, success: bool) {
        unsafe { msg_send![self.im_self(), setTaskCompletedWithSuccess: success] }
    }
}

/// A task that runs while the app is in the background.
pub trait IBGProcessingTask: IBGTask {}

/// An object representing a short task typically used to refresh content
/// that’s run while the app is in the background.
pub trait IBGAppRefreshTask: IBGTask {}
