#![allow(clippy::let_unit_value)]

use std::{fmt, marker::PhantomData, sync::Once};

use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{
    ns_application_delegate::PNSApplicationDelegate, register_app_delegate_class, INSResponder,
    NSApplicationActivationPolicy, NSApplicationDelegateReply, NSMenu,
};
///
pub static NSAPPLICATION_PTR: &str = "rstNSApplicationPtr";

fn register_app_class() -> *const Class {
    static mut APP_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSApplication);
        let decl = ClassDecl::new("RSTNSApplication", superclass).unwrap();
        APP_CLASS = decl.register();
    });

    unsafe { APP_CLASS }
}

/// An object that manages an app’s main event loop and resources used by all of that app’s objects.
pub struct NSApplication<'app, M = ()> {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
    _message: PhantomData<&'app M>,
}

/// An object that manages an app’s main event loop and resources used by all
/// of that app’s objects.
pub trait INSApplication: INSResponder {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    fn tp_shared_application() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), sharedApplication]) }
    }

    /// The app delegate object.
    fn ip_delegate(&self) -> id {
        unsafe { msg_send![self.m_self(), delegate] }
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
            msg_send![self.m_self(), setDelegate: delegate]
        }
    }

    /* Managing the Event Loop
     */

    /// A Boolean value indicating whether the main event loop is running.
    fn ip_running(&self) -> bool {
        unsafe { msg_send![self.m_self(), isRunning] }
    }

    /// Starts the main event loop.
    fn im_run(&self) {
        unsafe { msg_send![self.m_self(), run] }
    }

    /// Activates the app, opens any files specified by the NSOpen user default, and unhighlights the app’s icon.
    fn im_finish_launching(&self) {
        unsafe { msg_send![self.m_self(), finishLaunching] }
    }

    /// Stops the main event loop.
    fn im_stop(&self, sender: id) {
        unsafe { msg_send![self.m_self(), stop: sender] }
    }

    /* Terminating the App
     */

    /// Terminates the receiver.
    fn im_terminate(&self, sender: id) {
        unsafe { msg_send![self.m_self(), terminate: sender] }
    }

    /// Responds to NSTerminateLater once the app knows whether it can terminate.
    fn im_reply_to_application_should_terminate(&self, should_terminate: bool) {
        unsafe {
            msg_send![
                self.m_self(),
                replyToApplicationShouldTerminate: should_terminate
            ]
        }
    }

    /* Activating and Deactivating the App
     */

    /// A Boolean value indicating whether this is the active app.
    fn ip_active(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isActive]) }
    }

    /// Makes the receiver the active app.
    fn im_activate_ignoring_other_apps(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), activateIgnoringOtherApps: flag] }
    }

    /// Deactivates the receiver.
    fn im_deactivate(&mut self) {
        unsafe { msg_send![self.m_self(), deactivate] }
    }

    /* Managing Relaunch on Login
     */

    /// Disables relaunching the app on login.
    fn im_disable_relaunch_on_login(&mut self) {
        unsafe { msg_send![self.m_self(), disableRelaunchOnLogin] }
    }

    /// Enables relaunching the app on login.
    fn im_enable_relaunch_on_login(&mut self) {
        unsafe { msg_send![self.m_self(), enableRelaunchOnLogin] }
    }

    /* Managing Remote Notifications
     */

    /// Register for notifications sent by Apple Push Notification service (APNs).
    fn im_register_for_remote_notifications(&mut self) {
        unsafe { msg_send![self.m_self(), registerForRemoteNotifications] }
    }

    /// Unregister for notifications received from Apple Push Notification service.
    fn im_unregister_for_remote_notifications(&mut self) {
        unsafe { msg_send![self.m_self(), unregisterForRemoteNotifications] }
    }

    /* Managing User Attention Requests
     */

    /// Handles errors that might occur when the user attempts to open or print files.
    fn im_reply_to_open_or_print(&self, response: NSApplicationDelegateReply) {
        unsafe { msg_send![self.m_self(), replyToOpenOrPrint: response] }
    }

    /* Configuring the Activation Policy
     */

    /// Returns the app’s activation policy.
    fn im_activation_policy(&self) -> NSApplicationActivationPolicy {
        unsafe { msg_send![self.m_self(), activationPolicy] }
    }

    /// Sets the app’s activation policy.
    ///
    /// # Arguments
    ///
    /// * `policy` - The activation policy to set.
    fn im_set_activation_policy(&mut self, policy: NSApplicationActivationPolicy) {
        unsafe { msg_send![self.m_self(), setActivationPolicy: policy] }
    }

    /* Menu */

    /// The app’s main menu bar.
    fn ip_main_menu(&self) -> NSMenu {
        unsafe { msg_send![self.m_self(), mainMenu] }
    }

    /// Sets the app’s main menu bar.
    fn ip_set_main_menu(&mut self, menu: NSMenu) {
        unsafe { msg_send![self.m_self(), setMainMenu: menu] }
    }
}

