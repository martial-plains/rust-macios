use block::{ConcreteBlock, IntoConcreteBlock, RcBlock};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::ShareId;

use crate::{
    core_graphics::{CGFloat, CGPoint, CGSize},
    foundation::{
        Int, NSAlignmentOptions, NSArray, NSData, NSDictionary, NSNumber, NSPoint, NSRect,
        NSRectEdge, NSSize, NSString, NSTimeInterval, UInt,
    },
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{
    ns_window_delegate::{register_window_class_with_delegate, PNSWindowDelegate},
    INSResponder, INSViewController, NSBackingStoreType, NSButton, NSColor, NSColorSpace,
    NSDeviceDescriptionKey, NSDockTile, NSImage, NSModalResponse, NSScreen,
    NSTitlebarSeparatorStyle, NSToolbar, NSUserInterfaceLayoutDirection, NSViewController,
    NSWindowButton, NSWindowCollectionBehavior, NSWindowDepth, NSWindowFrameAutosaveName,
    NSWindowLevel, NSWindowNumberListOptions, NSWindowOcclusionState, NSWindowOrderingMode,
    NSWindowPersistableFrameDescriptor, NSWindowSharingType, NSWindowStyleMask,
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
    fn m_class<'a>() -> &'a Class {
        class!(NSWindow)
    }

    fn m_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl<T> INSResponder for NSWindow<T> {}

/// A window that an app displays on the screen.
pub trait INSWindow: INSResponder {
    /* Creating a Window
     */

    /// Creates a titled window that contains the specified content view controller.
    fn tm_window_with_content_view_controller<V>(content_view_controller: V) -> Self
    where
        Self: Sized + FromId,
        V: INSViewController,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                windowWithContentViewController: content_view_controller
            ])
        }
    }

    /// Initializes the window with the specified values.
    fn im_init_with_content_rect_style_mask_backing_defer(
        &self,
        content_rect: NSRect,
        style: UInt,
        backing_store_type: NSBackingStoreType,
        flag: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                initWithContentRect: content_rect
                styleMask: style
                backing: backing_store_type
                defer: flag
            ])
        }
    }

    /// Initializes an allocated window with the specified values.
    fn im_init_with_content_rect_style_mask_backing_defer_screen(
        &self,
        content_rect: NSRect,
        style: UInt,
        backing_store_type: NSBackingStoreType,
        flag: bool,
        screen: NSScreen,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                initWithContentRect: content_rect
                styleMask: style
                backing: backing_store_type
                defer: flag
                screen: screen
            ])
        }
    }

    /* Managing the Window's Behavior
     */

    /// The window’s delegate.
    fn ip_delegate(&self) -> id {
        unsafe { msg_send![self.m_self(), delegate] }
    }

    /// Sets the window’s delegate.
    ///
    /// # Arguments
    ///
    /// * `delegate` - The delegate object.
    fn ip_set_delegate(&self, delegate: NSWindow) {
        unsafe { msg_send![self.m_self(), setDelegate: delegate] }
    }

    /* Configuring the Window's Appearance
     */

    /// The main content view controller for the window.
    fn ip_content_view_controller(&self) -> NSViewController {
        unsafe { msg_send![self.m_self(), contentViewController] }
    }

    /* Configuring the Window's Appearance
     */

    /// Flags that describe the window’s current style, such as if it’s resizable or in full-screen mode.
    fn ip_style_mask(&self) -> NSWindowStyleMask {
        unsafe { msg_send![self.m_self(), styleMask] }
    }

    /// Sets the flags that describe the window’s current style, such as if it’s resizable or in full-screen mode.
    ///
    /// # Arguments
    ///
    /// * `style_mask` - The flags that describe the window’s current style, such as if it’s resizable or in full-screen mode.
    fn ip_set_style_mask(&mut self, style_mask: NSWindowStyleMask) {
        unsafe { msg_send![self.m_self(), setStyleMask: style_mask] }
    }

    /// Takes the window into or out of fullscreen mode,
    fn im_toggle_full_screen(&self, sender: id) {
        unsafe { msg_send![self.m_self(), toggleFullScreen: sender] }
    }

    /// A Boolean value that indicates whether the window is able to receive keyboard and mouse events even when some other window is being run modally.
    fn ip_works_when_modal(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), worksWhenModal]) }
    }

    /// The window’s alpha value.
    fn ip_alpha_value(&self) -> CGFloat {
        unsafe { msg_send![self.m_self(), alphaValue] }
    }

    /// Sets the window’s alpha value.
    ///
    /// # Arguments
    ///
    /// * `value` - The window’s alpha value.
    fn ip_set_alpha_value(&mut self, value: CGFloat) {
        unsafe { msg_send![self.m_self(), setAlphaValue: value] }
    }

    /// The color of the window’s background.
    fn ip_background_color(&self) -> NSColor {
        unsafe { NSColor::from_id(msg_send![self.m_self(), backgroundColor]) }
    }

    /// Sets the color of the window’s background.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the window’s background.
    fn ip_set_background_color(&mut self, color: NSColor) {
        unsafe { msg_send![self.m_self(), setBackgroundColor: color] }
    }

    /// The window’s color space.
    fn ip_color_space(&self) -> NSColorSpace {
        unsafe { NSColorSpace::from_id(msg_send![self.m_self(), colorSpace]) }
    }

    /// Sets the window’s color space.
    ///
    /// # Arguments
    ///
    /// * `color_space` - The window’s color space.
    fn ip_set_color_space(&mut self, color_space: NSColorSpace) {
        unsafe { msg_send![self.m_self(), setColorSpace: color_space] }
    }

    /// Sets a Boolean value that indicates whether the window’s depth limit can change to match the depth of the screen it’s on.
    fn im_set_dynamic_depth_limit(&self, flag: bool) {
        unsafe { msg_send![self.m_self(), setDynamicDepthLimit: flag] }
    }

    /// A Boolean value that indicates whether the window can hide when its application becomes hidden.
    fn ip_can_hide(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), canHide]) }
    }

    /// Sets a Boolean value that indicates whether the window can hide when its application becomes hidden.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window can hide when its application becomes hidden.
    fn ip_set_can_hide(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setCanHide: flag] }
    }

    /// A Boolean value that indicates whether the window is on the currently active space.
    fn ip_is_on_active_space(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isOnActiveSpace]) }
    }

    /// A Boolean value that indicates whether the window is removed from the screen when its application becomes inactive.
    fn ip_hides_on_deactivate(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), hidesOnDeactivate]) }
    }

    /// Sets a Boolean value that indicates whether the window is removed from the screen when its application becomes inactive.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window is removed from the screen when its application becomes inactive.
    fn ip_set_hides_on_deactivate(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setHidesOnDeactivate: flag] }
    }

    /// A value that identifies the window’s behavior in window collections.
    fn ip_collection_behavior(&self) -> NSWindowCollectionBehavior {
        unsafe { msg_send![self.m_self(), collectionBehavior] }
    }

    /// A Boolean value that indicates whether the window is opaque.
    fn ip_is_opaque(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isOpaque]) }
    }

    /// Sets a Boolean value that indicates whether the window is opaque.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window is opaque.
    fn ip_set_opaque(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setOpaque: flag] }
    }

    /// A Boolean value that indicates whether the window has a shadow.
    fn ip_has_shadow(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), hasShadow]) }
    }

    /// Sets a Boolean value that indicates whether the window has a shadow.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window has a shadow.
    fn ip_set_has_shadow(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setHasShadow: flag] }
    }

    /// Invalidates the window shadow so that it is recomputed based on the current window shape.
    fn im_invalidate_shadow(&self) {
        unsafe { msg_send![self.m_self(), invalidateShadow] }
    }

    /// Indicates whether the window calculates the thickness of a given border automatically.
    fn im_autorecalculates_content_border_thickness_for_edge(&self, edge: NSRectEdge) -> bool {
        unsafe {
            to_bool(msg_send![
                self.m_self(),
                autorecalculatesContentBorderThicknessForEdge: edge
            ])
        }
    }

    /// Specifies whether the window calculates the thickness of a given border automatically.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window calculates the thickness of a given border automatically.
    /// * `edge` - The edge of the window for which to set the autorecalculates content border thickness value.
    fn im_set_autorecalculates_content_border_thickness_for_edge(
        &self,
        flag: bool,
        edge: NSRectEdge,
    ) {
        unsafe {
            msg_send![self.m_self(), setAutorecalculatesContentBorderThickness: flag forEdge: edge]
        }
    }

    /// Indicates the thickness of a given border of the window.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge of the window for which to return the thickness.
    fn im_content_border_thickness_for_edge(&self, edge: NSRectEdge) -> CGFloat {
        unsafe { msg_send![self.m_self(), contentBorderThicknessForEdge: edge] }
    }

    /// Specifies the thickness of a given border of the window.
    ///
    /// # Arguments
    ///
    /// * `thickness` - The thickness of the border.
    /// * `edge` - The edge of the window for which to set the thickness.
    fn im_set_content_border_thickness_for_edge(&self, thickness: CGFloat, edge: NSRectEdge) {
        unsafe { msg_send![self.m_self(), setContentBorderThickness: thickness forEdge: edge] }
    }

    /// A Boolean value that indicates whether the window prevents application termination when modal.
    fn ip_prevents_application_termination(&self) -> bool {
        unsafe {
            to_bool(msg_send![
                self.m_self(),
                preventsApplicationTerminationWhenModal
            ])
        }
    }

    /// An object that the window inherits its appearance from.
    fn ip_appearance_source(&self) -> id {
        unsafe { msg_send![self.m_self(), appearanceSource] }
    }

    /* Accessing Window Information
     */

    /// The depth limit of the window.
    fn ip_depth_limit(&self) -> NSWindowDepth {
        unsafe { msg_send![self.m_self(), depthLimit] }
    }

    /// Sets the depth limit of the window.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth limit of the window.
    fn ip_set_depth_limit(&mut self, depth: NSWindowDepth) {
        unsafe { msg_send![self.m_self(), setDepthLimit: depth] }
    }

    /// A Boolean value that indicates whether the window’s depth limit can change to match the depth of the screen it’s on.
    fn ip_has_dynamic_depth_limit(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), hasDynamicDepthLimit]) }
    }

    /// Returns the default depth limit for instances of NSWindow.
    fn tp_default_depth_limit() -> NSWindowDepth {
        unsafe { msg_send![Self::m_class(), defaultDepthLimit] }
    }

    /// The window number of the window’s window device.
    fn ip_window_number(&self) -> Int {
        unsafe { msg_send![self.m_self(), windowNumber] }
    }

    /// Returns the window numbers for all visible windows satisfying the specified options.
    fn tm_window_numbers_with_options(options: NSWindowNumberListOptions) -> NSArray<NSNumber> {
        unsafe {
            NSArray::from_id(msg_send![
                Self::m_class(),
                windowNumbersWithOptions: options
            ])
        }
    }

    /// A dictionary containing information about the window’s resolution, such as color, depth, and so on.
    fn ip_device_description(&self) -> NSDictionary<NSDeviceDescriptionKey, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), deviceDescription]) }
    }

    /// A Boolean value that indicates whether the window can be displayed at the login window.
    fn ip_can_become_visible_without_login(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), canBecomeVisibleWithoutLogin]) }
    }

    /// A Boolean value that indicates the level of access other processes have to the window’s content.
    fn ip_sharing_type(&self) -> NSWindowSharingType {
        unsafe { msg_send![self.m_self(), sharingType] }
    }

    /// The window’s backing store type.
    fn ip_backing_type(&self) -> NSBackingStoreType {
        unsafe { msg_send![self.m_self(), backingType] }
    }

    /* Getting Layout Information
     */

    /// Returns the content rectangle used by a window with a given frame rectangle and window style.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle of the window.
    /// * `style` - The window style.
    fn tm_content_rect_for_frame_rect_style_mask(
        frame: NSRect,
        style: NSWindowStyleMask,
    ) -> NSRect {
        unsafe { msg_send![Self::m_class(), contentRectForFrameRect: frame styleMask: style] }
    }

    /// Returns the frame rectangle used by a window with a given content rectangle and window style.
    ///
    /// # Arguments
    ///
    /// * `content` - The content rectangle of the window.
    /// * `style` - The window style.
    fn tm_frame_rect_for_content_rect_style_mask(
        content: NSRect,
        style: NSWindowStyleMask,
    ) -> NSRect {
        unsafe { msg_send![Self::m_class(), frameRectForContentRect: content styleMask: style] }
    }

    /// Returns the minimum width a window’s frame rectangle must have for it to display a title, with a given window style.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the window.
    /// * `style` - The window style.
    fn tm_min_frame_width_with_title_style_mask(title: &str, style: NSWindowStyleMask) -> CGFloat {
        unsafe { msg_send![Self::m_class(), minFrameWidthWithTitle: title styleMask: style] }
    }

    /// Returns the window’s content rectangle with a given frame rectangle.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle of the window.
    fn im_content_rect_for_frame_rect(&self, frame: NSRect) -> NSRect {
        unsafe { msg_send![self.m_self(), contentRectForFrameRect: frame] }
    }

    /// Returns the window’s frame rectangle with a given content rectangle.
    fn im_frame_rect_for_content_rectim_frame_rect_for_content_rect(
        &self,
        content: NSRect,
    ) -> NSRect {
        unsafe { msg_send![self.m_self(), frameRectForContentRect: content] }
    }

    /* Managing Sheets
     */

    /// The sheet attached to the window.
    fn ip_attached_sheet(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![self.m_self(), attachedSheet]) }
    }

    /// The sheet attached to the window.
    fn ip_sheet(&mut self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isSheet]) }
    }

    /// Starts a document-modal session and presents—or queues for presentation—a sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet` - The sheet to present.
    /// * `handler` - The handler to call when the sheet's modal session ends.
    fn im_begin_sheet_completion_handler<W>(
        &self,
        sheet: &NSWindow<W>,
        handler: RcBlock<(NSModalResponse,), ()>,
    ) where
        W: PNSWindowDelegate + 'static,
    {
        unsafe { msg_send![self.m_self(), beginSheet: sheet completionHandler: handler] }
    }

    /// Starts a document-modal session and presents the specified critical sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet` - The sheet to present.
    /// * `handler` - The handler to call when the sheet's modal session ends.
    fn im_begin_critical_sheet_completion_handler(
        &self,
        sheet: NSWindow,
        handler: RcBlock<NSModalResponse, ()>,
    ) {
        unsafe { msg_send![self.m_self(), beginCriticalSheet: sheet completionHandler: handler] }
    }

    /// Ends a document-modal session and dismisses the specified sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet` - The sheet to dismiss.
    fn im_end_sheet<W>(&self, sheet: &NSWindow<W>)
    where
        W: PNSWindowDelegate + 'static,
    {
        unsafe { msg_send![self.m_self(), endSheet: sheet] }
    }

    /// Ends a document-modal session and dismisses the specified sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet` - The sheet to dismiss.
    /// * `code` - The return code of the sheet.
    fn im_end_sheet_with_return_code(&self, sheet: NSWindow, code: NSModalResponse) {
        unsafe { msg_send![self.m_self(), endSheet: sheet returnCode: code] }
    }

    /// The window to which the sheet is attached.
    fn ip_sheet_parent(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![self.m_self(), sheetParent]) }
    }

    /// An array of the sheets currently attached to the window.
    fn ip_sheets(&self) -> NSArray<NSWindow> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), sheets]) }
    }

    /*Sizing Windows
     */

    /// The window’s frame rectangle in screen coordinates, including the title bar.
    fn ip_frame(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), frame] }
    }

    /// Positions the bottom-left corner of the window’s frame rectangle at a given point in screen coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - The point at which to position the bottom-left corner of the window’s frame rectangle.
    fn im_set_frame_origin(&self, point: NSPoint) {
        unsafe { msg_send![self.m_self(), setFrameOrigin: point] }
    }

    /// Positions the top-left corner of the window’s frame rectangle at a given point in screen coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - The point at which to position the top-left corner of the window’s frame rectangle.
    fn im_set_frame_top_left_point(&self, point: NSPoint) {
        unsafe { msg_send![self.m_self(), setFrameTopLeftPoint: point] }
    }

    /// Modifies and returns a frame rectangle so that its top edge lies on a specific screen.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle to modify.
    /// * `screen` - The screen on which to position the top edge of the frame rectangle.
    fn im_constrain_frame_rect_to_screen(&self, frame: NSRect, screen: NSScreen) -> NSRect {
        unsafe { msg_send![self.m_self(), constrainFrameRect: frame toScreen: screen] }
    }

    /// Positions the window’s top-left to a given point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point at which to position the top-left corner of the window.
    fn im_cascade_top_left_from_point(&self, point: NSPoint) {
        unsafe { msg_send![self.m_self(), cascadeTopLeftFromPoint: point] }
    }

    /// Sets the origin and size of the window’s frame rectangle according to a given frame rectangle, thereby setting its position and size onscreen.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle to set.
    /// * `flag` - When `true` the window sends a `displayIfNeeded` message down its view hierarchy, thus redrawing all views.
    fn im_set_frame_display(&self, frame: NSRect, flag: bool) {
        unsafe { msg_send![self.m_self(), setFrame: frame display: flag] }
    }

    /// Sets the origin and size of the window’s frame rectangle, with optional animation, according to a given frame rectangle, thereby setting its position and size onscreen.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle to set.
    /// * `flag` - When `true` the window sends a `displayIfNeeded` message down its view hierarchy, thus redrawing all views.
    /// * `animate` - `true` to perform the animation, whose duration is specified by `animation_resize_time`
    fn im_set_frame_display_animate(&self, frame: NSRect, flag: bool, animate: bool) {
        unsafe { msg_send![self.m_self(), setFrame: frame display: flag animate: animate] }
    }

    /// Specifies the duration of a smooth frame-size change.
    fn im_animation_resize_time(&self, frame: NSRect) -> NSTimeInterval {
        unsafe { msg_send![self.m_self(), animationResizeTime: frame] }
    }

    /// The window’s aspect ratio, which constrains the size of its frame rectangle to integral multiples of this ratio when the user resizes it.
    fn ip_aspect_ratio(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), aspectRatio] }
    }

    /// Sets the window’s aspect ratio.
    ///
    /// # Arguments
    ///
    /// * `ratio` - The aspect ratio to set.
    fn ip_set_aspect_ratio(&self, ratio: NSSize) {
        unsafe { msg_send![self.m_self(), setAspectRatio: ratio] }
    }

    /// The minimum size to which the window’s frame (including its title bar) can be sized.
    fn ip_min_size(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), minSize] }
    }

    /// Sets the minimum size to which the window’s frame (including its title bar) can be sized.
    ///
    /// # Arguments
    ///
    /// * `size` - The minimum size to which the window’s frame (including its title bar) can be sized.
    fn ip_set_min_size(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setMinSize: size] }
    }

    /// The maximum size to which the window’s frame (including its title bar) can be sized.
    fn ip_max_size(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), maxSize] }
    }

    /// Sets the maximum size to which the window’s frame (including its title bar) can be sized.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum size to which the window’s frame (including its title bar) can be sized.
    fn ip_set_max_size(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setMaxSize: size] }
    }

    /// A Boolean value that indicates whether the window is in a zoomed state.
    fn ip_zoomed(&self) -> bool {
        unsafe { msg_send![self.m_self(), isZoomed] }
    }

    /// This action method simulates the user clicking the zoom box by momentarily highlighting the button and then zooming the window.
    fn im_perform_zoom(&self, sender: id) {
        unsafe { msg_send![self.m_self(), performZoom: sender] }
    }

    /// Toggles the size and location of the window between its standard state (which the application provides as the best size to display the window’s data) and its user state (a new size and location the user may have set by moving or resizing the window).
    fn im_zoom(&self, sender: id) {
        unsafe { msg_send![self.m_self(), zoom: sender] }
    }

    /// The window’s resizing increments.
    fn ip_resize_increments(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), resizeIncrements] }
    }

    /// Sets the window’s resizing increments.
    ///
    /// # Arguments
    ///
    /// * `increments` - The resizing increments to set.
    fn ip_set_resize_increments(&self, increments: NSSize) {
        unsafe { msg_send![self.m_self(), setResizeIncrements: increments] }
    }

    /// A Boolean value that indicates whether the window tries to optimize user-initiated resize operations by preserving the content of views that have not changed.
    fn ip_preserves_content_during_live_resize(&self) -> bool {
        unsafe { msg_send![self.m_self(), preservesContentDuringLiveResize] }
    }

    /// Sets whether the window tries to optimize user-initiated resize operations by preserving the content of views that have not changed.
    ///
    /// # Arguments
    ///
    /// * `flag` - `true` to optimize user-initiated resize operations by preserving the content of views that have not changed.
    fn ip_set_preserves_content_during_live_resize(&self, flag: bool) {
        unsafe { msg_send![self.m_self(), setPreservesContentDuringLiveResize: flag] }
    }

    /// A Boolean value that indicates whether the window is being resized by the user.
    fn ip_in_live_resize(&self) -> bool {
        unsafe { msg_send![self.m_self(), inLiveResize] }
    }

    /// Sets a Boolean value that indicates whether the window is being resized by the user.
    ///
    /// # Arguments
    ///
    /// * `flag` - `true` if the window is being resized by the user.
    fn ip_set_in_live_resize(&self, flag: bool) {
        unsafe { msg_send![self.m_self(), setInLiveResize: flag] }
    }

    /* Sizing Content
     */

    /// The window’s content aspect ratio.
    fn ip_content_aspect_ratio(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), contentAspectRatio] }
    }

    /// Sets the window’s content aspect ratio.
    ///
    /// # Arguments
    ///
    /// * `content_aspect_ratio` - The window’s content aspect ratio.
    fn ip_set_content_aspect_ratio(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setContentAspectRatio: size] }
    }

    /// The minimum size of the window’s content view in the window’s base coordinate system.
    fn ip_content_min_size(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), contentMinSize] }
    }

    /// Sets the minimum size of the window’s content view in the window’s base coordinate system.
    ///
    /// # Arguments
    ///
    /// * `content_min_size` - The minimum size of the window’s content view in the window’s base coordinate system.
    fn ip_set_content_min_size(&self, content_min_size: NSSize) {
        unsafe { msg_send![self.m_self(), setContentMinSize: content_min_size] }
    }

    /// Sets the size of the window’s content view to a given size, which is expressed in the window’s base coordinate system.
    ///
    /// # Arguments
    ///
    /// * `content_size` - The size of the window’s content view in the window’s base coordinate system.
    fn im_set_content_size(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setContentSize: size] }
    }

    /// The maximum size of the window’s content view in the window’s base coordinate system.
    fn ip_content_max_size(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), contentMaxSize] }
    }

    /// Sets the maximum size of the window’s content view in the window’s base coordinate system.
    ///
    /// # Arguments
    ///
    /// * `content_max_size` - The maximum size of the window’s content view in the window’s base coordinate system.
    fn ip_set_content_max_size(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setContentMaxSize: size] }
    }

    /// The window’s content-view resizing increments.
    fn ip_content_resize_increments(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), contentResizeIncrements] }
    }

    /// Sets the window’s content-view resizing increments.
    ///
    /// # Arguments
    ///
    /// * `content_resize_increments` - The window’s content-view resizing increments.
    fn ip_set_content_resize_increments(&self, content_resize_increments: NSSize) {
        unsafe {
            msg_send![
                self.m_self(),
                setContentResizeIncrements: content_resize_increments
            ]
        }
    }

    /// A value used by Auto Layout constraints to automatically bind to the value of contentLayoutRect.
    fn ip_content_layout_guide(&self) -> id {
        unsafe { msg_send![self.m_self(), contentLayoutGuide] }
    }

    /// Sets a value used by Auto Layout constraints to automatically bind to the value of contentLayoutRect.
    ///
    /// # Arguments
    ///
    /// * `content_layout_guide` - A value used by Auto Layout constraints to automatically bind to the value of contentLayoutRect.
    fn ip_set_content_layout_guide(&self, content_layout_guide: id) {
        unsafe { msg_send![self.m_self(), setContentLayoutGuide: content_layout_guide] }
    }

    /// The area inside the window that is for non-obscured content, in window coordinates.
    fn ip_content_layout_rect(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), contentLayoutRect] }
    }

    /// Sets the area inside the window that is for non-obscured content, in window coordinates.
    ///
    /// # Arguments
    ///
    /// * `content_layout_rect` - The area inside the window that is for non-obscured content, in window coordinates.
    fn ip_set_content_layout_rect(&self, content_layout_rect: NSRect) {
        unsafe { msg_send![self.m_self(), setContentLayoutRect: content_layout_rect] }
    }

    /// A maximum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_full_screen_tile_size(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), fullScreenTileSize] }
    }

    /// Sets a maximum size that is used to determine if a window can fit when it is in full screen in a tile.
    ///
    /// # Arguments
    ///
    /// * `full_screen_tile_size` - A maximum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_set_full_screen_tile_size(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setFullScreenTileSize: size] }
    }

    /// A minimum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_full_screen_tile_content_size(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), fullScreenTileContentSize] }
    }

    /// Sets a minimum size that is used to determine if a window can fit when it is in full screen in a tile.
    ///
    /// # Arguments
    ///
    /// * `full_screen_tile_content_size` - A minimum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_set_full_screen_tile_content_size(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setFullScreenTileContentSize: size] }
    }

    /* Managing Window Layers
     */

    /// Removes the window from the screen list, which hides the window.
    fn im_order_out(&self, sender: id) {
        unsafe { msg_send![self.m_self(), orderOut: sender] }
    }

    /// Moves the window to the back of its level in the screen list, without changing either the key window or the main window.
    fn im_order_back(&self, sender: id) {
        unsafe { msg_send![self.m_self(), orderBack: sender] }
    }

    /// Moves the window to the front of its level in the screen list, without changing either the key window or the main window.
    fn im_order_front(&self, sender: id) {
        unsafe { msg_send![self.m_self(), orderFront: sender] }
    }

    /// Moves the window to the front of its level, even if its application isn’t active, without changing either the key window or the main window.
    fn im_order_front_regardless(&self) {
        unsafe { msg_send![self.m_self(), orderFrontRegardless] }
    }

    /// Repositions the window’s window device in the window server’s screen list.
    fn im_order_window_relative_to(&self, place: NSWindowOrderingMode, other: Int) {
        unsafe { msg_send![self.m_self(), orderWindow: place relativeTo: other] }
    }

    /// The window level of the window.
    fn ip_window_level(&self) -> NSWindowLevel {
        unsafe { msg_send![self.m_self(), windowLevel] }
    }

    /// Sets the window level of the window.
    ///
    /// # Arguments
    ///
    /// * `level` - The window level of the window.
    fn ip_set_window_level(&self, level: NSWindowLevel) {
        unsafe { msg_send![self.m_self(), setWindowLevel: level] }
    }

    /* Managing Window Visibility and Occlusion State
     */

    /// A Boolean value that indicates whether the window is visible onscreen (even when it’s obscured by other windows).
    fn ip_visible(&self) -> bool {
        unsafe { msg_send![self.m_self(), isVisible] }
    }

    /// The occlusion state of the window.
    fn ip_occlusion_state(&self) -> NSWindowOcclusionState {
        unsafe { msg_send![self.m_self(), occlusionState] }
    }

    /* Managing Window Frames in User Defaults
     */

    /// Removes the frame data stored under a given name from the application’s user defaults.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the frame data to remove.
    fn tm_remove_frame_using_name(name: &NSWindowFrameAutosaveName) {
        unsafe { msg_send![Self::m_class(), removeFrameUsingName: name] }
    }

    /// Sets the window’s frame rectangle by reading the rectangle data stored under a given name from the defaults system.
    fn im_set_frame_using_name(&self, name: NSWindowFrameAutosaveName) -> bool {
        unsafe { msg_send![self.m_self(), setFrameUsingName: name] }
    }

    /// Sets the window’s frame rectangle by reading the rectangle data stored under a given name from the defaults system. Can operate on non-resizable windows.
    fn im_set_frame_using_name_force(&self, name: NSWindowFrameAutosaveName, force: bool) -> bool {
        unsafe { msg_send![self.m_self(), setFrameUsingName:name force:force] }
    }

    /// Saves the window’s frame rectangle in the user defaults system under a given name.
    fn im_save_frame_using_name(&self, name: NSWindowFrameAutosaveName) {
        unsafe { msg_send![self.m_self(), saveFrameUsingName: name] }
    }

    /// Sets the name AppKit uses to automatically save the window’s frame rectangle data in the defaults system.
    fn im_set_frame_autosave_name(&self, name: NSWindowFrameAutosaveName) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), setFrameAutosaveName: name]) }
    }

    /// The name used to automatically save the window’s frame rectangle data in the defaults system.
    fn ip_frame_autosave_name(&self) -> NSWindowFrameAutosaveName {
        unsafe { msg_send![self.m_self(), frameAutosaveName] }
    }

    /// A string representation of the window’s frame rectangle.
    fn ip_string_with_saved_frame(&self) -> NSWindowPersistableFrameDescriptor {
        unsafe { msg_send![self.m_self(), stringWithSavedFrame] }
    }

    /// Sets the window’s frame rectangle from a given string representation.
    fn im_set_frame_from_string(&self, string: NSWindowPersistableFrameDescriptor) {
        unsafe { msg_send![self.m_self(), setFrameFromString: string] }
    }

    /* Managing Key Status
     */

    /// A Boolean value that indicates whether the window is the key window for the application.
    fn ip_key_window(&self) -> bool {
        unsafe { msg_send![self.m_self(), isKeyWindow] }
    }

    /// A Boolean value that indicates whether the window can become the key window.
    fn ip_can_become_key_window(&self) -> bool {
        unsafe { msg_send![self.m_self(), canBecomeKeyWindow] }
    }

    /// Makes the window the key window.
    fn im_make_key_window(&self) {
        unsafe { msg_send![self.m_self(), makeKeyWindow] }
    }

    /// Moves the window to the front of the screen list, within its level, and makes it the key window; that is, it shows the window.
    fn im_make_key_and_order_front(&self, sender: id) {
        unsafe { msg_send![self.m_self(), makeKeyAndOrderFront: sender] }
    }

    /// Informs the window that it has become the key window.
    fn im_become_key_window(&self) {
        unsafe { msg_send![self.m_self(), becomeKeyWindow] }
    }

    /// Resigns the window’s key window status.
    fn im_resign_key_window(&self) {
        unsafe { msg_send![self.m_self(), resignKeyWindow] }
    }

    /* Managing Main Status
     */

    /// A Boolean value that indicates whether the window is the application’s main window.
    fn ip_main_window(&self) -> bool {
        unsafe { msg_send![self.m_self(), isMainWindow] }
    }

    /// A Boolean value that indicates whether the window can become the application’s main window.
    fn ip_can_become_main_window(&self) -> bool {
        unsafe { msg_send![self.m_self(), canBecomeMainWindow] }
    }

    /// Makes the window the main window.
    fn im_make_main_window(&self) {
        unsafe { msg_send![self.m_self(), makeMainWindow] }
    }

    /// Informs the window that it has become the main window.
    fn im_become_main_window(&self) {
        unsafe { msg_send![self.m_self(), becomeMainWindow] }
    }

    /// Resigns the window’s main window status.
    fn im_resign_main_window(&self) {
        unsafe { msg_send![self.m_self(), resignMainWindow] }
    }

    /// The window’s toolbar.
    fn ip_toolbar(&self) -> NSToolbar {
        unsafe { msg_send![self.m_self(), toolbar] }
    }

    /// Sets the window’s toolbar.
    ///
    /// # Arguments
    ///
    /// * `toolbar` - The window’s toolbar.
    fn ip_set_toolbar(&self, toolbar: NSToolbar) {
        unsafe { msg_send![self.m_self(), setToolbar: toolbar] }
    }

    /// Toggles the visibility of the window’s toolbar.
    fn ip_toggle_toolbar_shown(&self, sender: id) {
        unsafe { msg_send![self.m_self(), toggleToolbarShown: sender] }
    }

    /// Presents the toolbar customization user interface.
    fn ip_run_toolbar_customization_palette(&self, sender: id) {
        unsafe { msg_send![self.m_self(), runToolbarCustomizationPalette: sender] }
    }

    /* Managing Attached Windows
     */

    /// An array of the window’s attached child windows.
    fn ip_child_windows(&self) -> id {
        unsafe { msg_send![self.m_self(), childWindows] }
    }

    /// Attaches a child window to the window.
    fn add_child_window_ordered(&self, child: id, order: NSWindowOrderingMode) {
        unsafe { msg_send![self.m_self(), addChildWindow: child ordered: order] }
    }

    /// Detaches a given child window from the window.
    fn remove_child_window(&self, child: id) {
        unsafe { msg_send![self.m_self(), removeChildWindow: child] }
    }

    /// The parent window to which the window is attached as a child.
    fn ip_parent_window(&self) -> id {
        unsafe { msg_send![self.m_self(), parentWindow] }
    }

    /* Managing the Window Menu
     */

    /// A Boolean value that indicates whether the window is excluded from the application’s Windows menu.
    fn ip_excluded_from_windows_menu(&self) -> bool {
        unsafe { msg_send![self.m_self(), isExcludedFromWindowsMenu] }
    }

    /// Sets a Boolean value that indicates whether the window is excluded from the application’s Windows menu.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window is excluded from the application’s Windows menu.
    fn ip_set_excluded_from_windows_menu(&self, flag: bool) {
        unsafe { msg_send![self.m_self(), setExcludedFromWindowsMenu: flag] }
    }

    /* Managing Cursor Rectangles
     */

    /// A Boolean value that indicates whether the window’s cursor rectangles are enabled.
    fn ip_are_cursor_rects_enabled(&self) -> bool {
        unsafe { msg_send![self.m_self(), areCursorRectsEnabled] }
    }

    /// Reenables cursor rectangle management within the window after a disableCursorRects message.
    fn im_enable_cursor_rects(&self) {
        unsafe { msg_send![self.m_self(), enableCursorRects] }
    }

    /// Disables all cursor rectangle management within the window.
    fn im_disable_cursor_rects(&self) {
        unsafe { msg_send![self.m_self(), disableCursorRects] }
    }

    /// Invalidates all cursor rectangles in the window.
    fn im_discard_cursor_rects(&self, view: id) {
        unsafe { msg_send![self.m_self(), discardCursorRectsForView: view] }
    }

    /// Clears the window’s cursor rectangles and the cursor rectangles of the NSView objects in its view hierarchy.
    fn im_reset_cursor_rects(&self) {
        unsafe { msg_send![self.m_self(), resetCursorRects] }
    }

    /* Managing Title Bars
     */

    /// Returns a new instance of a given standard window button, sized appropriately for a given window style.
    fn tm_standard_window_button_for_style_mask(
        &self,
        b: NSWindowButton,
        style: NSWindowStyleMask,
    ) -> NSButton {
        unsafe {
            NSButton::from_id(
                msg_send![Self::m_class(), standardWindowButton: b forStyleMask: style],
            )
        }
    }

    /// Returns the window button of a given window button kind in the window’s view hierarchy.
    fn ip_standard_window_button(&self, b: NSWindowButton) -> NSButton {
        unsafe { msg_send![self.m_self(), standardWindowButton: b] }
    }

    /// A Boolean value that indicates whether the toolbar control button is currently displayed.
    fn ip_shows_toolbar_button(&self) -> bool {
        unsafe { msg_send![self.m_self(), showsToolbarButton] }
    }

    /// Sets a Boolean value that indicates whether the toolbar control button is displayed.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the toolbar control button is displayed.
    fn ip_set_shows_toolbar_button(&self, flag: bool) {
        unsafe { msg_send![self.m_self(), setShowsToolbarButton: flag] }
    }

    /// A Boolean value that indicates whether the title bar draws its background.
    fn ip_titlebar_appears_transparent(&self) -> bool {
        unsafe { msg_send![self.m_self(), titlebarAppearsTransparent] }
    }

    /// Sets a Boolean value that indicates whether the title bar draws its background.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the title bar draws its background.
    fn ip_set_titlebar_appears_transparent(&self, flag: bool) {
        unsafe { msg_send![self.m_self(), setTitlebarAppearsTransparent: flag] }
    }

    /// The style that determines the appearance and location of the toolbar in relation to the title bar.
    fn ip_toolbar_style(&self) -> NSWindowToolbarStyle {
        unsafe { msg_send![self.m_self(), toolbarStyle] }
    }

    /// Sets the style that determines the appearance and location of the toolbar in relation to the title bar.
    ///
    /// # Arguments
    ///
    /// * `style` - The style that determines the appearance and location of the toolbar in relation to the title bar.
    fn ip_set_toolbar_style(&self, style: NSWindowToolbarStyle) {
        unsafe { msg_send![self.m_self(), setToolbarStyle: style] }
    }

    /// The type of separator that the app displays between the title bar and content of a window.
    fn ip_titlebar_separator_style(&self) -> NSTitlebarSeparatorStyle {
        unsafe { msg_send![self.m_self(), titlebarSeparatorStyle] }
    }

    /// Sets the type of separator that the app displays between the title bar and content of a window.
    ///
    /// # Arguments
    ///
    /// * `style` - The type of separator that the app displays between the title bar and content of a window.
    fn ip_set_titlebar_separator_style(&self, style: NSTitlebarSeparatorStyle) {
        unsafe { msg_send![self.m_self(), setTitlebarSeparatorStyle: style] }
    }

    /// The direction the window’s title bar lays text out, either left to right or right to left.
    fn ip_titlebar_layout_direction(&self) -> NSUserInterfaceLayoutDirection {
        unsafe { msg_send![self.m_self(), titlebarLayoutDirection] }
    }

    /* Managing Window Tabs
     */

    /// A Boolean value that indicates whether the app can automatically organize windows into tabs.
    fn tp_allows_automatic_window_tabbing() -> bool {
        unsafe { to_bool(msg_send![Self::m_class(), allowsAutomaticWindowTabbing]) }
    }

    /// Sets a Boolean value that indicates whether the app can automatically organize windows into tabs.
    fn tm_set_allows_automatic_window_tabbing(allows_automatic_window_tabbing: bool) {
        unsafe {
            msg_send![
                Self::m_class(),
                setAllowsAutomaticWindowTabbing: allows_automatic_window_tabbing
            ]
        }
    }

    /* Converting Coordinates
     */

    /// The backing scale factor.
    fn ip_backing_scale_factor(&self) -> CGFloat {
        unsafe { msg_send![self.m_self(), backingScaleFactor] }
    }

    /// Returns a backing store pixel-aligned rectangle in window coordinates.
    fn im_backing_aligned_rect_options(&self, options: NSAlignmentOptions) -> NSRect {
        unsafe { msg_send![self.m_self(), backingAlignedRect: options] }
    }

    /// Converts a rectangle from its pixel-aligned backing store coordinate system to the window’s coordinate system.
    fn im_convert_rect_from_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.m_self(), convertRectFromBacking: rect] }
    }

    /// Converts a rectangle from the screen coordinate system to the window’s coordinate system.
    fn im_convert_rect_from_screen(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.m_self(), convertRectFromScreen: rect] }
    }

    /// Converts a point from its pixel-aligned backing store coordinate system to the window’s coordinate system.
    fn im_convert_point_from_backing(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.m_self(), convertPointFromBacking: point] }
    }

    /// Converts a point from the screen coordinate system to the window’s coordinate system.
    fn im_convert_point_from_screen(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.m_self(), convertPointFromScreen: point] }
    }

    /// Converts a rectangle from the window’s coordinate system to its pixel-aligned backing store coordinate system.
    fn im_convert_rect_to_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.m_self(), convertRectToBacking: rect] }
    }

    /// Converts a rectangle to the screen coordinate system from the window’s coordinate system.
    fn im_convert_rect_to_screen(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.m_self(), convertRectToScreen: rect] }
    }

    /// Converts a point from the window’s coordinate system to its pixel-aligned backing store coordinate system.
    fn im_convert_point_to_backing(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.m_self(), convertPointToBacking: point] }
    }

    /// Converts a point to the screen coordinate system from the window’s coordinate system.
    fn im_convert_point_to_screen(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.m_self(), convertPointToScreen: point] }
    }

    /* Managing Titles
     */

    /// The string that appears in the title bar of the window or the path to the represented file.
    fn ip_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), title]) }
    }

    /// Sets the string that appears in the title bar of the window or the path to the represented file.
    ///
    /// # Arguments
    ///
    /// * `title` - The string that appears in the title bar of the window or the path to the represented file.
    fn ip_set_title(&self, title: NSString) {
        unsafe { msg_send![self.m_self(), setTitle: title] }
    }

    /// A secondary line of text that appears in the title bar of the window.
    fn ip_subtitle(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), subtitle]) }
    }

    /// Sets a secondary line of text that appears in the title bar of the window.
    ///
    /// # Arguments
    ///
    /// * `subtitle` - A secondary line of text that appears in the title bar of the window.
    fn ip_set_subtitle(&self, subtitle: NSString) {
        unsafe { msg_send![self.m_self(), setSubtitle: subtitle] }
    }

    /// A value that indicates the visibility of the window’s title and title bar buttons.
    fn ip_title_visibility(&self) -> NSWindowTitleVisibility {
        unsafe { msg_send![self.m_self(), titleVisibility] }
    }

    /// Sets a value that indicates the visibility of the window’s title and title bar buttons.
    ///
    /// # Arguments
    ///
    /// * `title_visibility` - A value that indicates the visibility of the window’s title and title bar buttons.
    fn ip_set_title_visibility(&self, title_visibility: NSWindowTitleVisibility) {
        unsafe { msg_send![self.m_self(), setTitleVisibility: title_visibility] }
    }

    /// Sets a given path as the window’s title, formatting it as a file-system path, and records this path as the window’s associated file.
    fn im_set_title_with_represented_filename(&self, path: NSString) {
        unsafe { msg_send![self.m_self(), setTitleWithRepresentedFilename: path] }
    }

    /// The path to the file of the window’s represented file.
    fn ip_represented_filename(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), representedFilename]) }
    }

    /// Sets the path to the file of the window’s represented file.
    ///
    /// # Arguments
    ///
    /// * `represented_filename` - The path to the file of the window’s represented file.
    fn ip_set_represented_filename(&self, represented_filename: NSString) {
        unsafe { msg_send![self.m_self(), setRepresentedFilename: represented_filename] }
    }

    /* Accessing Screen Information
     */

    /// The screen the window is on.
    fn ip_screen(&self) -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![self.m_self(), screen]) }
    }

    /// The deepest screen the window is on (it may be split over several screens).
    fn ip_deepest_screen(&self) -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![self.m_self(), deepestScreen]) }
    }

    /// A Boolean value that indicates whether the window context should be updated when the screen profile changes or when the window moves to a different screen.
    fn ip_displays_when_screen_profile_changes(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), displaysWhenScreenProfileChanges]) }
    }

    /// A Boolean value that indicates whether the window is movable by clicking and dragging anywhere in its background.
    fn ip_movable_by_window_background(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), movableByWindowBackground]) }
    }

    /// Sets a Boolean value that indicates whether the window is movable by clicking and dragging anywhere in its background.
    ///
    /// # Arguments
    ///
    /// * `movable_by_window_background` - A Boolean value that indicates whether the window is movable by clicking and dragging anywhere in its background.
    fn ip_set_movable_by_window_background(&self, movable_by_window_background: bool) {
        unsafe {
            msg_send![
                self.m_self(),
                setMovableByWindowBackground: movable_by_window_background
            ]
        }
    }

    /// A Boolean value that indicates whether the window can be dragged by clicking in its title bar or background.
    fn ip_movable(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isMovable]) }
    }

    /// Sets a Boolean value that indicates whether the window can be dragged by clicking in its title bar or background.
    ///
    /// # Arguments
    ///
    /// * `movable` - A Boolean value that indicates whether the window can be dragged by clicking in its title bar or background.
    fn ip_set_movable(&self, movable: bool) {
        unsafe { msg_send![self.m_self(), setMovable: movable] }
    }

    /// Sets the window’s location to the center of the screen.
    fn im_center(&self) {
        unsafe { msg_send![self.m_self(), center] }
    }

    /* Closing Windows
     */

    /// Simulates the user clicking the close button by momentarily highlighting the button and then closing the window.
    fn im_perform_close(&self, sender: id) {
        unsafe { msg_send![self.m_self(), performClose: sender] }
    }

    /// Removes the window from the screen.
    fn im_close(&self) {
        unsafe { msg_send![self.m_self(), close] }
    }

    /// A Boolean value that indicates whether the window is released when it receives the close message.
    fn ip_released_when_closed(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isReleasedWhenClosed]) }
    }

    /// Sets a Boolean value that indicates whether the window is released when it receives the close message.
    fn ip_set_released_when_closed(&self, released_when_closed: bool) {
        unsafe { msg_send![self.m_self(), setReleasedWhenClosed: released_when_closed] }
    }

    /* Minimizing Windows
     */

    /// A Boolean value that indicates whether the window is minimized.
    fn ip_miniaturized(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isMiniaturized]) }
    }

    /// Simulates the user clicking the minimize button by momentarily highlighting the button, then minimizing the window.
    fn im_perform_miniaturize(&self, sender: id) {
        unsafe { msg_send![self.m_self(), performMiniaturize: sender] }
    }

    /// Removes the window from the screen list and displays the minimized window in the Dock.
    fn im_miniaturize(&self, sender: id) {
        unsafe { msg_send![self.m_self(), miniaturize: sender] }
    }

    /// De-minimizes the window.
    fn im_deminiaturize(&self, sender: id) {
        unsafe { msg_send![self.m_self(), deminiaturize: sender] }
    }

    /// The custom miniaturized window image of the window.
    fn ip_miniwindow_image(&self) -> NSImage {
        unsafe { NSImage::from_id(msg_send![self.m_self(), miniwindowImage]) }
    }

    /// The title displayed in the window’s minimized window.
    fn ip_miniwindow_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), miniwindowTitle]) }
    }

    /* Getting the Dock Tile
     */

    /// The application’s Dock tile.
    fn ip_dock_tile(&self) -> NSDockTile {
        unsafe { NSDockTile::from_id(msg_send![self.m_self(), dockTile]) }
    }

    /* Printing Windows
     */

    /// Runs the Print panel, and if the user chooses an option other than canceling, prints the window (its frame view and all subviews).
    fn im_print(&self, sender: id) {
        unsafe { msg_send![self.m_self(), print: sender] }
    }

    /// Returns EPS data that draws the region of the window within a given rectangle.
    fn im_data_with_esp_inside_rect(&self, rect: NSRect) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), dataWithEPSInsideRect: rect]) }
    }

    /// Returns PDF data that draws the region of the window within a given rectangle.
    fn im_data_with_pdf_inside_rect(&self, rect: NSRect) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), dataWithPDFInsideRect: rect]) }
    }

    /* Handling Window Restoration
     */

    /// A Boolean value indicating whether the window configuration is preserved between application launches.
    fn ip_restorable(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isRestorable]) }
    }

    /// Sets a Boolean value indicating whether the window configuration is preserved between application launches.
    ///
    /// # Arguments
    ///
    /// * `restorable` - A Boolean value indicating whether the window configuration is preserved between application launches.
    fn ip_set_restorable(&self, restorable: bool) {
        unsafe { msg_send![self.m_self(), setRestorable: restorable] }
    }

    /// Disables snapshot restoration.
    fn im_disable_snapshot_restoration(&self) {
        unsafe { msg_send![self.m_self(), disableSnapshotRestoration] }
    }

    /// Enables snapshot restoration.
    fn im_enable_snapshot_restoration(&self) {
        unsafe { msg_send![self.m_self(), enableSnapshotRestoration] }
    }
}

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
