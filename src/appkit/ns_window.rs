use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::ShareId;

use crate::{
    core_graphics::{CGPoint, CGSize},
    foundation::{NSRect, NSSize, NSString, UInt},
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject, ToId},
    },
};

use super::{
    ns_window_delegate::register_window_class_with_delegate,
    traits::{INSResponder, INSWindow, PNSWindowDelegate},
    NSBackingStoreType, NSModalResponse, NSTitlebarSeparatorStyle, NSWindowStyleMask,
    NSWindowTitleVisibility, NSWindowToolbarStyle,
};

pub(crate) static WINDOW_DELEGATE_PTR: &str = "rstNSWindowDelegate";

/// A config for a window.
#[derive(Debug)]
pub struct WindowConfig {
    /// The style the window should have.
    pub style: UInt,

    /// The initial dimensions for the window.
    pub initial_dimensions: NSRect,

    /// When true, the window server defers creating the window device
    /// until the window is moved onscreen. All display messages sent to
    /// the window or its views are postponed until the window is created,
    /// just before it’s moved onscreen.
    pub defer: bool,

    /// The style of the toolbar
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
    /// Given a set of styles, converts them to `NSUInteger` and sets them on the
    /// window config.
    pub fn set_styles(&mut self, styles: &[NSWindowStyleMask]) {
        let mut style: UInt = 0;

        for mask in styles {
            let i = *mask as UInt;
            style |= i;
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

    /// Sets the toolbar style of this window.
    pub fn set_toolbar_style(&mut self, style: NSWindowToolbarStyle) {
        self.toolbar_style = style;
    }
}

/// A `Window` represents your way of interacting with an `NSWindow`. It wraps the various moving
/// pieces to enable you to focus on reacting to lifecycle methods and doing your thing.
#[derive(Debug)]
pub struct NSWindow<T = ()> {
    /// Represents an `NS/UIWindow` in the Objective-C runtime.
    pub ptr: ShareId<Object>,

    /// A delegate for this window.
    pub delegate: Option<Box<T>>,
}

impl Default for NSWindow {
    fn default() -> Self {
        NSWindow::new(WindowConfig::default())
    }
}

impl NSWindow {
    /// Constructs a new `NSWindow`
    pub fn new(config: WindowConfig) -> NSWindow {
        unsafe {
            NSWindow::<()>::tm_set_allows_automatic_window_tabbing(false);

            let alloc: id = msg_send![class!(NSWindow), alloc];

            let dimensions: NSRect = config.initial_dimensions;

            let mut window = NSWindow::from_id(alloc);

            window = window.im_init_with_content_rect_style_mask_backing_defer(
                dimensions,
                config.style,
                NSBackingStoreType::Buffered,
                config.defer,
            );

            window = window.autorelease();

            window.ip_set_released_when_closed(false);
            window.ip_set_restorable(false);

            let toolbar_style = config.toolbar_style;

            window.ip_set_toolbar_style(toolbar_style);

            window
        }
    }

    /// Allocates a new `Window`
    fn alloc<T>(delegate: &T) -> Self
    where
        T: PNSWindowDelegate + 'static,
    {
        let objc = unsafe {
            let class = register_window_class_with_delegate::<T>(delegate);
            let alloc: id = msg_send![class, alloc];
            ShareId::from_ptr(alloc)
        };

        NSWindow {
            ptr: objc,
            delegate: None,
        }
    }

    fn autorelease(&self) -> Self {
        unsafe { msg_send![&*self.ptr, autorelease] }
    }
}

impl<T> NSWindow<T>
where
    T: PNSWindowDelegate + 'static,
{
    /// Constructs a new NSWindow with a `config`
    pub fn with(config: WindowConfig, delegate: T) -> Self {
        let mut window = NSWindow::alloc::<T>(&delegate);
        let mut delegate = Box::new(delegate);

        let objc = unsafe {
            NSWindow::<()>::tm_set_allows_automatic_window_tabbing(false);

            let dimensions: NSRect = config.initial_dimensions;

            window = window.im_init_with_content_rect_style_mask_backing_defer(
                dimensions,
                config.style,
                NSBackingStoreType::Buffered,
                config.defer,
            );

            let delegate_ptr: *const T = &*delegate;
            let ptr: id = msg_send![&*window.ptr, self];

            (*ptr).set_ivar(WINDOW_DELEGATE_PTR, delegate_ptr as usize);

            let mut window = NSWindow::from_id(ptr);

            window = window.autorelease();

            window.ip_set_released_when_closed(false);

            window.ip_set_delegate(msg_send![&*window.ptr, self]);
            window.ip_set_restorable(false);

            window.ip_set_toolbar_style(config.toolbar_style);

            window
        };

        {
            delegate.did_load(NSWindow {
                delegate: None,
                ptr: objc.ptr.clone(),
            });
        }

        NSWindow {
            ptr: objc.ptr.clone(),
            delegate: Some(delegate),
        }
    }
}

impl<T> NSWindow<T> {
    /// Sets the string that appears in the title bar of the window or the path to the represented file.
    pub fn set_title<S>(&self, title: S)
    where
        S: Into<NSString>,
    {
        self.ip_set_title(title.into())
    }

    /// Sets a value that indicates the visibility of the window’s title and title bar buttons.
    pub fn set_title_visibility(&mut self, visibility: NSWindowTitleVisibility) {
        self.ip_set_title_visibility(visibility)
    }

    /// Sets a Boolean value that indicates whether the window is movable by clicking and dragging anywhere in its background.
    pub fn set_movable_by_background(&mut self, movable: bool) {
        self.ip_set_movable_by_window_background(movable)
    }

    /// Sets a Boolean value that indicates whether the title bar draws its background.
    pub fn set_titlebar_appears_transparent(&mut self, transparent: bool) {
        self.ip_set_titlebar_appears_transparent(transparent)
    }

    /// Sets the name AppKit uses to automatically save the window’s frame rectangle data in the defaults system.
    pub fn set_autosave_name<S>(&self, name: S) -> bool
    where
        S: Into<NSString>,
    {
        self.im_set_frame_autosave_name(name.into())
    }

    /// Sets the size of the window’s content view to a given size, which is expressed in the window’s base coordinate system.
    pub fn set_content_size(&self, size: NSSize) {
        self.im_set_content_size(size);
    }

    /// Sets the minimum size of the window’s content view in the window’s base coordinate system.
    pub fn set_minimum_content_size(&self, size: NSSize) {
        self.ip_set_content_min_size(size);
    }

    /// Sets the minimum size to which the window’s frame (including its title bar) can be sized.
    pub fn set_maximum_content_size(&self, size: NSSize) {
        self.ip_set_content_max_size(size);
    }

    /// Sets the minimum size this window can shrink to.
    pub fn set_minimum_size<F: Into<f64>>(&self, size: NSSize) {
        self.ip_set_min_size(size)
    }

    /// Toggles the visibility of the window’s toolbar.
    pub fn toggle_toolbar_shown(&self) {
        self.ip_toggle_toolbar_shown(nil)
    }

    /// Sets a Boolean value that indicates whether the toolbar control button is displayed.
    pub fn set_shows_toolbar_button(&self, shows: bool) {
        self.ip_set_shows_toolbar_button(shows);
    }

    /// Removes the window from the screen.
    pub fn close(&self) {
        self.im_close()
    }

    /// Takes the window into or out of fullscreen mode,
    pub fn toggle_full_screen(&self, sender: id) {
        self.im_toggle_full_screen(sender)
    }

    /// A Boolean value that indicates whether the window is opaque.
    pub fn is_opaque(&self) -> bool {
        self.ip_is_opaque()
    }

    /// A Boolean value that indicates whether the window is minimized.
    pub fn miniaturized(&self) -> bool {
        self.ip_miniaturized()
    }

    /// Removes the window from the screen list and displays the minimized window in the Dock.
    pub fn miniaturize(&self, sender: id) {
        self.im_miniaturize(sender)
    }

    /// De-minimizes the window.
    pub fn deminiaturize(&self, sender: id) {
        self.im_deminiaturize(sender)
    }

    /// Runs the Print panel, and if the user chooses an option other than canceling, prints the window (its frame view and all subviews).
    pub fn print(&self, sender: id) {
        self.im_print(sender)
    }

    /// A Boolean value that indicates whether the window is on the currently active space.
    pub fn is_on_active_space(&self) -> bool {
        self.ip_is_on_active_space()
    }

    /// A Boolean value that indicates whether the window is visible onscreen (even when it’s obscured by other windows).
    pub fn is_visible(&self) -> bool {
        self.ip_visible()
    }

    /// A Boolean value that indicates whether the window is the key window for the application.
    pub fn is_key_window(&self) -> bool {
        self.ip_key_window()
    }

    /// A Boolean value that indicates whether the window can become the key window.
    pub fn can_become_key_window(&self) -> bool {
        self.ip_can_become_key_window()
    }

    /// Makes the window the key window.
    pub fn make_key_window(&self) {
        self.im_make_key_window()
    }

    /// Moves the window to the front of the screen list, within its level, and makes it the key window; that is, it shows the window.
    pub fn make_key_and_order_front(&self, sender: id) {
        self.im_make_key_and_order_front(sender)
    }

    /// A Boolean value that indicates whether the window is the application’s main window.
    pub fn main_window(&self) -> bool {
        self.ip_main_window()
    }

    /// A Boolean value that indicates whether the window can become the application’s main window.
    pub fn can_become_main_window(&self) -> bool {
        self.ip_can_become_main_window()
    }

    /// Set whether this window should be excluded from the top-level "Windows" menu.
    pub fn set_excluded_from_windows_menu(&self, excluded: bool) {
        self.ip_set_excluded_from_windows_menu(excluded)
    }

    /// Sets the type of separator that the app displays between the title bar and content of a window.
    pub fn set_titlebar_separator_style(&self, style: NSTitlebarSeparatorStyle) {
        self.ip_set_titlebar_separator_style(style)
    }

    /// The backing scale factor.
    pub fn backing_scale_factor(&self) -> f64 {
        self.ip_backing_scale_factor()
    }

    /// Starts a document-modal session and presents—or queues for presentation—a sheet.
    pub fn begin_sheet_completion_handler<F, W>(&self, window: &NSWindow<W>, completion: F)
    where
        F: IntoConcreteBlock<(NSModalResponse,), Ret = ()> + 'static,
        W: PNSWindowDelegate + 'static,
    {
        let block = ConcreteBlock::new(completion);
        let block = block.copy();

        self.im_begin_sheet_completion_handler(window, block)
    }

    /// Ends a document-modal session and dismisses the specified sheet.
    pub fn end_sheet<W>(&self, window: &NSWindow<W>)
    where
        W: PNSWindowDelegate + 'static,
    {
        self.im_end_sheet(window);
    }
}

impl<T> PNSObject for NSWindow<T> {
    fn im_class<'a>() -> &'a Class {
        class!(NSWindow)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl<T> INSResponder for NSWindow<T> {}

impl<T> INSWindow for NSWindow<T> {}

impl<T> ToId for NSWindow<T> {
    fn to_id(self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl<T> FromId for NSWindow<T> {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: ShareId::from_ptr(id),
            delegate: None,
        }
    }
}

impl<T> Clone for NSWindow<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr.clone(),
            delegate: self.delegate.clone(),
        }
    }
}

impl<T> Drop for NSWindow<T> {
    fn drop(&mut self) {
        if self.delegate.is_some() {
            unsafe {
                self.ip_set_delegate(NSWindow::from_id(nil));
            }
        }
    }
}
