#![allow(trivial_casts)]

use std::sync::Once;

use objc::{class, declare::ClassDecl};

use crate::{
    core_graphics::CGSize,
    foundation::{NSRect, NSSize},
    objective_c_runtime::{
        id,
        runtime::{Class, Object, Sel},
        sel, sel_impl,
    },
};

use super::{NSWindow, NSWINDOW_DELEGATE_PTR};

/// A set of optional methods that a window’s delegate can implement to respond to events, such as window resizing, moving, exposing, and minimizing.
pub trait PNSWindowDelegate {
    /// Used to cache subclass creations on the Objective-C side.
    /// You can just set this to be the name of your view type. This
    /// value *must* be unique per-type.
    const NAME: &'static str;

    /// You should rarely (read: probably never) need to implement this yourself.
    /// It simply acts as a getter for the associated `NAME` const on this trait.
    fn subclass_name(&self) -> &'static str {
        Self::NAME
    }

    /* Managing Sheets
     */

    /// Tells the delegate that the window is about to show a sheet at the
    /// specified location, giving it the opportunity to return a custom
    /// location for the attachment of the sheet to the window.
    ///
    /// # Arguments
    ///
    /// * `window` - The window containing the sheet to be animated.
    /// * `sheet` - The sheet to be shown.
    /// * `rect` - The default sheet location, just under the title bar of the window, aligned with the left and right edges of the window.
    ///
    /// # Returns
    ///
    /// The new sheet location.
    fn im_window_will_position_sheet_using_rect(
        _window: NSWindow,
        _sheet: NSWindow,
        _rect: NSRect,
    ) -> NSRect {
        NSRect::default()
    }

    /// Notifies the delegate that the window is about to open a sheet.
    ///
    /// # Arguments
    ///
    /// * `notification` - A notification named `NSWindowWillBeginSheetNotification`.
    fn im_will_begin_sheet(&self) {}

    /// Tells the delegate that the window has closed a sheet.
    ///
    /// # Arguments
    ///
    /// * `notification` - A notification named `NSWindowDidEndSheetNotification`.
    fn im_did_end_sheet(&self) {}

    /* Sizing Windows
     */

    /// Tells the delegate that the window is being resized (whether by the user or through one of the setFrame
    ///
    /// # Arguments
    ///
    /// * `width` - The new width of the window.
    fn im_will_resize_to_size(&self, frame_size: NSSize) -> NSSize {
        frame_size
    }

    /// Tells the delegate that the window has been resized.
    fn im_did_resize(&self) {}

    /// Tells the delegate that the window is about to be live resized.
    fn im_will_start_live_resize(&self) {}

    /// Tells the delegate that a live resize operation on the window has ended.
    fn im_did_end_live_resize(&self) {}

    /* Minimizing Windows
     */

    /// Tells the delegate that the window is about to be minimized.
    fn im_will_miniaturize(&self) {}

    /// Tells the delegate that the window has been minimized.
    fn im_did_miniaturize(&self) {}

    /// Tells the delegate that the window has been deminimized.
    fn im_did_deminiaturize(&self) {}

    /* Managing Full-Screen Presentation
     */

    /// The window is about to enter full-screen mode.
    fn im_will_enter_full_screen(&self) {}

    /// The window has entered full-screen mode.
    fn im_did_enter_full_screen(&self) {}

    /// The window is about to exit full-screen mode.
    fn im_will_exit_full_screen(&self) {}

    /// The window has left full-screen mode.
    fn im_did_exit_full_screen(&self) {}

    /* Custom Full-Screen Presentation Animations
     */

    /// Called if the window failed to enter full-screen mode.
    fn im_did_fail_to_enter_full_screen(&self) {}

    /// Called if the window failed to exit full-screen mode.
    fn im_did_fail_to_exit_full_screen(&self) {}

    /* Moving Windows
     */

    /// Tells the delegate that the window is about to move.
    fn im_will_move(&self) {}

    /// Tells the delegate that the window has moved.
    fn im_did_move(&self) {}

    /// Tells the delegate that the window has changed screens.
    fn im_did_change_screen(&self) {}

    /// Tells the delegate that the window has changed screen display profiles.
    fn im_did_change_screen_profile(&self) {}

    /// Tells the delegate that the window backing properties changed.
    fn im_did_change_backing_properties(&self) {}

    /* Closing Windows
     */

    /// Tells the delegate that the user has attempted to close a window or the window has received a `perform_close` message.
    fn im_should_close(&self) -> bool {
        true
    }

    /// Tells the delegate that the window is about to close.
    fn im_will_close(&self) {}

    /* Managing Key Status
     */

    /// Tells the delegate that the window has become the key window.
    fn im_did_become_key(&self) {}

    /// Tells the delegate that the window has resigned key window status.
    fn im_did_resign_key(&self) {}

    /* Managing Main Status
     */

    /// Tells the delegate that the window has become main.
    fn im_did_become_main(&self) {}

    /// Tells the delegate that the window has resigned main window status.
    fn im_did_resign_main(&self) {}

    /* Updating Windows
     */

    /// Tells the delegate that the window received an `update` message.
    fn im_did_update(&self) {}

    /* Exposing Windows
     */

    /// Tells the delegate that the window has been exposed.
    fn im_did_expose(&self) {}

    /* Managing Occlusion State
     */

    /// Tells the delegate that the window changed its occlusion state.
    fn im_did_change_occlusion_state(&self) {}

    /// Fires when this window has loaded in memory, and is about to display. This is a good point
    /// to set up your views and what not.
    ///
    /// If you're coming from the web, you can think of this as `DOMContentLoaded`.
    fn did_load(&mut self, _window: NSWindow) {}

    /// Fires when the system is moving a window to full screen and wants to know what content size
    /// to use. By default, this just returns the system-provided content size, but you can
    /// override it if need be.
    fn content_size_for_full_screen(
        &self,
        proposed_width: f64,
        proposed_height: f64,
    ) -> (f64, f64) {
        (proposed_width, proposed_height)
    }

    /// If you want your window to close when the `ESC` key is hit, implement this.
    /// This is mostly useful for windows that present as modal sheets.
    fn cancel(&self) {}
}

