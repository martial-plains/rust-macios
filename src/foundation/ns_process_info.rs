use std::fmt::Display;

use block::{ConcreteBlock, IntoConcreteBlock};
use libc::c_ulonglong;
use objc::{msg_send, sel, sel_impl};

use crate::{
    object,
    objective_c_runtime::{
        id,
        macros::interface_impl,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{Int, NSArray, NSDictionary, NSNotificationName, NSString, NSTimeInterval, UInt};

/// A structure that contains version information about the currently executing operating system, including major, minor, and patch version numbers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NSOperatingSystemVersion {
    /// The major release number, such as 10 in version 10.9.3.
    pub major: isize,
    /// The minor release number, such as 9 in version 10.9.3.
    pub minor: isize,
    /// The update release number, such as 3 in version 10.9.3.
    pub patch_version: isize,
}

impl Display for NSOperatingSystemVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch_version)
    }
}

/// Option flags used with `begin_activity_with_options_reason` and `perform_activity_with_options_reason_using_block`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum NSActivityOptions {
    /// A flag to require the screen to stay powered on.
    IdleDisplaySleepDisabled = 1 << 40,
    /// A flag to prevent idle sleep.
    IdleSystemSleepDisabled = 1 << 20,
    /// A flag to prevent sudden termination.
    SuddenTerminationDisabled = 1 << 14,
    /// A flag to prevent automatic termination.
    AutomaticTerminationDisabled = 1 << 15,
    /// A flag to indicate the app is performing a user-requested action.
    UserInitiated = 0x00FFFFFF | Self::IdleSystemSleepDisabled as u64,
    /// A flag to indicate the app is responding to user interaction.
    UserInteractive = (Self::UserInitiated as u64 | Self::LatencyCritical as u64),
    /// A flag to indicate the app has initiated some kind of work, but not as the direct result of user request.
    Background = 0x000000ff,
    /// A flag to indicate the activity requires the highest amount of timer and I/O precision available.
    LatencyCritical = 0xFF00000000,
    /// A flag to indicate the app is performing a user-requested action, but that the system can sleep on idle.
    InitiatedAllowingIdleSystemSleep =
        Self::UserInitiated as u64 & !(Self::IdleSystemSleepDisabled as u64),
    /// A flag to track the activity with an animation signpost interval.
    AnimationTrackingEnabled = (1 << 45),
    /// A flag to track the activity with a signpost interval.
    TrackingEnabled = (1 << 46),
}

/// Values used to indicate the system’s thermal state.
#[derive(Debug)]
#[repr(i64)]
pub enum NSProcessInfoThermalState {
    /// The thermal state is within normal limits.
    Nominal,
    /// The thermal state is slightly elevated.
    Fair,
    /// The thermal state is high.
    Serious,
    /// The thermal state is significantly impacting the performance of the system and the device needs to cool down.
    Critical,
}

object! {
    /// A collection of information about the current process.
    unsafe pub struct NSProcessInfo;
}

#[interface_impl(NSObject)]
impl NSProcessInfo {
    /* Getting the Process Information Agent
     */

    /// Returns the process information agent for the process.
    #[property]
    pub fn process_info() -> NSProcessInfo {
        unsafe { NSProcessInfo::from_id(msg_send![Self::m_class(), processInfo]) }
    }

    /* Getting the Process Information Agent
     */

