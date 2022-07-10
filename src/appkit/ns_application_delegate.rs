#![allow(trivial_casts)]

use std::sync::Once;

use objc::{
    class,
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};

use crate::objective_c_runtime::id;

use super::{
    traits::PNSApplicationDelegate, NSApplicationTerminateReply, NSMenu, NSAPPLICATION_PTR,
};

/// A handy method for grabbing our `NSApplicationDelegate` from the pointer. This is different from our
/// standard `utils` version as this doesn't require `RefCell` backing.
fn app<T>(this: &mut Object) -> &mut T {
    unsafe {
        let app_ptr: usize = *this.get_ivar(NSAPPLICATION_PTR);
        let app = app_ptr as *mut T;
        &mut *app
    }
}

/// Fires when the Application Delegate receives a `applicationWillFinishLaunching` notification.
extern "C" fn will_finish_launching<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).will_finish_launching();
}

/// Fires when the Application Delegate receives a `applicationDidFinishLaunching` notification.
extern "C" fn did_finish_launching<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).did_finish_launching();
}

/// Fires when the Application Delegate receives a `applicationWillBecomeActive` notification.
extern "C" fn will_become_active<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).will_become_active();
}

/// Fires when the Application Delegate receives a `applicationDidBecomeActive` notification.
extern "C" fn did_become_active<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).did_become_active();
}

/// Fires when the Application Delegate receives a `applicationWillResignActive` notification.
extern "C" fn will_resign_active<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).will_resign_active();
}

/// Fires when the Application Delegate receives a `applicationDidResignActive` notification.
extern "C" fn did_resign_active<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).did_resign_active();
}

/// Fires when the Application Delegate receives a 'applicationShouldTerminate:` notification.
extern "C" fn should_terminate<T: PNSApplicationDelegate>(
    this: &mut Object,
    _: Sel,
    _: id,
) -> NSApplicationTerminateReply {
    app::<T>(this).should_terminate()
}

/// Fires when the Application Delegate receives a `applicationWillTerminate:` notification.
extern "C" fn will_terminate<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).will_terminate();
}

/// Fires when the Application Delegate receives a `applicationWillHide:` notification.
extern "C" fn will_hide<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).will_hide();
}

/// Fires when the Application Delegate receives a `applicationDidHide:` notification.
extern "C" fn did_hide<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).did_hide();
}

/// Fires when the Application Delegate receives a `applicationWillUnhide:` notification.
extern "C" fn will_unhide<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).will_unhide();
}

/// Fires when the Application Delegate receives a `applicationDidUnhide:` notification.
extern "C" fn did_unhide<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).did_unhide();
}

/// Fires when the Application Delegate receives a `applicationWillUpdate:` notification.
extern "C" fn will_update<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).will_update();
}

/// Fires when the Application Delegate receives a `applicationDidUpdate:` notification.
extern "C" fn did_update<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) {
    app::<T>(this).did_update();
}

extern "C" fn should_terminate_after_last_window_closed<T>(this: &mut Object, _: Sel, _: id) -> bool
where
    T: PNSApplicationDelegate,
{
    app::<T>(this).should_terminate_after_last_window_closed()
}

/// Fires when the Application Delegate receives a
/// `applicationShouldHandleReopen:hasVisibleWindows:` notification.
extern "C" fn should_handle_reopen<T: PNSApplicationDelegate>(
    this: &mut Object,
    _: Sel,
    _: id,
    has_visible_windows: bool,
) -> bool {
    app::<T>(this).should_handle_reopen(has_visible_windows)
}

/// Fires when the application delegate receives a `applicationDockMenu:` request.
#[allow(improper_ctypes_definitions)]
extern "C" fn dock_menu<T: PNSApplicationDelegate>(this: &mut Object, _: Sel, _: id) -> NSMenu {
    app::<T>(this).dock_menu().unwrap_or_default()
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
pub fn register_app_delegate_class<T: PNSApplicationDelegate + PNSApplicationDelegate>(
) -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSTNSApplicationDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(NSAPPLICATION_PTR);

        // Launching Applications
        decl.add_method(
            sel!(applicationWillFinishLaunching:),
            will_finish_launching::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching::<T> as extern "C" fn(&mut Object, _, _),
        );

        // Managing Active Status
        decl.add_method(
            sel!(applicationWillBecomeActive:),
            will_become_active::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidBecomeActive:),
            did_become_active::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationWillResignActive:),
            will_resign_active::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidResignActive:),
            did_resign_active::<T> as extern "C" fn(&mut Object, _, _),
        );

        // Terminating Applications
        decl.add_method(
            sel!(applicationShouldTerminate:),
            should_terminate::<T>
                as extern "C" fn(&mut Object, _, _) -> NSApplicationTerminateReply,
        );
        decl.add_method(
            sel!(applicationWillTerminate:),
            will_terminate::<T> as extern "C" fn(&mut Object, _, _),
        );

        // Hiding Applications
        decl.add_method(
            sel!(applicationWillHide:),
            will_hide::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidHide:),
            did_hide::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationWillUnhide:),
            will_unhide::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidUnhide:),
            did_unhide::<T> as extern "C" fn(&mut Object, _, _),
        );

        // Managing Windows
        decl.add_method(
            sel!(applicationWillUpdate:),
            will_update::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationDidUpdate:),
            did_update::<T> as extern "C" fn(&mut Object, _, _),
        );
        decl.add_method(
            sel!(applicationShouldHandleReopen:hasVisibleWindows:),
            should_handle_reopen::<T> as extern "C" fn(&mut Object, _, _, bool) -> bool,
        );

        // Dock Menu
        decl.add_method(
            sel!(applicationDockMenu:),
            dock_menu::<T> as extern "C" fn(&mut Object, _, _) -> NSMenu,
        );

        decl.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            should_terminate_after_last_window_closed::<T>
                as extern "C" fn(&mut Object, _, _) -> bool,
        );

        DELEGATE_CLASS = decl.register();
    });

    unsafe { DELEGATE_CLASS }
}
