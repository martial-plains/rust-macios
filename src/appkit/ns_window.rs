use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::ShareId;

use crate::{
    core_graphics::{CGFloat, CGPoint, CGSize},
    foundation::{
        Int, NSAlignmentOptions, NSArray, NSData, NSDictionary, NSPoint, NSRect, NSRectEdge,
        NSSize, NSString, UInt,
    },
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{
    ns_window_delegate::register_window_class_with_delegate, INSResponder, INSWindow,
    NSBackingStoreType, NSButton, NSColor, NSColorSpace, NSDockTile, NSImage, NSModalResponse,
    NSScreen, NSTitlebarSeparatorStyle, NSToolbar, NSUserInterfaceLayoutDirection,
    NSViewController, NSWindowDepth, NSWindowFrameAutosaveName, NSWindowLevel,
    NSWindowOcclusionState, NSWindowPersistableFrameDescriptor, NSWindowStyleMask,
    NSWindowTitleVisibility, NSWindowToolbarStyle, PNSWindowDelegate,
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

            (&mut *ptr).set_ivar(WINDOW_DELEGATE_PTR, delegate_ptr as usize);

            let mut window = NSWindow::from_id(ptr);

            window = window.autorelease();

            window.ip_set_released_when_closed(false);

            window.ip_set_delegate(msg_send![&*window.ptr, self]);
            window.ip_set_restorable(false);

            window.ip_set_toolbar_style(config.toolbar_style);

            window
        };

        {
            (&mut delegate).did_load(NSWindow {
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

    /// Make the this window the key window and bring it to the front. Calling `show` does this for
    /// you.
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

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isEqual:&*object.ptr]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isKindOfClass: class]) }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMemberOfClass: class]) }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, respondsToSelector: selector]) }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, conformsToProtocol: protocol]) }
    }

    fn ip_description(&self) -> NSString {
        unsafe { msg_send![&*self.ptr, description] }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { msg_send![&*self.ptr, debugDescription] }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isProxy]) }
    }
}

impl<T> INSResponder for NSWindow<T> {}

impl<T> INSWindow for NSWindow<T> {
    fn im_init_with_content_rect_style_mask_backing_defer(
        &self,
        content_rect: NSRect,
        style: UInt,
        backing_store_type: NSBackingStoreType,
        defer: bool,
    ) -> Self {
        unsafe {
            Self::from_id(msg_send![&*self.ptr, initWithContentRect: content_rect
                                                    styleMask: style
                                                    backing: backing_store_type
                                                    defer: defer])
        }
    }

    fn im_init_with_content_rect_style_mask_backing_defer_screen(
        &self,
        content_rect: NSRect,
        style: UInt,
        backing_store_type: NSBackingStoreType,
        flag: bool,
        screen: NSScreen,
    ) -> Self {
        unsafe {
            Self::from_id(msg_send![&*self.ptr, initWithContentRect: content_rect
                                                    styleMask: style
                                                    backing: backing_store_type
                                                    defer: flag
                                                    screen: screen])
        }
    }

    fn ip_delegate(&self) -> id {
        unsafe { msg_send![&*self.ptr, delegate] }
    }

    fn ip_set_delegate(&self, delegate: NSWindow) {
        unsafe { msg_send![&*self.ptr, setDelegate: delegate] }
    }

    fn ip_content_view_controller(&self) -> NSViewController {
        unsafe { msg_send![&*self.ptr, contentViewController] }
    }

    fn ip_style_mask(&self) -> NSWindowStyleMask {
        unsafe { msg_send![&*self.ptr, styleMask] }
    }

    fn ip_set_style_mask(&mut self, style_mask: NSWindowStyleMask) {
        unsafe { msg_send![&*self.ptr, setStyleMask: style_mask] }
    }