fn load<'a, T>(this: &'a Object, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}

extern "C" fn should_close<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) -> bool {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);

    window.im_should_close()
}

extern "C" fn will_close<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_will_close();
}

extern "C" fn will_move<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_will_move();
}

extern "C" fn did_move<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_move();
}

extern "C" fn did_change_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_change_screen();
}

extern "C" fn did_change_screen_profile<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_change_screen_profile();
}

extern "C" fn will_resize<T: PNSWindowDelegate>(
    this: &Object,
    _: Sel,
    _: id,
    size: CGSize,
) -> CGSize {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_will_resize_to_size(size)
}

extern "C" fn did_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_resize();
}

extern "C" fn will_start_live_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_will_start_live_resize();
}

extern "C" fn did_end_live_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_end_live_resize();
}

extern "C" fn will_miniaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_will_miniaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn did_miniaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_miniaturize();
}

extern "C" fn did_deminiaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_deminiaturize();
}

extern "C" fn will_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_will_enter_full_screen();
}

extern "C" fn did_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_enter_full_screen();
}

extern "C" fn content_size_for_full_screen<T: PNSWindowDelegate>(
    this: &Object,
    _: Sel,
    _: id,
    size: CGSize,
) -> CGSize {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);

    let (width, height) = window.content_size_for_full_screen(size.width, size.height);

    CGSize { width, height }
}

extern "C" fn will_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_will_exit_full_screen();
}

extern "C" fn did_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_exit_full_screen();
}

extern "C" fn did_fail_to_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_fail_to_enter_full_screen();
}

