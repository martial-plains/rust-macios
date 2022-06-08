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
    foundation::{NSArray, NSAutoreleasePool},
    id,
    objective_c_runtime::{
        nil,
        traits::{FromId, PNSObject},
    },
};

use super::{
    ns_application_delegate::register_app_delegate_class, ns_menu::NSMenu,
    ns_menu_item::NSMenuItem, INSApplication, INSResponder, NSApplicationActivationOptions,
    NSApplicationActivationPolicy, NSApplicationDelegateReply, NSRunningApplication,
    PNSApplicationDelegate,
};

pub(crate) static NSApplication_PTR: &str = "rstNSApplicationPtr";

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

#[inline]
fn shared_application<F: Fn(NSApplication)>(handler: F) {
    let app: NSApplication = unsafe { msg_send![register_app_class(), sharedApplication] };
    handler(app);
}

/// An object that manages an app’s main event loop and resources used by all of that app’s objects.
pub struct NSApplication<T = (), M = ()> {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
    /// The delegate of the underlying Objective-C object.
    pub ptr_delegate: Id<Object>,
    /// The stored delegate of the underlying Objective-C object.
    pub delegate: Option<Box<T>>,
    /// The main-thread AutoReleasePool. Drains on app exit.
    pub pool: NSAutoreleasePool,
    _message: PhantomData<M>,
}

impl NSApplication {
    /// Registers for remote notifications from APNS.
    pub fn register_for_remote_notifications() {
        shared_application(|app| app.im_registerForRemoteNotifications());
    }

    /// Unregisters for remote notifications from APNS.
    pub fn unregister_for_remote_notifications() {
        shared_application(|app| {
            app.im_unregisterForRemoteNotifications();
        });
    }

    /// Sets whether this application should relaunch at login.
    pub fn set_relaunch_on_login(relaunch: bool) {
        shared_application(|app| {
            if relaunch {
                app.im_enableRelaunchOnLogin();
            } else {
                app.im_disableRelaunchOnLogin();
            }
        });
    }

    /// Respond to a termination request. Generally done after returning `TerminateResponse::Later`
    /// from your trait implementation of `should_terminate()`.
    pub fn reply_to_termination_request(should_terminate: bool) {
        shared_application(|app| {
            app.replyToApplicationShouldTerminate(should_terminate);
        });
    }

    /// An optional call that you can use for certain scenarios surrounding opening/printing files.
    pub fn reply_to_open_or_print(response: NSApplicationDelegateReply) {
        shared_application(|app| app.replyToOpenOrPrint(response));
    }

    /// Sets a set of `Menu`'s as the top level Menu for the current application. Note that behind
    /// the scenes, Cocoa/AppKit make a copy of the menu you pass in - so we don't retain it, and
    /// you shouldn't bother to either.
    pub fn set_menu(menus: NSArray<NSMenu>) {
        let main_menu = {
            let mut main_menu = NSMenu::new();

            for menu in menus.iter() {
                let mut item = NSMenuItem::new();
                item.set_submenu(menu);
                main_menu.add_item(item);
            }

            main_menu
        };

        shared_application(move |app| app.ip_setMainMenu(main_menu.clone()));
    }

    /// Terminates the application, firing the requisite cleanup delegate methods in the process.
    ///
    /// This is typically called when the user chooses to quit via the App menu.
    pub fn terminate() {
        shared_application(|app| app.im_terminate(nil));
    }

    /// For nib-less applications (which, if you're here, this is) need to call the activation
    /// routines after the NSMenu has been set, otherwise it won't be interact-able without
    /// switching away from the app and then coming back.
    pub fn activate<T>(app: &NSApplication<T>) {
        app.im_setActivationPolicy(NSApplicationActivationPolicy::Regular);
        let mut current_app = NSRunningApplication::current_application();
        current_app.activate_with_options(NSApplicationActivationOptions::IgnoringOtherWindows);
    }
}

impl<T> NSApplication<T> {
    /// Starts the main event loop.
    pub fn run(&mut self) {
        unsafe {
            let _: () = msg_send![self.ptr, run];
        };
        self.pool.drain();
    }
}

