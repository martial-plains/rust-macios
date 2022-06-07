#![allow(trivial_casts)]

use std::{collections::HashMap, ffi::CString, sync::RwLock};

use block::ConcreteBlock;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{objc_getClass, Class, Object, Sel},
    sel, sel_impl,
};
use objc_id::ShareId;

use crate::{
    core_graphics::{CGFloat, CGPoint, CGSize},
    foundation::{Int, NSRect, NSString, UInt},
    id,
    objective_c_runtime::nil,
    utils::to_bool,
};

use super::{NSWindowStyleMask, NSWindowTitleVisibility, NSWindowToolbarStyle, PNSWindowDelegate};

pub(crate) static WINDOW_DELEGATE_PTR: &str = "rstNSWindowDelegate";

/// A config for a window.
#[derive(Debug)]
pub struct WindowConfig {
    /// The style the window should have.
    pub style: UInt,

    /// The initial dimensions for the window.
    pub initial_dimensions: NSRect,

    /// From the Apple docs:
    ///
    /// _"When true, the window server defers creating the window device
    /// until the window is moved onscreen. All display messages sent to
    /// the window or its views are postponed until the window is created,
    /// just before it’s moved onscreen."_
    ///
    /// You generally just want this to be true, and it's the default for this struct.
    pub defer: bool,

    /// The style of toolbar that should be set here. This one is admittedly odd to be set here,
    /// but that's how the underlying API is designed, so we're sticking to it.
    ///
    /// This property is not used on macOS versions prior to Big Sur. This defaults to
    /// `ToolbarStyle::Automatic`; consult the specified enum
    /// for other variants.
    ///
    /// This setting is notably important for Preferences windows.
    pub toolbar_style: NSWindowToolbarStyle,
}

impl Default for WindowConfig {
    fn default() -> Self {
        let mut config = WindowConfig {
            style: 0,
            initial_dimensions: NSRect {
                origin: CGPoint { x: 100., y: 100. },
                size: CGSize {
                    width: 1024.,
                    height: 768.,
                },
            },
            defer: true,
            toolbar_style: NSWindowToolbarStyle::Automatic,
        };

        config.set_styles(&[
            NSWindowStyleMask::Resizable,
            NSWindowStyleMask::Miniaturizable,
            NSWindowStyleMask::UnifiedTitleAndToolbar,
            NSWindowStyleMask::Closable,
            NSWindowStyleMask::Titled,
            NSWindowStyleMask::FullSizeContentView,
        ]);

        config
    }
}

impl WindowConfig {
    /// Given a set of styles, converts them to `NSUInteger` and stores them for later use.
    pub fn set_styles(&mut self, styles: &[NSWindowStyleMask]) {
        let mut style: UInt = 0;

        for mask in styles {
            let i = *mask as UInt;
            style = style | i;
        }

        self.style = style;
    }

    /// Set the initial dimensions of this window.
    pub fn set_initial_dimensions(&mut self, top: f64, left: f64, width: f64, height: f64) {
        self.initial_dimensions = NSRect {
            origin: CGPoint { x: left, y: top },
            size: CGSize { width, height },
        };
    }

    /// Offered as a convenience API to match the others. You can just set this property directly
    /// if you prefer.
    ///
    /// Sets the Toolbar style. This is not used prior to macOS Big Sur (11.0); consult the
    /// `ToolbarStyle` enum for more information on possible values.
    pub fn set_toolbar_style(&mut self, style: NSWindowToolbarStyle) {
        self.toolbar_style = style;
    }
}

/// A `Window` represents your way of interacting with an `NSWindow`. It wraps the various moving
/// pieces to enable you to focus on reacting to lifecycle methods and doing your thing.
#[derive(Debug)]
pub struct NSWindow<T = ()> {
    /// Represents an `NS/UIWindow` in the Objective-C runtime.
    pub objc: ShareId<Object>,

    /// A delegate for this window.
    pub delegate: Option<Box<T>>,
}

impl Default for NSWindow {
    /// Returns a default `Window`, with a default `WindowConfig`.
    fn default() -> Self {
        NSWindow::new(WindowConfig::default())
    }
}

