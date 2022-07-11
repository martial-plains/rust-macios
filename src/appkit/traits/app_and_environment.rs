use objc::{msg_send, sel, sel_impl};

use crate::{
    appkit::{
        ns_menu::NSMenu, register_app_delegate_class, NSApplicationActivationOptions,
        NSApplicationActivationPolicy, NSApplicationDelegateReply, NSApplicationTerminateReply,
        NSAPPLICATION_PTR,
    },
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::mouse_keyboard_and_trackpad::INSResponder;

/// An object that manages an app’s main event loop and resources used by all
/// of that app’s objects.
pub trait INSApplication: INSResponder {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    fn tp_shared_application() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), sharedApplication]) }
    }

    /// The app delegate object.
    fn ip_delegate(&self) -> id {
        unsafe { msg_send![self.im_self(), delegate] }
    }

    /// Sets the app delegate object.
    fn ip_set_delegate<'app, T>(&'app mut self, app_delegate: T)
    where
        T: PNSApplicationDelegate + 'app,
    {
        unsafe {
            let delegate_class = register_app_delegate_class::<T>();
            let delegate: id = msg_send![delegate_class, new];
            let delegate_ptr: *const T = &app_delegate;
            (*delegate).set_ivar(NSAPPLICATION_PTR, delegate_ptr as usize);
            msg_send![self.im_self(), setDelegate: delegate]
        }
    }

    /* Managing the Event Loop
     */

    /// A Boolean value indicating whether the main event loop is running.
    fn ip_running(&self) -> bool {
        unsafe { msg_send![self.im_self(), isRunning] }
    }

    /// Starts the main event loop.
    fn im_run(&self) {
        unsafe { msg_send![self.im_self(), run] }
    }

    /// Activates the app, opens any files specified by the NSOpen user default, and unhighlights the app’s icon.
    fn im_finish_launching(&self) {
        unsafe { msg_send![self.im_self(), finishLaunching] }
    }

    /// Stops the main event loop.
    fn im_stop(&self, sender: id) {
        unsafe { msg_send![self.im_self(), stop: sender] }
    }

    /* Terminating the App
     */

    /// Terminates the receiver.
    fn im_terminate(&self, sender: id) {
        unsafe { msg_send![self.im_self(), terminate: sender] }
    }

    /// Responds to NSTerminateLater once the app knows whether it can terminate.
    fn im_reply_to_application_should_terminate(&self, should_terminate: bool) {
        unsafe {
            msg_send![
                self.im_self(),
                replyToApplicationShouldTerminate: should_terminate
            ]
        }
    }

    /* Activating and Deactivating the App
     */

    /// A Boolean value indicating whether this is the active app.
    fn ip_active(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isActive]) }
    }

    /// Makes the receiver the active app.
    fn im_activate_ignoring_other_apps(&mut self, flag: bool) {
        unsafe { msg_send![self.im_self(), activateIgnoringOtherApps: flag] }
    }

    /// Deactivates the receiver.
    fn im_deactivate(&mut self) {
        unsafe { msg_send![self.im_self(), deactivate] }
    }

    /* Managing Relaunch on Login
     */

    /// Disables relaunching the app on login.
    fn im_disable_relaunch_on_login(&mut self) {
        unsafe { msg_send![self.im_self(), disableRelaunchOnLogin] }
    }

    /// Enables relaunching the app on login.
    fn im_enable_relaunch_on_login(&mut self) {
        unsafe { msg_send![self.im_self(), enableRelaunchOnLogin] }
    }

    /* Managing Remote Notifications
     */

    /// Register for notifications sent by Apple Push Notification service (APNs).
    fn im_register_for_remote_notifications(&mut self) {
        unsafe { msg_send![self.im_self(), registerForRemoteNotifications] }
    }

    /// Unregister for notifications received from Apple Push Notification service.
    fn im_unregister_for_remote_notifications(&mut self) {
        unsafe { msg_send![self.im_self(), unregisterForRemoteNotifications] }
    }

    /* Managing User Attention Requests
     */

    /// Handles errors that might occur when the user attempts to open or print files.
    fn im_reply_to_open_or_print(&self, response: NSApplicationDelegateReply) {
        unsafe { msg_send![self.im_self(), replyToOpenOrPrint: response] }
    }

    /* Configuring the Activation Policy
     */

    /// Returns the app’s activation policy.
    fn im_activation_policy(&self) -> NSApplicationActivationPolicy {
        unsafe { msg_send![self.im_self(), activationPolicy] }
    }

    /// Sets the app’s activation policy.
    ///
    /// # Arguments
    ///
    /// * `policy` - The activation policy to set.
    fn im_set_activation_policy(&mut self, policy: NSApplicationActivationPolicy) {
        unsafe { msg_send![self.im_self(), setActivationPolicy: policy] }
    }

    /* Menu */

    /// The app’s main menu bar.
    fn ip_main_menu(&self) -> NSMenu {
        unsafe { msg_send![self.im_self(), mainMenu] }
    }

    /// Sets the app’s main menu bar.
    fn ip_set_main_menu(&mut self, menu: NSMenu) {
        unsafe { msg_send![self.im_self(), setMainMenu: menu] }
    }
}