impl<'app> NSApplication<'app> {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    pub fn shared_application() -> NSApplication<'app> {
        NSApplication::tp_shared_application()
    }

    /// The app delegate object.
    pub fn delegate(&self) -> id {
        self.ip_delegate()
    }
}

impl NSApplication<'_> {
    /// A Boolean value indicating whether the main event loop is running.
    pub fn running(&self) -> bool {
        self.ip_running()
    }

    /// Starts the main event loop.
    pub fn run(&mut self) {
        self.im_run();
    }

    /// Activates the app, opens any files specified by the NSOpen user default, and unhighlights the app’s icon.
    pub fn finish_launching(&mut self) {
        self.im_finish_launching()
    }

    /// Stops the main event loop.
    pub fn stop(&mut self, sender: id) {
        self.im_stop(sender)
    }

    /// Terminates the receiver.
    pub fn terminate(&mut self, sender: id) {
        self.im_terminate(sender)
    }

    /// Responds to NSTerminateLater once the app knows whether it can terminate.
    pub fn reply_to_application_should_terminate(&self, should_terminate: bool) {
        self.im_reply_to_application_should_terminate(should_terminate)
    }

    /// Disables relaunching the app on login.
    pub fn disable_relaunch_on_login(&mut self) {
        self.im_disable_relaunch_on_login()
    }

    /// Enables relaunching the app on login.
    pub fn enable_relaunch_on_login(&mut self) {
        self.im_enable_relaunch_on_login()
    }

    /// Register for notifications sent by Apple Push Notification service (APNs).
    pub fn register_for_remote_notifications(&mut self) {
        self.im_register_for_remote_notifications()
    }

    /// Unregister for notifications received from Apple Push Notification service.
    pub fn unregister_for_remote_notifications(&mut self) {
        self.im_unregister_for_remote_notifications()
    }

    /// Handles errors that might occur when the user attempts to open or print files.
    pub fn reply_to_open_or_print(&self, response: NSApplicationDelegateReply) {
        self.im_reply_to_open_or_print(response)
    }

    /// Returns the app’s activation policy.
    pub fn activation_policy(&self) -> NSApplicationActivationPolicy {
        self.im_activation_policy()
    }

    /// Sets the app’s activation policy.
    ///
    /// # Arguments
    ///
    /// * `policy` - The activation policy to set.
    pub fn set_activation_policy(&mut self, policy: NSApplicationActivationPolicy) {
        self.im_set_activation_policy(policy)
    }

    /// The app’s main menu bar.
    pub fn main_menu(&self) -> NSMenu {
        self.ip_main_menu()
    }

    /// Sets the app’s main menu bar.
    pub fn set_main_menu(&mut self, menu: NSMenu) {
        self.ip_set_main_menu(menu)
    }
}

impl NSApplication<'_> {
    /// Creates a new [`NSApplication`],
    pub fn new() -> Self {
        let ptr = unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            Id::from_ptr(app)
        };

        Self {
            ptr,
            _message: PhantomData,
        }
    }
}

impl Default for NSApplication<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSApplication<'_> {
    fn m_class<'a>() -> &'a Class {
        unsafe { &*register_app_class() }
    }

    fn m_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSResponder for NSApplication<'_> {}

impl INSApplication for NSApplication<'_> {}

impl fmt::Debug for NSApplication<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.p_debug_description())
    }
}

impl ToId for NSApplication<'_> {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSApplication<'_> {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
            _message: PhantomData,
        }
    }
}