extern "C" fn did_fail_to_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_fail_to_exit_full_screen();
}

extern "C" fn did_change_backing_properties<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_change_backing_properties();
}

extern "C" fn did_change_occlusion_state<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_change_occlusion_state();
}

extern "C" fn did_update<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_update();
}

extern "C" fn did_become_main<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_become_main();
}

extern "C" fn did_resign_main<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_resign_main();
}

extern "C" fn did_become_key<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_become_key();
}

extern "C" fn did_resign_key<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_resign_key();
}

extern "C" fn did_expose<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.im_did_expose();
}

extern "C" fn cancel<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, NSWINDOW_DELEGATE_PTR);
    window.cancel();
}

/// Injects an `NSWindowDelegate` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_window_class_with_delegate<T: PNSWindowDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSWindow);
        let mut decl = ClassDecl::new("RSTNSWindowDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(NSWINDOW_DELEGATE_PTR);

        // NSWindowDelegate methods
        decl.add_method(
            sel!(windowShouldClose:),
            should_close::<T> as extern "C" fn(&Object, _, _) -> bool,
        );
        decl.add_method(
            sel!(windowWillClose:),
            will_close::<T> as extern "C" fn(&Object, _, _),
        );

        // Sizing
        decl.add_method(
            sel!(windowWillResize:toSize:),
            will_resize::<T> as extern "C" fn(&Object, _, _, CGSize) -> CGSize,
        );
        decl.add_method(
            sel!(windowDidResize:),
            did_resize::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowWillStartLiveResize:),
            will_start_live_resize::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidEndLiveResize:),
            did_end_live_resize::<T> as extern "C" fn(&Object, _, _),
        );

        // Minimizing
        decl.add_method(
            sel!(windowWillMiniaturize:),
            will_miniaturize::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidMiniaturize:),
            did_miniaturize::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidDeminiaturize:),
            did_deminiaturize::<T> as extern "C" fn(&Object, _, _),
        );

        // Full Screen
        decl.add_method(
            sel!(window:willUseFullScreenContentSize:),
            content_size_for_full_screen::<T> as extern "C" fn(&Object, _, _, CGSize) -> CGSize,
        );

        decl.add_method(
            sel!(windowWillEnterFullScreen:),
            will_enter_full_screen::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidEnterFullScreen:),
            did_enter_full_screen::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowWillExitFullScreen:),
            will_exit_full_screen::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidExitFullScreen:),
            did_exit_full_screen::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidFailToEnterFullScreen:),
            did_fail_to_enter_full_screen::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidFailToExitFullScreen:),
            did_fail_to_exit_full_screen::<T> as extern "C" fn(&Object, _, _),
        );

        // Key status
        decl.add_method(
            sel!(windowDidBecomeKey:),
            did_become_key::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidResignKey:),
            did_resign_key::<T> as extern "C" fn(&Object, _, _),
        );

        // Main status
        decl.add_method(
            sel!(windowDidBecomeMain:),
            did_become_main::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidResignMain:),
            did_resign_main::<T> as extern "C" fn(&Object, _, _),
        );

        // Moving Windows
        decl.add_method(
            sel!(windowWillMove:),
            will_move::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidMove:),
            did_move::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidChangeScreen:),
            did_change_screen::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidChangeScreenProfile:),
            did_change_screen_profile::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidChangeBackingProperties:),
            did_change_backing_properties::<T> as extern "C" fn(&Object, _, _),
        );

        // Random
        decl.add_method(
            sel!(windowDidChangeOcclusionState:),
            did_change_occlusion_state::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidExpose:),
            did_expose::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(windowDidUpdate:),
            did_update::<T> as extern "C" fn(&Object, _, _),
        );
        decl.add_method(
            sel!(cancelOperation:),
            cancel::<T> as extern "C" fn(&Object, _, _),
        );
    });

    unsafe { DELEGATE_CLASS }
}
