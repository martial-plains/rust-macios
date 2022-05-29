use crate::{
    foundation::{NSDate, NSString},
    objective_c_runtime::traits::PNSObject,
};

/// A class for scheduling task requests that launch your app in the background.
pub trait IBGTaskScheduler: PNSObject {
    /* Getting the Shared Task Scheduler
     */

    /// The shared background task scheduler instance.
    fn sharedScheduler() -> Self;
}

/// An abstract class for representing task requests.
pub trait IBGTaskRequest: PNSObject {
    /// The earliest date and time at which to run the task.
    fn earliestBeginDate() -> NSDate;
}

/// A request to launch your app in the background to execute a processing task that can take minutes to complete.
pub trait IBGProcessingTaskRequest: IBGTaskRequest {
    /* Initializing a Processing Task Request
     */

    /// Return a new processing task request for the specified identifier.
    fn initWithIdentifier(identifier: NSString) -> Self;

    /* Setting Task Request Options
     */

    /// A Boolean specifying if the processing task requires a device connected to power.
    fn requiresExternalPower() -> bool;

    /// A Boolean specifying if the processing task requires network connectivity.
    fn requiresNetworkConnectivity() -> bool;
}

/// A request to launch your app in the background to execute a short refresh task.
pub trait BGAppRefreshTaskRequest {
    /// Return a new refresh task request for the specified identifier.
    fn initWithIdentifier(identifier: NSString) -> Self;

}