    /// Array of strings with the command-line arguments for the process.
    #[property]
    pub fn arguments(&self) -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), arguments]) }
    }

    /// The variable names (keys) and their values in the environment from which the process was launched.
    #[property]
    pub fn environment(&self) -> NSDictionary<NSString, NSString> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), environment]) }
    }

    /// Global unique identifier for the process.
    #[property]
    pub fn globally_unique_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), globallyUniqueString]) }
    }

    /// A Boolean value that indicates whether the process originated as an iOS app and runs on macOS.
    #[property]
    pub fn mac_catalyst_app(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isMacCatalystApp]) }
    }

    /// A Boolean value that indicates whether the process is an iPhone or iPad app running on a Mac.
    #[property]
    pub fn ios_app_on_mac(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isiOSAppOnMac]) }
    }

    /// The identifier of the process (often called process ID).
    #[property]
    pub fn process_identifier(&self) -> Int {
        unsafe { msg_send![self.m_self(), processIdentifier] }
    }

    /// The name of the process.
    #[property]
    pub fn process_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), processName]) }
    }

    /// Sets the name of the process
    ///
    /// # Warning
    ///
    /// User defaults and other aspects of the environment might depend on the process name, so be very careful if you change it. Setting the process name in this manner is not thread safe.

    #[property]
    pub fn set_process_name(&mut self, name: NSString) {
        unsafe { msg_send![self.m_self(), setProcessName: name] }
    }

    /* Accessing User Information
     */

    /// Returns the account name of the current user.
    #[property]
    pub fn user_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), userName]) }
    }

    /// Returns the full name of the current user.
    #[property]
    pub fn full_user_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), fullUserName]) }
    }

    /* Sudden Application Termination
     */

    /// Disables the application for quickly killing using sudden termination.
    #[method]
    pub fn disable_sudden_termination(&mut self) {
        unsafe { msg_send![self.m_self(), disableSuddenTermination] }
    }

    /// Enables the application for quick killing using sudden termination.
    #[method]
    pub fn enable_sudden_termination(&mut self) {
        unsafe { msg_send![self.m_self(), enableSuddenTermination] }
    }

    /* Controlling Automatic Termination
     */

    /// Disables automatic termination for the application.
    #[method]
    pub fn disable_automatic_termination(&mut self, reason: &NSString) {
        unsafe { msg_send![self.m_self(), disableAutomaticTermination: reason.m_self()] }
    }

    /// Enables automatic termination for the application.
    #[method]
    pub fn enable_automatic_termination(&mut self, reason: &NSString) {
        unsafe { msg_send![self.m_self(), enableAutomaticTermination: reason.m_self()] }
    }

    /// A Boolean value indicating whether the app supports automatic termination.
    #[property]
    pub fn automatic_termination_support_enabled(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), automaticTerminationSupportEnabled]) }
    }

    /* Getting Host Information
     */

    /// The name of the host computer on which the process is executing.
    #[property]
    pub fn host_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), hostName]) }
    }

    /// A string containing the version of the operating system on which the process is executing.
    #[property]
    pub fn operating_system_version_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), operatingSystemVersionString]) }
    }

    /// The version of the operating system on which the process is executing.
    #[property]
    pub fn operating_system_version(&self) -> NSOperatingSystemVersion {
        unsafe { msg_send![self.m_self(), operatingSystemVersion] }
    }

    /// Returns a Boolean value indicating whether the version of the operating system on which the process is executing is the same or later than the given version.
    #[method]
    pub fn is_operating_system_at_least_version(&self, version: NSOperatingSystemVersion) -> bool {
        unsafe {
            to_bool(msg_send![
                self.m_self(),
                isOperatingSystemAtLeastVersion: version
            ])
        }
    }

    /// Returns a constant to indicate the operating system on which the process is executing.
    #[deprecated = "Use `operating_system_version` or `is_operating_system_at_least_version` instead"]
    #[method]
    pub fn operating_system(&self) -> UInt {
        unsafe { msg_send![self.m_self(), operatingSystem] }
    }

    /// Returns a string containing the name of the operating system on which the process is executing.
    #[method]
    #[deprecated = "Use `operating_system_version` or `is_operating_system_at_least_version` instead"]
    pub fn operating_system_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), operatingSystemName]) }
    }

    /* Getting Computer Information */

    /// The number of processing cores available on the computer.
    #[property]
    pub fn processor_count(&self) -> UInt {
        unsafe { msg_send![self.m_self(), processorCount] }
    }

    /// The number of active processing cores available on the computer.
    #[property]
    pub fn active_processor_count(&self) -> UInt {
        unsafe { msg_send![self.m_self(), activeProcessorCount] }
    }

    /// The amount of physical memory on the computer in bytes.
    #[property]
    pub fn physical_memory(&self) -> c_ulonglong {
        unsafe { msg_send![self.m_self(), physicalMemory] }
    }

    /// The amount of time the system has been awake since the last time it was restarted.
    #[property]
    pub fn system_uptime(&self) -> NSTimeInterval {
        unsafe { msg_send![self.m_self(), systemUptime] }
    }

    /* Managing Activities
     */

    /// Begin an activity using the given options and reason.
    #[method]
    pub fn begin_activity_with_options_reason(
        &self,
        options: NSActivityOptions,
        reason: &NSString,
    ) -> id {
        unsafe {
            msg_send![self.m_self(), beginActivityWithOptions: options reason: reason.m_self()]
        }
    }

    /// Ends the given activity.
    #[method]
    pub fn end_activity(&self, activity: id) {
        unsafe { msg_send![self.m_self(), endActivity: activity] }
    }

    /// Synchronously perform an activity defined by a given block using the given options.
    #[method]
    pub fn perform_activity_with_options_reason_using_block<F>(
        &self,
        activity: id,
        reason: &NSString,
        block: F,
    ) where
        F: IntoConcreteBlock<(), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), performActivityWithOptions: activity reason: reason.m_self() usingBlock: block]
        }
    }

    /// Performs the specified block asynchronously and notifies you if the process is about to be suspended.
    #[method]
    pub fn perform_expiring_activity_with_reason_using_block<F>(&self, reason: &NSString, block: F)
    where
        F: IntoConcreteBlock<(bool,), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), performExpiringActivityWithReason: reason.m_self() usingBlock: block]
        }
    }

    /*  Getting the Thermal State */

    /// The current thermal state of the system.
    #[property]
    pub fn thermal_state(&self) -> NSProcessInfoThermalState {
        unsafe { msg_send![self.m_self(), thermalState] }
    }

    /* Determining Whether Low Power Mode is Enabled
     */

    /// A Boolean value that indicates the current state of Low Power Mode.
    #[property]
    pub fn low_power_mode_enabled(&self) -> bool {
        unsafe { msg_send![self.m_self(), isLowPowerModeEnabled] }
    }
}

extern "C" {
    /// Posts when the thermal state of the system changes.
    pub static NSProcessInfoThermalStateDidChangeNotification: NSNotificationName;

    /// Posts when the power state of a device changes.
    pub static NSProcessInfoPowerStateDidChangeNotification: NSNotificationName;
}