impl NSWindow {
    /// Constructs a new `Window`. You can use this instead of the `default()` method if you'd like
    /// to customize the appearance of a `Window`.
    ///
    /// Why the config? Well, certain properties of windows are really not meant to be altered
    /// after we initialize the backing `NSWindow`.
    pub fn new(config: WindowConfig) -> NSWindow {
        let objc = unsafe {
            // This behavior might make sense to keep as default (YES), but I think the majority of
            // apps that would use this toolkit wouldn't be tab-oriented...
            let _: () = msg_send![class!(NSWindow), setAllowsAutomaticWindowTabbing: false];

            let alloc: id = msg_send![class!(NSWindow), alloc];

            // Other types of backing (Retained/NonRetained) are archaic, dating back to the
            // NeXTSTEP era, and are outright deprecated... so we don't allow setting them.
            let buffered: UInt = 2;
            let dimensions: NSRect = config.initial_dimensions.into();
            let window: id = msg_send![alloc, initWithContentRect:dimensions
                styleMask:config.style
                backing:buffered
                defer: config.defer
            ];

            let _: () = msg_send![window, autorelease];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync by releasing the window out from underneath of
            // us.
            let _: () = msg_send![window, setReleasedWhenClosed: false];

            let _: () = msg_send![window, setRestorable: false];

            let toolbar_style = config.toolbar_style;
            let _: () = msg_send![window, setToolbarStyle: toolbar_style];

            ShareId::from_ptr(window)
        };

        NSWindow {
            objc,
            delegate: None,
        }
    }
}

impl<T> NSWindow<T>
where
    T: PNSWindowDelegate + 'static,
{
    /// Constructs a new Window with a `config` and `delegate`. Using a `PNSWindowDelegate` enables
    /// you to respond to window lifecycle events - visibility, movement, and so on. It also
    /// enables easier structure of your codebase, and in a way simulates traditional class based
    /// architectures... just without the subclassing.
    pub fn with(config: WindowConfig, delegate: T) -> Self {
        let class = register_window_class_with_delegate::<T>(&delegate);
        let mut delegate = Box::new(delegate);

        let objc = unsafe {
            // This behavior might make sense to keep as default (YES), but I think the majority of
            // apps that would use this toolkit wouldn't be tab-oriented...
            let _: () = msg_send![class!(NSWindow), setAllowsAutomaticWindowTabbing: false];

            let alloc: id = msg_send![class, alloc];

            // Other types of backing (Retained/NonRetained) are archaic, dating back to the
            // NeXTSTEP era, and are outright deprecated... so we don't allow setting them.
            let buffered: UInt = 2;
            let dimensions: NSRect = config.initial_dimensions.into();
            let window: id = msg_send![alloc, initWithContentRect:dimensions
                styleMask:config.style
                backing:buffered
                defer: config.defer
            ];

            let delegate_ptr: *const T = &*delegate;
            (&mut *window).set_ivar(WINDOW_DELEGATE_PTR, delegate_ptr as usize);

            let _: () = msg_send![window, autorelease];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync by releasing the window out from underneath of
            // us.
            let _: () = msg_send![window, setReleasedWhenClosed: false];

            // We set the window to be its own delegate - this is cleaned up inside `Drop`.
            let _: () = msg_send![window, setDelegate: window];

            let _: () = msg_send![window, setRestorable: false];

            // This doesn't exist prior to Big Sur, but is important to support for Big Sur.
            //
            // Why this isn't a setting on the Toolbar itself I'll never know.
            let toolbar_style = config.toolbar_style;
            let _: () = msg_send![window, setToolbarStyle: toolbar_style];

            ShareId::from_ptr(window)
        };

        {
            (&mut delegate).did_load(NSWindow {
                delegate: None,
                objc: objc.clone(),
            });
        }

        NSWindow {
            objc,
            delegate: Some(delegate),
        }
    }
}

