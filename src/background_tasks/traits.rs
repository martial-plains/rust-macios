use crate::{
    foundation::{NSDate, NSString},
    objective_c_runtime::traits::INSObject,
};

/// A class for scheduling task requests that launch your app in the background.
pub trait IBGTaskScheduler: INSObject {
    /* Getting the Shared Task Scheduler
     */

    /// The shared background task scheduler instance.
    fn tp_sharedScheduler() -> Self;
}

/// An abstract class for representing task requests.
pub trait IBGTaskRequest: INSObject {
    /// The earliest date and time at which to run the task.
    fn ip_earliestBeginDate() -> NSDate;
}

/// A request to launch your app in the background to execute a processing task that can take minutes to complete.
pub trait IBGProcessingTaskRequest: IBGTaskRequest {
    /* Initializing a Processing Task Request
     */

    /// Return a new processing task request for the specified identifier.
    fn im_initWithIdentifier(identifier: NSString) -> Self;

    /* Setting Task Request Options
     */

    /// A Boolean specifying if the processing task requires a device connected to power.
    fn ip_requiresExternalPower() -> bool;

    /// A Boolean specifying if the processing task requires network connectivity.
    fn ip_requiresNetworkConnectivity() -> bool;
}

/// A request to launch your app in the background to execute a short refresh task.
pub trait BGAppRefreshTaskRequest {
    /// Return a new refresh task request for the specified identifier.
    fn im_initWithIdentifier(identifier: NSString) -> Self;
}

/// An abstract class representing a task that’s run while the app is in the background.
pub trait IBGTask {
    /// The identifier of the task.
    fn ip_identifier() -> NSString;

    /// A handler called shortly before the task’s background time expires.
    fn ip_expirationHandler();

    /// Informs the background task scheduler that the task is complete.
    fn im_setTaskCompletedWithSuccess(&self, success: bool);
}

/// A task that runs while the app is in the background.
pub trait IBGProcessingTask: IBGTask {}

/// An object representing a short task typically used to refresh content
/// that’s run while the app is in the background.
pub trait IBGAppRefreshTask: IBGTask {}