/// An object that can manipulate and provide information for a single instance of an app.
pub trait INSRunningApplication: PNSObject {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    fn tp_current_application() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), currentApplication]) }
    }

    /// Indicates whether the application is currently frontmost.
    fn ip_is_active(&self) -> bool {
        unsafe { msg_send![self.im_self(), isActive] }
    }

    /// Attempts to activate the application using the specified options.
    fn im_activate_with_options(&mut self, options: NSApplicationActivationOptions) {
        unsafe { msg_send![self.im_self(), activateWithOptions: options] }
    }
}

/// A set of methods that manage your app’s life cycle and its interaction
/// with common system services.

pub trait PNSApplicationDelegate {
    /// Called right before the application will finish launching. You really, probably, want to do
    /// your setup in `did_finish_launching` unless you're sure of what you're doing.
    fn will_finish_launching(&mut self) {}

    /// Fired when the application has finished launching.
    fn did_finish_launching(&mut self) {}

    /// Fired when the application is about to become active.
    fn did_become_active(&mut self) {}

    /// Fired when the application is about to resign active state.
    fn will_resign_active(&mut self) {}

    /// Fired when the user is going to continue an activity.
    fn will_continue_user_activity(&mut self, _activity_type: &str) -> bool {
        false
    }

    /// Fired before the application terminates. You can use this to do any required cleanup.
    fn will_terminate(&mut self) {}

    /// Fired immediately before the application is about to become active.
    fn will_become_active(&mut self) {}

    /// Fired when the application has resigned active state.
    fn did_resign_active(&mut self) {}

    /// Fired when the application is about to hide.
    fn will_hide(&mut self) {}

    /// Fired after the application has hidden.
    fn did_hide(&mut self) {}

    /// Fired when the application is about to unhide itself.
    fn will_unhide(&mut self) {}

    /// Fired after the application has unhidden itself.
    fn did_unhide(&mut self) {}

    /// Fired immediately before the application object updates its windows.
    fn will_update(&mut self) {}

    /// Fired immediately after the application object updates its windows.
    fn did_update(&mut self) {}

    /// This is fired after the `Quit` menu item has been selected, or after you've called `App::terminate()`.
    ///
    /// In most cases you just want `TerminateResponse::Now` here, which enables business as usual. If you need,
    /// though, you can cancel the termination via `TerminateResponse::Cancel` to continue something essential. If
    /// you do this, you'll need to be sure to call `App::reply_to_termination_request()` to circle
    /// back.
    fn should_terminate(&mut self) -> NSApplicationTerminateReply {
        NSApplicationTerminateReply::Now
    }

    /// Called after closing the last open window. Return `true` here if you want
    /// the application to terminate.
    fn should_terminate_after_last_window_closed(&mut self) -> bool {
        false
    }

    /// Sent by the application to the delegate prior to default behavior to reopen AppleEvents.
    ///
    /// `has_visible_windows` indicates whether the Application object found any visible windows in your application.
    /// You can use this value as an indication of whether the application would do anything if you return `true`.
    ///
    /// Return `true` if you want the application to perform its normal tasks, or `false` if you want the
    /// application to do nothing. The default implementation of this method returns `true`.
    ///
    /// Some finer points of discussion, from Apple documentation:
    ///
    /// These events are sent whenever the Finder reactivates an already running application because someone
    /// double-clicked it again or used the dock to activate it.
    ///
    /// For most document-based applications, an untitled document will be created.
    ///
    /// [Read more
    /// here](https://developer.apple.com/documentation/appkit/nsapplicationdelegate/1428638-applicationshouldhandlereopen?language=objc)
    fn should_handle_reopen(&mut self, _has_visible_windows: bool) -> bool {
        true
    }

    /// Supply a dock menu for the application dynamically. The default implementation for this
    /// method returns `None`, for no menu.
    fn dock_menu(&mut self) -> Option<NSMenu> {
        None
    }
}