impl<T> NSWindow<T> {
    /// Handles setting the title on the underlying window. Allocates and passes an `NSString` over
    /// to the Objective C runtime.
    pub fn set_title(&self, title: &str) {
        unsafe {
            let title = NSString::from(title);
            let _: () = msg_send![&*self.objc, setTitle: title];
        }
    }

    /// Sets the title visibility for the underlying window.
    pub fn set_title_visibility(&self, visibility: NSWindowTitleVisibility) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitleVisibility: visibility];
        }
    }

    /// Used for configuring whether the window is movable via the background.
    pub fn set_movable_by_background(&self, movable: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setMovableByWindowBackground: movable];
        }
    }

    /// Used for setting whether this titlebar appears transparent.
    pub fn set_titlebar_appears_transparent(&self, transparent: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitlebarAppearsTransparent: transparent];
        }
    }

    /// Used for setting this Window autosave name.
    pub fn set_autosave_name(&self, name: &str) {
        unsafe {
            let autosave = NSString::from(name);
            let _: () = msg_send![&*self.objc, setFrameAutosaveName: autosave];
        }
    }

    /// Sets the content size for this window.
    pub fn set_content_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize {
                width: width.into(),
                height: height.into(),
            };
            let _: () = msg_send![&*self.objc, setContentSize: size];
        }
    }

    /// Sets the minimum size this window can shrink to.
    pub fn set_minimum_content_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize {
                width: width.into(),
                height: height.into(),
            };
            let _: () = msg_send![&*self.objc, setContentMinSize: size];
        }
    }

    /// Sets the maximum size this window can shrink to.
    pub fn set_maximum_content_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize {
                width: width.into(),
                height: height.into(),
            };

            let _: () = msg_send![&*self.objc, setContentMaxSize: size];
        }
    }

    /// Sets the minimum size this window can shrink to.
    pub fn set_minimum_size<F: Into<f64>>(&self, width: F, height: F) {
        unsafe {
            let size = CGSize {
                width: width.into(),
                height: height.into(),
            };

            let _: () = msg_send![&*self.objc, setMinSize: size];
        }
    }

    // /// Used for setting a toolbar on this window.
    // pub fn set_toolbar<TC: ToolbarDelegate>(&self, toolbar: &Toolbar<TC>) {
    //     unsafe {
    //         let _: () = msg_send![&*self.objc, setToolbar:&*toolbar.objc];
    //     }
    // }

    /// Toggles whether the toolbar is shown for this window. Has no effect if no toolbar exists on
    /// this window.
    pub fn toggle_toolbar_shown(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, toggleToolbarShown: nil];
        }
    }

    /// Set whether the toolbar toggle button is shown. Has no effect if no toolbar exists on this
    /// window.
    pub fn set_shows_toolbar_button(&self, shows: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setShowsToolbarButton: shows];
        }
    }

    // /// Given a view, sets it as the content view for this window.
    // pub fn set_content_view<L: Layout + 'static>(&self, view: &L) {
    //     view.with_backing_node(|backing_node| unsafe {
    //         let _: () = msg_send![&*self.objc, setContentView:&*backing_node];
    //     });
    // }

    // /// Given a view, sets it as the content view controller for this window.
    // pub fn set_content_view_controller<VC: Controller + 'static>(&self, controller: &VC) {
    //     let backing_node = controller.get_backing_node();

    //     unsafe {
    //         let _: () = msg_send![&*self.objc, setContentViewController:&*backing_node];
    //     }
    // }

    /// Shows the window.
    pub fn show(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, makeKeyAndOrderFront: nil];
        }
    }

    /// On macOS, calling `close()` is equivalent to calling... well, `close`. It closes the
    /// window.
    ///
    /// I dunno what else to say here, lol.
    pub fn close(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, close];
        }
    }

    /// Toggles a Window being full screen or not.
    pub fn toggle_full_screen(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, toggleFullScreen: nil];
        }
    }

    // /// Sets the background color for the window. You generally don't want to do this often.
    // pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
    //     let color: id = color.as_ref().into();

    //     unsafe {
    //         let _: () = msg_send![&*self.objc, setBackgroundColor: color];
    //     }
    // }

    /// Returns whether this window is opaque or not.
    pub fn is_opaque(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, isOpaque] })
    }

    /// Returns whether this window is miniaturized or not.
    pub fn is_miniaturized(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, isMiniaturized] })
    }

    /// Miniaturize this window.
    pub fn miniaturize(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, miniaturize];
        }
    }

    /// De-mimizes this window.
    pub fn deminiaturize(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, deminiaturize];
        }
    }

    /// Runs the print panel, and if the user does anything except cancel, prints the window and
    /// its contents.
    pub fn print(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, print];
        }
    }

    /// Indicates whether the window is on a currently active space.
    ///
    /// From Apple's documentation:
    ///
    /// _The value of this property is YES if the window is on the currently active space; otherwise, NO.
    /// For visible windows, this property indicates whether the window is currently visible on the active
    /// space. For nonvisible windows, it indicates whether ordering the window onscreen would cause it to
    /// be on the active space._
    pub fn is_on_active_space(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, isOnActiveSpace] })
    }

    /// Returns whether this window is visible or not.
    pub fn is_visible(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, isVisible] })
    }

    /// Returns whether this window is the key or not.
    pub fn is_key(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, isKeyWindow] })
    }

    /// Returns whether this window can become the key window.
    pub fn can_become_key(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, canBecomeKeyWindow] })
    }

    /// Make this window the key window.
    pub fn make_key_window(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, makeKeyWindow];
        }
    }

    /// Make the this window the key window and bring it to the front. Calling `show` does this for
    /// you.
    pub fn make_key_and_order_front(&self) {
        unsafe {
            let _: () = msg_send![&*self.objc, makeKeyAndOrderFront: nil];
        }
    }

    /// Returns if this is the main window or not.
    pub fn is_main_window(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, isMainWindow] })
    }

    /// Returns if this can become the main window.
    pub fn can_become_main_window(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.objc, canBecomeMainWindow] })
    }

    /// Set whether this window should be excluded from the top-level "Windows" menu.
    pub fn set_excluded_from_windows_menu(&self, excluded: bool) {
        unsafe {
            let _: () = msg_send![&*self.objc, setExcludedFromWindowsMenu: excluded];
        }
    }

    /// Sets the type of separator that the app displays between the title bar and content of a window.
    pub fn set_titlebar_separator_style(&self, style: Int) {
        unsafe {
            let _: () = msg_send![&*self.objc, setTitlebarSeparatorStyle: style];
        }
    }

    /// Returns the backing scale (e.g, `1.0` for non retina, `2.0` for retina) used on this
    /// window.
    ///
    /// Note that Apple recommends AGAINST using this in most cases. It's exposed here for the rare
    /// cases where you DO need it.
    pub fn backing_scale_factor(&self) -> f64 {
        unsafe {
            let scale: CGFloat = msg_send![&*self.objc, backingScaleFactor];
            scale
        }
    }

    /// Given a window and callback handler, will run it as a "sheet" (model-ish) and then run the
    /// handler once the sheet is dismissed.
    ///
    /// This is a bit awkward due to Rust semantics; you have to use the same type of Window as the
    /// one you're presenting on, but in practice this isn't too bad since you rarely want a Window
    /// without a PNSWindowDelegate.
    pub fn begin_sheet<F, W>(&self, window: &NSWindow<W>, completion: F)
    where
        F: Fn() + Send + Sync + 'static,
        W: PNSWindowDelegate + 'static,
    {
        let block = ConcreteBlock::new(move |_response: Int| {
            completion();
        });
        let block = block.copy();

        unsafe {
            let _: () = msg_send![&*self.objc, beginSheet:&*window.objc completionHandler:block];
        }
    }

    /// Closes a sheet.
    pub fn end_sheet<W>(&self, window: &NSWindow<W>)
    where
        W: PNSWindowDelegate + 'static,
    {
        unsafe {
            let _: () = msg_send![&*self.objc, endSheet:&*window.objc];
        }
    }
}

