#![allow(trivial_casts)]

use objc::{
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};

use crate::{core_graphics::CGSize, objective_c_runtime::id, utils::load_or_register_class};

use super::{WINDOW_DELEGATE_PTR, traits::PNSWindowDelegate};

fn load<'a, T>(this: &'a Object, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}

extern "C" fn should_close<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) -> bool {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    window.im_should_close()
}

extern "C" fn will_close<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_will_close();
}

extern "C" fn will_move<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_will_move();
}

extern "C" fn did_move<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_move();
}

extern "C" fn did_change_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_change_screen();
}

extern "C" fn did_change_screen_profile<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_change_screen_profile();
}

extern "C" fn will_resize<T: PNSWindowDelegate>(
    this: &Object,
    _: Sel,
    _: id,
    size: CGSize,
) -> CGSize {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_will_resize_to_size(size)
}

extern "C" fn did_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_resize();
}

extern "C" fn will_start_live_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_will_start_live_resize();
}

extern "C" fn did_end_live_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_end_live_resize();
}

extern "C" fn will_miniaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_will_miniaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn did_miniaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_miniaturize();
}

extern "C" fn did_deminiaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_deminiaturize();
}

extern "C" fn will_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_will_enter_full_screen();
}

extern "C" fn did_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_enter_full_screen();
}

extern "C" fn content_size_for_full_screen<T: PNSWindowDelegate>(
    this: &Object,
    _: Sel,
    _: id,
    size: CGSize,
) -> CGSize {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    let (width, height) = window.content_size_for_full_screen(size.width, size.height);

    CGSize { width, height }
}

extern "C" fn will_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_will_exit_full_screen();
}

extern "C" fn did_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_exit_full_screen();
}

extern "C" fn did_fail_to_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_fail_to_enter_full_screen();
}

extern "C" fn did_fail_to_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_fail_to_exit_full_screen();
}

extern "C" fn did_change_backing_properties<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_change_backing_properties();
}

extern "C" fn did_change_occlusion_state<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_change_occlusion_state();
}

extern "C" fn did_update<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_update();
}

extern "C" fn did_become_main<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_become_main();
}

extern "C" fn did_resign_main<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_resign_main();
}

extern "C" fn did_become_key<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_become_key();
}

extern "C" fn did_resign_key<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_resign_key();
}

extern "C" fn did_expose<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.im_did_expose();
}

extern "C" fn cancel<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.cancel();
}

/// Injects an `NSWindowDelegate` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_window_class_with_delegate<T: PNSWindowDelegate>(
    instance: &T,
) -> *const Class {
    load_or_register_class("NSWindow", instance.subclass_name(), |decl| unsafe {
        decl.add_ivar::<usize>(WINDOW_DELEGATE_PTR);

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
    })
}
