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
    foundation::{NSArray, NSAutoreleasePool},
    objective_c_runtime::{
        id, nil,
        traits::{PNSObject, ToId},
    },
};

use super::{
    ns_application_delegate::register_app_delegate_class,
    ns_menu::NSMenu,
    ns_menu_item::NSMenuItem,
    traits::{INSApplication, INSResponder, PNSApplicationDelegate},
    NSApplicationActivationOptions, NSApplicationActivationPolicy, NSApplicationDelegateReply,
    NSRunningApplication,
};

pub(crate) static NSAPPLICATION_PTR: &str = "rstNSApplicationPtr";

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
        shared_application(|app| app.im_register_for_remote_notifications());
    }

    /// Unregisters for remote notifications from APNS.
    pub fn unregister_for_remote_notifications() {
        shared_application(|app| {
            app.im_unregister_for_remote_notifications();
        });
    }

    /// Sets whether this application should relaunch at login.
    pub fn set_relaunch_on_login(relaunch: bool) {
        shared_application(|app| {
            if relaunch {
                app.im_enable_relaunch_on_login();
            } else {
                app.im_disable_relaunch_on_login();
            }
        });
    }

    /// Respond to a termination request. Generally done after returning `TerminateResponse::Later`
    /// from your trait implementation of `should_terminate()`.
    pub fn reply_to_termination_request(should_terminate: bool) {
        shared_application(|app| {
            app.im_reply_to_application_should_terminate(should_terminate);
        });
    }

    /// An optional call that you can use for certain scenarios surrounding opening/printing files.
    pub fn reply_to_open_or_print(response: NSApplicationDelegateReply) {
        shared_application(|app| app.im_reply_to_open_or_print(response));
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

        shared_application(move |app| app.ip_set_main_menu(main_menu.clone()));
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
        app.im_set_activation_policy(NSApplicationActivationPolicy::Regular);
        let mut current_app = NSRunningApplication::current_application();
        current_app.activate_with_options(NSApplicationActivationOptions::IgnoringOtherWindows);
    }
}

impl<T> NSApplication<T> {
    /// Starts the main event loop.
    pub fn run(&mut self) {
        self.im_run();
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
            (*delegate).set_ivar(NSAPPLICATION_PTR, delegate_ptr as usize);

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

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl<T> INSResponder for NSApplication<T, ()> {}

impl<T> INSApplication for NSApplication<T, ()> {
    fn tp_shared_application() -> Self {
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
}

impl<T> fmt::Debug for NSApplication<T, ()> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl ToId for NSApplication {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}