impl<T> Drop for NSWindow<T> {
    /// When a Window is dropped on the Rust side, we want to ensure that we break the delegate
    /// link on the Objective-C side. While this shouldn't actually be an issue, I'd rather be
    /// safer than sorry.
    ///
    /// Note that only the originating `Window<T>` carries the delegate, and we
    /// intentionally don't provide this when cloning it as a handler. This ensures that we only
    /// release the backing Window when the original `Window<T>` is dropped.
    ///
    /// Well, theoretically.
    fn drop(&mut self) {
        if self.delegate.is_some() {
            unsafe {
                // Break the delegate - this shouldn't be an issue, but we should strive to be safe
                // here anyway.
                let _: () = msg_send![&*self.objc, setDelegate: nil];
            }
        }
    }
}

fn load<'a, T>(this: &'a Object, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}

/// Called when an `NSWindowDelegate` receives a `windowWillClose:` event.
/// Good place to clean up memory and what not.
extern "C" fn should_close<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) -> bool {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);

    window.should_close()
}

/// Called when an `NSWindowDelegate` receives a `windowWillClose:` event.
/// Good place to clean up memory and what not.
extern "C" fn will_close<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_close();
}

/// Called when an `NSWindowDelegate` receives a `windowWillMove:` event.
extern "C" fn will_move<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_move();
}