    fn im_toggle_full_screen(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, toggleFullScreen: sender] }
    }

    fn ip_works_when_modal(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, worksWhenModal]) }
    }

    fn ip_alpha_value(&self) -> CGFloat {
        unsafe { msg_send![&*self.ptr, alphaValue] }
    }

    fn ip_set_alpha_value(&mut self, alpha_value: CGFloat) {
        unsafe { msg_send![&*self.ptr, setAlphaValue: alpha_value] }
    }

    fn ip_background_color(&self) -> NSColor {
        unsafe { msg_send![&*self.ptr, backgroundColor] }
    }

    fn ip_set_background_color(&mut self, color: NSColor) {
        unsafe { msg_send![&*self.ptr, setBackgroundColor: color] }
    }

    fn ip_color_space(&self) -> NSColorSpace {
        unsafe { msg_send![&*self.ptr, colorSpace] }
    }

    fn ip_set_color_space(&mut self, color_space: NSColorSpace) {
        unsafe { msg_send![&*self.ptr, setColorSpace: color_space] }
    }

    fn im_set_dynamic_depth_limit(&self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setDynamicDepthLimit: flag] }
    }

    fn ip_can_hide(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, canHide]) }
    }

    fn ip_set_can_hide(&mut self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setCanHide: flag] }
    }

    fn ip_is_on_active_space(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isOnActiveSpace]) }
    }

    fn ip_hides_on_deactivate(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, hidesOnDeactivate]) }
    }

    fn ip_set_hides_on_deactivate(&mut self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setHidesOnDeactivate: flag] }
    }

    fn ip_collection_behavior(&self) -> super::NSWindowCollectionBehavior {
        unsafe { msg_send![&*self.ptr, collectionBehavior] }
    }

    fn ip_is_opaque(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isOpaque]) }
    }

    fn ip_set_opaque(&mut self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setOpaque: flag] }
    }

    fn ip_has_shadow(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, hasShadow]) }
    }

    fn ip_set_has_shadow(&mut self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setHasShadow: flag] }
    }

    fn im_invalidate_shadow(&self) {
        unsafe { msg_send![&*self.ptr, invalidateShadow] }
    }

    fn im_autorecalculates_content_border_thickness_for_edge(&self, edge: NSRectEdge) -> bool {
        unsafe {
            to_bool(msg_send![
                &*self.ptr,
                autorecalculatesContentBorderThicknessForEdge: edge
            ])
        }
    }

    fn im_set_autorecalculates_content_border_thickness_for_edge(
        &self,
        flag: bool,
        edge: NSRectEdge,
    ) {
        unsafe {
            msg_send![
                &*self.ptr,
                setAutorecalculatesContentBorderThickness: flag
                forEdge: edge
            ]
        }
    }

    fn im_content_border_thickness_for_edge(&self, edge: NSRectEdge) -> CGFloat {
        unsafe { msg_send![&*self.ptr, contentBorderThicknessForEdge: edge] }
    }

    fn im_set_content_border_thickness_for_edge(&self, thickness: CGFloat, edge: NSRectEdge) {
        unsafe {
            msg_send![
                &*self.ptr,
                setContentBorderThickness: thickness
                forEdge: edge
            ]
        }
    }

    fn ip_prevents_application_termination(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, preventsApplicationTermination]) }
    }

    fn ip_appearance_source(&self) -> id {
        unsafe { msg_send![&*self.ptr, appearanceSource] }
    }

    fn ip_depth_limit(&self) -> NSWindowDepth {
        unsafe { msg_send![&*self.ptr, depthLimit] }
    }

    fn ip_set_depth_limit(&mut self, depth: NSWindowDepth) {
        unsafe { msg_send![&*self.ptr, setDepthLimit: depth] }
    }

    fn ip_has_dynamic_depth_limit(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, hasDynamicDepthLimit]) }
    }

    fn ip_window_number(&self) -> Int {
        unsafe { msg_send![&*self.ptr, windowNumber] }
    }

    fn ip_device_description(&self) -> NSDictionary<super::NSDeviceDescriptionKey, id> {
        unsafe { msg_send![&*self.ptr, deviceDescription] }
    }

    fn ip_can_become_visible_without_login(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, canBecomeVisibleWithoutLogin]) }
    }

    fn ip_sharing_type(&self) -> super::NSWindowSharingType {
        unsafe { msg_send![&*self.ptr, sharingType] }
    }

    fn ip_backing_type(&self) -> NSBackingStoreType {
        unsafe { msg_send![&*self.ptr, backingType] }
    }

    fn im_content_rect_for_frame_rect(&self, frame: NSRect) -> NSRect {
        unsafe { msg_send![&*self.ptr, contentRectForFrameRect: frame] }
    }

    fn im_frame_rect_for_content_rectim_frame_rect_for_content_rect(
        &self,
        content: NSRect,
    ) -> NSRect {
        unsafe { msg_send![&*self.ptr, frameRectForContentRect: content] }
    }

    fn ip_attached_sheet(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![&*self.ptr, attachedSheet]) }
    }

    fn ip_sheet(&mut self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isSheet]) }
    }

    fn im_begin_sheet_completion_handler<W>(
        &self,
        sheet: &NSWindow<W>,
        handler: block::RcBlock<(i64,), ()>,
    ) {
        unsafe {
            msg_send![
                &*self.ptr,
                beginSheet: sheet
                completionHandler: handler
            ]
        }
    }

    fn im_begin_critical_sheet_completion_handler(
        &self,
        sheet: NSWindow,
        handler: block::RcBlock<NSModalResponse, ()>,
    ) {
        unsafe {
            msg_send![
                &*self.ptr,
                beginCriticalSheet: &*sheet.ptr
                completionHandler: handler
            ]
        }
    }

    fn im_end_sheet<W>(&self, sheet: &NSWindow<W>)
    where
        W: PNSWindowDelegate + 'static,
    {
        unsafe { msg_send![&*self.ptr, endSheet:&*sheet.ptr] }
    }

    fn im_end_sheet_with_return_code(&self, sheet: NSWindow, code: NSModalResponse) {
        unsafe { msg_send![&*self.ptr, endSheet: sheet returnCode: code] }
    }

    fn ip_sheet_parent(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![&*self.ptr, sheetParent]) }
    }

    fn ip_sheets(&self) -> NSArray<NSWindow> {
        unsafe { NSArray::from_id(msg_send![&*self.ptr, sheets]) }
    }

    fn ip_frame(&self) -> NSRect {
        unsafe { msg_send![&*self.ptr, frame] }
    }

    fn im_set_frame_origin(&self, point: NSPoint) {
        unsafe { msg_send![&*self.ptr, setFrameOrigin: point] }
    }

    fn im_set_frame_top_left_point(&self, point: NSPoint) {
        unsafe { msg_send![&*self.ptr, setFrameTopLeftPoint: point] }
    }

    fn im_constrain_frame_rect_to_screen(&self, frame: NSRect, screen: NSScreen) -> NSRect {
        unsafe { msg_send![&*self.ptr, constrainFrameRect: frame toScreen: screen] }
    }

    fn im_cascade_top_left_from_point(&self, point: NSPoint) {
        unsafe { msg_send![&*self.ptr, cascadeTopLeftFromPoint: point] }
    }

    fn im_set_frame_display(&self, frame: NSRect, flag: bool) {
        unsafe { msg_send![&*self.ptr, setFrame: frame display: flag] }
    }

    fn im_set_frame_display_animate(&self, frame: NSRect, flag: bool, animate: bool) {
        unsafe { msg_send![&*self.ptr, setFrame: frame display: flag animate: animate] }
    }

    fn im_animation_resize_time(&self, frame: NSRect) -> crate::foundation::NSTimeInterval {
        unsafe { msg_send![&*self.ptr, animationResizeTime: frame] }
    }

    fn ip_aspect_ratio(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, aspectRatio] }
    }

    fn ip_set_aspect_ratio(&self, ratio: NSSize) {
        unsafe { msg_send![&*self.ptr, setAspectRatio: ratio] }
    }

    fn ip_min_size(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, minSize] }
    }

    fn ip_set_min_size(&self, size: NSSize) {
        unsafe { msg_send![&*self.ptr, setMinSize: size] }
    }

    fn ip_max_size(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, maxSize] }
    }

    fn ip_set_max_size(&self, size: NSSize) {
        unsafe { msg_send![&*self.ptr, setMaxSize: size] }
    }

    fn ip_zoomed(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isZoomed]) }
    }

    fn im_perform_zoom(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, performZoom: sender] }
    }

    fn im_zoom(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, zoom: sender] }
    }

    fn ip_resize_increments(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, resizeIncrements] }
    }

    fn ip_set_resize_increments(&self, increments: NSSize) {
        unsafe { msg_send![&*self.ptr, setResizeIncrements: increments] }
    }

    fn ip_preserves_content_during_live_resize(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, preservesContentDuringLiveResize]) }
    }

    fn ip_set_preserves_content_during_live_resize(&self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setPreservesContentDuringLiveResize: flag] }
    }

    fn ip_in_live_resize(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, inLiveResize]) }
    }

    fn ip_set_in_live_resize(&self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setInLiveResize: flag] }
    }

    fn ip_content_aspect_ratio(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, contentAspectRatio] }
    }

    fn ip_set_content_aspect_ratio(&self, size: NSSize) {
        unsafe { msg_send![&*self.ptr, setContentAspectRatio: size] }
    }

    fn ip_content_min_size(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, contentMinSize] }
    }

    fn ip_set_content_min_size(&self, content_min_size: NSSize) {
        unsafe { msg_send![&*self.ptr, setContentMinSize: content_min_size] }
    }

    fn im_set_content_size(&self, size: NSSize) {
        unsafe { msg_send![&*self.ptr, setContentSize: size] }
    }

    fn ip_content_max_size(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, contentMaxSize] }
    }

    fn ip_set_content_max_size(&self, size: NSSize) {
        unsafe { msg_send![&*self.ptr, setContentMaxSize: size] }
    }

    fn ip_content_resize_increments(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, contentResizeIncrements] }
    }

    fn ip_set_content_resize_increments(&self, content_resize_increments: NSSize) {
        unsafe {
            msg_send![
                &*self.ptr,
                setContentResizeIncrements: content_resize_increments
            ]
        }
    }

    fn ip_content_layout_guide(&self) -> id {
        unsafe { msg_send![&*self.ptr, contentLayoutGuide] }
    }

    fn ip_set_content_layout_guide(&self, content_layout_guide: id) {
        unsafe { msg_send![&*self.ptr, setContentLayoutGuide: content_layout_guide] }
    }

    fn ip_content_layout_rect(&self) -> NSRect {
        unsafe { msg_send![&*self.ptr, contentLayoutRect] }
    }

    fn ip_set_content_layout_rect(&self, content_layout_rect: NSRect) {
        unsafe { msg_send![&*self.ptr, setContentLayoutRect: content_layout_rect] }
    }

    fn ip_full_screen_tile_size(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, fullScreenTileSize] }
    }

    fn ip_set_full_screen_tile_size(&self, size: NSSize) {
        unsafe { msg_send![&*self.ptr, setFullScreenTileSize: size] }
    }

    fn ip_full_screen_tile_content_size(&self) -> NSSize {
        unsafe { msg_send![&*self.ptr, fullScreenTileContentSize] }
    }

    fn ip_set_full_screen_tile_content_size(&self, size: NSSize) {
        unsafe { msg_send![&*self.ptr, setFullScreenTileContentSize: size] }
    }

    fn im_order_out(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, orderOut: sender] }
    }

    fn im_order_back(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, orderBack: sender] }
    }

    fn im_order_front(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, orderFront: sender] }
    }

    fn im_order_front_regardless(&self) {
        unsafe { msg_send![&*self.ptr, orderFrontRegardless] }
    }

    fn im_order_window_relative_to(&self, place: super::NSWindowOrderingMode, other: Int) {
        unsafe { msg_send![&*self.ptr, orderWindow: place relativeTo: other] }
    }

    fn ip_window_level(&self) -> NSWindowLevel {
        unsafe { msg_send![&*self.ptr, level] }
    }

    fn ip_set_window_level(&self, level: NSWindowLevel) {
        unsafe { msg_send![&*self.ptr, setLevel: level] }
    }

    fn ip_visible(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isVisible]) }
    }

    fn ip_occlusion_state(&self) -> NSWindowOcclusionState {
        unsafe { msg_send![&*self.ptr, occlusionState] }
    }

    fn im_set_frame_using_name(&self, name: NSWindowFrameAutosaveName) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, setFrameUsingName: name]) }
    }

    fn im_set_frame_using_name_force(&self, name: NSWindowFrameAutosaveName, force: bool) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, setFrameUsingName:name force:force]) }
    }

    fn im_save_frame_using_name(&self, name: NSWindowFrameAutosaveName) {
        unsafe { msg_send![&*self.ptr, saveFrameUsingName: name] }
    }

    fn im_set_frame_autosave_name(&self, name: NSWindowFrameAutosaveName) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, setFrameAutosaveName: name]) }
    }

    fn ip_frame_autosave_name(&self) -> NSWindowFrameAutosaveName {
        unsafe { msg_send![&*self.ptr, frameAutosaveName] }
    }

    fn ip_string_with_saved_frame(&self) -> NSWindowPersistableFrameDescriptor {
        unsafe { msg_send![&*self.ptr, stringWithSavedFrame] }
    }

    fn im_set_frame_from_string(&self, string: NSWindowPersistableFrameDescriptor) {
        unsafe { msg_send![&*self.ptr, setFrameFromString: string] }
    }

    fn ip_key_window(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isKeyWindow]) }
    }

    fn ip_can_become_key_window(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, canBecomeKeyWindow]) }
    }

    fn im_make_key_window(&self) {
        unsafe { msg_send![&*self.ptr, makeKeyWindow] }
    }

    fn im_make_key_and_order_front(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, makeKeyAndOrderFront: sender] }
    }

    fn im_become_key_window(&self) {
        unsafe { msg_send![&*self.ptr, becomeKeyWindow] }
    }

    fn im_resign_key_window(&self) {
        unsafe { msg_send![&*self.ptr, resignKeyWindow] }
    }

    fn ip_main_window(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMainWindow]) }
    }

    fn ip_can_become_main_window(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, canBecomeMainWindow]) }
    }

    fn im_make_main_window(&self) {
        unsafe { msg_send![&*self.ptr, makeMainWindow] }
    }

    fn im_become_main_window(&self) {
        unsafe { msg_send![&*self.ptr, becomeMainWindow] }
    }

    fn im_resign_main_window(&self) {
        unsafe { msg_send![&*self.ptr, resignMainWindow] }
    }

    fn ip_toolbar(&self) -> NSToolbar {
        unsafe { NSToolbar::from_id(msg_send![&*self.ptr, toolbar]) }
    }

    fn ip_set_toolbar(&self, toolbar: NSToolbar) {
        unsafe { msg_send![&*self.ptr, setToolbar: toolbar] }
    }

    fn ip_toggle_toolbar_shown(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, toggleToolbarShown: sender] }
    }

    fn ip_run_toolbar_customization_palette(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, runToolbarCustomizationPalette: sender] }
    }

    fn ip_child_windows(&self) -> id {
        unsafe { msg_send![&*self.ptr, childWindows] }
    }

    fn add_child_window_ordered(&self, child: id, order: super::NSWindowOrderingMode) {
        unsafe { msg_send![&*self.ptr, addChildWindow:child ordered:order] }
    }

    fn remove_child_window(&self, child: id) {
        unsafe { msg_send![&*self.ptr, removeChildWindow: child] }
    }

    fn ip_parent_window(&self) -> id {
        unsafe { msg_send![&*self.ptr, parentWindow] }
    }

    fn ip_excluded_from_windows_menu(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isExcludedFromWindowsMenu]) }
    }

    fn ip_set_excluded_from_windows_menu(&self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setExcludedFromWindowsMenu: flag] }
    }

    fn ip_are_cursor_rects_enabled(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, areCursorRectsEnabled]) }
    }

    fn im_enable_cursor_rects(&self) {
        unsafe { msg_send![&*self.ptr, enableCursorRects] }
    }

    fn im_disable_cursor_rects(&self) {
        unsafe { msg_send![&*self.ptr, disableCursorRects] }
    }

    fn im_discard_cursor_rects(&self, view: id) {
        unsafe { msg_send![&*self.ptr, discardCursorRectsForView: view] }
    }

    fn im_reset_cursor_rects(&self) {
        unsafe { msg_send![&*self.ptr, resetCursorRects] }
    }

    fn ip_standard_window_button(&self, b: super::NSWindowButton) -> NSButton {
        unsafe { NSButton::from_id(msg_send![&*self.ptr, standardWindowButton: b]) }
    }

    fn ip_shows_toolbar_button(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, showsToolbarButton]) }
    }

    fn ip_set_shows_toolbar_button(&self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setShowsToolbarButton: flag] }
    }

    fn ip_titlebar_appears_transparent(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, titlebarAppearsTransparent]) }
    }

    fn ip_set_titlebar_appears_transparent(&self, flag: bool) {
        unsafe { msg_send![&*self.ptr, setTitlebarAppearsTransparent: flag] }
    }

    fn ip_toolbar_style(&self) -> NSWindowToolbarStyle {
        unsafe { msg_send![&*self.ptr, toolbarStyle] }
    }

    fn ip_set_toolbar_style(&self, style: NSWindowToolbarStyle) {
        unsafe { msg_send![&*self.ptr, setToolbarStyle: style] }
    }

    fn ip_titlebar_separator_style(&self) -> NSTitlebarSeparatorStyle {
        unsafe { msg_send![&*self.ptr, titlebarSeparatorStyle] }
    }

    fn ip_set_titlebar_separator_style(&self, style: NSTitlebarSeparatorStyle) {
        unsafe { msg_send![&*self.ptr, setTitlebarSeparatorStyle: style] }
    }

    fn ip_titlebar_layout_direction(&self) -> NSUserInterfaceLayoutDirection {
        unsafe { msg_send![&*self.ptr, titlebarLayoutDirection] }
    }

    fn ip_backing_scale_factor(&self) -> CGFloat {
        unsafe { msg_send![&*self.ptr, backingScaleFactor] }
    }

    fn im_backing_aligned_rect_options(&self, options: NSAlignmentOptions) -> NSRect {
        unsafe { msg_send![&*self.ptr, backingAlignedRect: options] }
    }

    fn im_convert_rect_from_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![&*self.ptr, convertRectFromBacking: rect] }
    }

    fn im_convert_rect_from_screen(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![&*self.ptr, convertRectFromScreen: rect] }
    }

    fn im_convert_point_from_backing(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![&*self.ptr, convertPointFromBacking: point] }
    }

    fn im_convert_point_from_screen(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![&*self.ptr, convertPointFromScreen: point] }
    }

    fn im_convert_rect_to_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![&*self.ptr, convertRectToBacking: rect] }
    }

    fn im_convert_rect_to_screen(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![&*self.ptr, convertRectToScreen: rect] }
    }

    fn im_convert_point_to_backing(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![&*self.ptr, convertPointToBacking: point] }
    }

    fn im_convert_point_to_screen(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![&*self.ptr, convertPointToScreen: point] }
    }

    fn ip_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, title]) }
    }

    fn ip_set_title(&self, title: NSString) {
        unsafe { msg_send![&*self.ptr, setTitle: title] }
    }

    fn ip_subtitle(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, subtitle]) }
    }

    fn ip_set_subtitle(&self, subtitle: NSString) {
        unsafe { msg_send![&*self.ptr, setSubtitle: subtitle] }
    }

    fn ip_title_visibility(&self) -> NSWindowTitleVisibility {
        unsafe { msg_send![&*self.ptr, titleVisibility] }
    }

    fn ip_set_title_visibility(&self, title_visibility: NSWindowTitleVisibility) {
        unsafe { msg_send![&*self.ptr, setTitleVisibility: title_visibility] }
    }

    fn im_set_title_with_represented_filename(&self, path: NSString) {
        unsafe { msg_send![&*self.ptr, setTitleWithRepresentedFilename: path] }
    }

    fn ip_represented_filename(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, representedFilename]) }
    }

    fn ip_set_represented_filename(&self, represented_filename: NSString) {
        unsafe { msg_send![&*self.ptr, setRepresentedFilename: represented_filename] }
    }

    fn ip_screen(&self) -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![&*self.ptr, screen]) }
    }

    fn ip_deepest_screen(&self) -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![&*self.ptr, deepestScreen]) }
    }

    fn ip_displays_when_screen_profile_changes(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, displaysWhenScreenProfileChanges]) }
    }

    fn ip_movable_by_window_background(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMovableByWindowBackground]) }
    }

    fn ip_set_movable_by_window_background(&self, movable_by_window_background: bool) {
        unsafe {
            msg_send![
                &*self.ptr,
                setMovableByWindowBackground: movable_by_window_background
            ]
        }
    }

    fn ip_movable(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMovable]) }
    }

    fn ip_set_movable(&self, movable: bool) {
        unsafe { msg_send![&*self.ptr, setMovable: movable] }
    }

    fn im_center(&self) {
        unsafe { msg_send![&*self.ptr, center] }
    }

    fn im_perform_close(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, performClose: sender] }
    }

    fn im_close(&self) {
        unsafe { msg_send![&*self.ptr, close] }
    }

    fn ip_released_when_closed(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isReleasedWhenClosed]) }
    }

    fn ip_set_released_when_closed(&self, released_when_closed: bool) {
        unsafe { msg_send![&*self.ptr, setReleasedWhenClosed: released_when_closed] }
    }

    fn ip_miniaturized(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMiniaturized]) }
    }

    fn im_perform_miniaturize(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, performMiniaturize: sender] }
    }

    fn im_miniaturize(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, miniaturize: sender] }
    }

    fn im_deminiaturize(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, deminiaturize: sender] }
    }

    fn ip_miniwindow_image(&self) -> NSImage {
        unsafe { NSImage::from_id(msg_send![&*self.ptr, miniwindowImage]) }
    }

    fn ip_miniwindow_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, miniwindowTitle]) }
    }

    fn ip_dock_tile(&self) -> NSDockTile {
        unsafe { NSDockTile::from_id(msg_send![&*self.ptr, dockTile]) }
    }

    fn im_print(&self, sender: id) {
        unsafe { msg_send![&*self.ptr, print: sender] }
    }

    fn im_data_with_esp_inside_rect(&self, rect: NSRect) -> NSData {
        unsafe { NSData::from_id(msg_send![&*self.ptr, dataWithEPSInsideRect: rect]) }
    }

    fn im_data_with_pdf_inside_rect(&self, rect: NSRect) -> NSData {
        unsafe { NSData::from_id(msg_send![&*self.ptr, dataWithPDFInsideRect: rect]) }
    }

    fn ip_restorable(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isRestorable]) }
    }

    fn ip_set_restorable(&self, restorable: bool) {
        unsafe { msg_send![&*self.ptr, setRestorable: restorable] }
    }

    fn im_disable_snapshot_restoration(&self) {
        unsafe { msg_send![&*self.ptr, disableSnapshotRestoration] }
    }

    fn im_enable_snapshot_restoration(&self) {
        unsafe { msg_send![&*self.ptr, enableSnapshotRestoration] }
    }
}

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

impl<T> Drop for NSWindow<T> {
    fn drop(&mut self) {
        if self.delegate.is_some() {
            unsafe {
                self.ip_set_delegate(NSWindow::from_id(nil));
            }
        }
    }
}