impl<T> NSApplication<T>
where
    T: PNSApplicationDelegate + 'static,
{
    /// Creates a new `NSApplication` with the given delegate.
    pub fn new(_bundle_id: &str, delegate: T) -> Self {
        // set_bundle_id(bundle_id);

        let pool = NSAutoreleasePool::new();

        let ptr = unsafe {
            let app: id = msg_send![register_app_class(), sharedApplication];
            Id::from_ptr(app)
        };

        let app_delegate = Box::new(delegate);

        let ptr_delegate = unsafe {
            let delegate_class = register_app_delegate_class::<T>();
            let delegate: id = msg_send![delegate_class, new];
            let delegate_ptr: *const T = &*app_delegate;
            (&mut *delegate).set_ivar(NSApplication_PTR, delegate_ptr as usize);
            let _: () = msg_send![&*ptr, setDelegate: delegate];
            Id::from_ptr(delegate)
        };

        Self {
            ptr,
            ptr_delegate,
            delegate: Some(app_delegate),
            pool,
            _message: PhantomData,
        }
    }
}

impl<T> PNSObject for NSApplication<T, ()> {
    fn im_class<'a>() -> &'a Class {
        unsafe { &*register_app_class() }
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> crate::foundation::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> crate::foundation::NSString {
        unsafe { msg_send![self.ptr, description] }
    }

    fn ip_debugDescription(&self) -> crate::foundation::NSString {
        unsafe { msg_send![self.ptr, debugDescription] }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl<T> INSResponder for NSApplication<T, ()> {}

impl<T> INSApplication for NSApplication<T, ()> {
    fn tp_sharedApplication() -> Self {
        unsafe {
            NSApplication {
                ptr: Id::from_ptr(msg_send![register_app_class(), sharedApplication]),
                ptr_delegate: Id::from_ptr(nil),
                delegate: None,
                pool: NSAutoreleasePool::new(),
                _message: PhantomData,
            }
        }
    }

    fn im_terminate(self, sender: id) {
        unsafe { msg_send![self.ptr, terminate: sender] }
    }

    fn replyToApplicationShouldTerminate(self, shouldTerminate: bool) {
        unsafe { msg_send![self.ptr, replyToApplicationShouldTerminate: shouldTerminate] }
    }

    fn im_disableRelaunchOnLogin(&self) {
        unsafe { msg_send![self.ptr, disableRelaunchOnLogin] }
    }

    fn im_enableRelaunchOnLogin(&self) {
        unsafe { msg_send![self.ptr, enableRelaunchOnLogin] }
    }

    fn im_registerForRemoteNotifications(&self) {
        unsafe { msg_send![self.ptr, registerForRemoteNotifications] }
    }

    fn im_unregisterForRemoteNotifications(&self) {
        unsafe { msg_send![self.ptr, unregisterForRemoteNotifications] }
    }

    fn replyToOpenOrPrint(self, response: NSApplicationDelegateReply) {
        unsafe { msg_send![self.ptr, replyToOpenOrPrint: response] }
    }

    fn im_activationPolicy(&self) -> NSApplicationActivationPolicy {
        unsafe { msg_send![self.ptr, activationPolicy] }
    }

    fn im_setActivationPolicy(&self, policy: NSApplicationActivationPolicy) {
        unsafe { msg_send![self.ptr, setActivationPolicy: policy] }
    }

    fn ip_mainMenu(&self) -> NSMenu {
        unsafe { NSMenu::from_id(msg_send![self.ptr, mainMenu]) }
    }

    fn ip_setMainMenu(&self, menu: NSMenu) {
        unsafe { msg_send![self.ptr, setMainMenu: menu] }
    }
}

impl<T, M> fmt::Debug for NSApplication<T, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let delegate = format!("{:?}", self.delegate);
        f.debug_struct("App")
            .field("objc", &self.ptr)
            .field("objc_delegate", &self.ptr_delegate)
            //.field("delegate", &delegate)
            .field("pool", &self.pool)
            .finish()
    }
}