/// Called when an `NSWindowDelegate` receives a `windowDidMove:` event.
extern "C" fn did_move<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_move();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn did_change_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern "C" fn did_change_screen_profile<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_screen_profile();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn will_resize<T: PNSWindowDelegate>(
    this: &Object,
    _: Sel,
    _: id,
    size: CGSize,
) -> CGSize {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    let s = window.will_resize(size.width, size.height);

    CGSize {
        width: s.0,
        height: s.1,
    }
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn did_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_resize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn will_start_live_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_start_live_resize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn did_end_live_resize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_end_live_resize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn will_miniaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_miniaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn did_miniaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_miniaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreen:` event.
extern "C" fn did_deminiaturize<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_deminiaturize();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern "C" fn will_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_enter_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern "C" fn did_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_enter_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
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

// /// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
// extern "C" fn options_for_full_screen<T: PNSWindowDelegate>(
//     this: &Object,
//     _: Sel,
//     _: id,
//     options: UInt,
// ) -> UInt {
//     let window = load::<T>(this, WINDOW_DELEGATE_PTR);

//     let desired_opts = window.presentation_options_for_full_screen();

//     if desired_opts.is_none() {
//         options
//     } else {
//         let mut opts: UInt = 0;
//         for opt in desired_opts.unwrap() {
//             opts = opts << UInt::from(opt);
//         }

//         opts
//     }
// }

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern "C" fn will_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.will_exit_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern "C" fn did_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_exit_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern "C" fn did_fail_to_enter_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_fail_to_enter_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeScreenProfile:` event.
extern "C" fn did_fail_to_exit_full_screen<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_fail_to_exit_full_screen();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeBackingProperties:` event.
extern "C" fn did_change_backing_properties<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_backing_properties();
}

/// Called when an `NSWindowDelegate` receives a `windowDidChangeBackingProperties:` event.
extern "C" fn did_change_occlusion_state<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_change_occlusion_state();
}

/// Called when an `NSWindowDelegate` receives a `windowDidUpdate:` event.
extern "C" fn did_update<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_update();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern "C" fn did_become_main<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_become_main();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern "C" fn did_resign_main<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_resign_main();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern "C" fn did_become_key<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_become_key();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern "C" fn did_resign_key<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_resign_key();
}

/// Called when an `NSWindowDelegate` receives a `windowDidExpose:` event.
extern "C" fn did_expose<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.did_expose();
}

/// Called as part of the responder chain, when, say, the ESC key is hit. If your
/// delegate returns `true` in `should_cancel_on_esc`, then this will allow your
/// window to close when the Esc key is hit. This is mostly useful for Sheet-presented
/// windows, and so the default response from delegates is `false` and must be opted in to.
extern "C" fn cancel<T: PNSWindowDelegate>(this: &Object, _: Sel, _: id) {
    let window = load::<T>(this, WINDOW_DELEGATE_PTR);
    window.cancel();
}

lazy_static::lazy_static! {
    static ref CLASSES: ClassMap = ClassMap::new();
}

struct ClassMap(RwLock<HashMap<&'static str, HashMap<&'static str, usize>>>);

impl ClassMap {
    /// Returns a new ClassMap.
    pub fn new() -> Self {
        ClassMap(RwLock::new({
            let mut map = HashMap::new();

            // Top-level classes, like `NSView`, we cache here. The reasoning is that if a subclass
            // is being created, we can avoid querying the runtime for the superclass - i.e, many
            // subclasses will have `NSView` as their superclass.
            let _ = map.insert("_supers", HashMap::new());

            map
        }))
    }

    /// Attempts to load a previously registered subclass.
    pub fn load_subclass(
        &self,
        subclass_name: &'static str,
        superclass_name: &'static str,
    ) -> Option<*const Class> {
        let reader = self.0.read().unwrap();

        if let Some(inner) = (*reader).get(subclass_name) {
            if let Some(class) = inner.get(superclass_name) {
                return Some(*class as *const Class);
            }
        }

        None
    }

    /// Store a newly created subclass type.
    pub fn store_subclass(
        &self,
        subclass_name: &'static str,
        superclass_name: &'static str,
        class: *const Class,
    ) {
        let mut writer = self.0.write().unwrap();

        if let Some(map) = (*writer).get_mut(subclass_name) {
            let _ = map.insert(superclass_name, class as usize);
        } else {
            let mut map = HashMap::new();
            let _ = map.insert(superclass_name, class as usize);
            let _ = (*writer).insert(subclass_name, map);
        }
    }

    /// Attempts to load a Superclass. This first checks for the cached pointer; if not present, it
    /// will load the superclass from the Objective-C runtime and cache it for future lookup. This
    /// assumes that the class is one that should *already* and *always* exist in the runtime, and
    /// by design will panic if it can't load the correct superclass, as that would lead to very
    /// invalid behavior.
    pub fn load_superclass(&self, name: &'static str) -> Option<*const Class> {
        {
            let reader = self.0.read().unwrap();
            if let Some(superclass) = (*reader)["_supers"].get(name) {
                return Some(*superclass as *const Class);
            }
        }

        let objc_superclass_name = CString::new(name).unwrap();
        let superclass = unsafe { objc_getClass(objc_superclass_name.as_ptr() as *const _) };

        // This should not happen, for our use-cases, but it's conceivable that this could actually
        // be expected, so just return None and let the caller panic if so desired.
        if superclass.is_null() {
            return None;
        }

        {
            let mut writer = self.0.write().unwrap();
            if let Some(supers) = (*writer).get_mut("_supers") {
                let _ = supers.insert(name, superclass as usize);
            }
        }

        Some(superclass)
    }
}

#[inline(always)]
fn load_or_register_class<F>(
    superclass_name: &'static str,
    subclass_name: &'static str,
    config: F,
) -> *const Class
where
    F: Fn(&mut ClassDecl) + 'static,
{
    if let Some(subclass) = CLASSES.load_subclass(subclass_name, superclass_name) {
        return subclass;
    }

    if let Some(superclass) = CLASSES.load_superclass(superclass_name) {
        let objc_subclass_name = format!("{}_{}", subclass_name, superclass_name);

        match ClassDecl::new(&objc_subclass_name, unsafe { &*superclass }) {
            Some(mut decl) => {
                config(&mut decl);

                let class = decl.register();
                CLASSES.store_subclass(subclass_name, superclass_name, class);
                return class;
            }

            None => {
                panic!(
                    "Subclass of type {}_{} could not be allocated.",
                    subclass_name, superclass_name
                );
            }
        }
    }

    panic!(
        "Attempted to create subclass for {}, but unable to load superclass of type {}.",
        subclass_name, superclass_name
    );
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
        // decl.add_method(
        //     sel!(window:willUseFullScreenPresentationOptions:),
        //     options_for_full_screen::<T> as extern "C" fn(&Object, _, _, UInt) -> UInt,
        // );
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
