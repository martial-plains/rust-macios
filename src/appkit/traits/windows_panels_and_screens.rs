use block::RcBlock;
use objc::{msg_send, sel, sel_impl};

use crate::{
    appkit::{
        ns_appearance::NSAppearance, ns_color::NSColor, ns_window::NSWindow, NSBackingStoreType,
        NSButton, NSColorSpace, NSDeviceDescriptionKey, NSDockTile, NSImage, NSModalResponse,
        NSPopover, NSPopoverBehavior, NSScreen, NSTitlebarSeparatorStyle, NSToolbar,
        NSUserInterfaceLayoutDirection, NSViewController, NSWindowButton,
        NSWindowCollectionBehavior, NSWindowDepth, NSWindowFrameAutosaveName, NSWindowLevel,
        NSWindowNumberListOptions, NSWindowOcclusionState, NSWindowOrderingMode,
        NSWindowPersistableFrameDescriptor, NSWindowSharingType, NSWindowStyleMask,
        NSWindowTitleVisibility, NSWindowToolbarStyle,
    },
    core_graphics::{CGFloat, CGRectEdge},
    foundation::{
        Int, NSAlignmentOptions, NSArray, NSCoder, NSData, NSDictionary, NSNumber, NSPoint, NSRect,
        NSRectEdge, NSSize, NSString, NSTimeInterval, UInt,
    },
    objective_c_runtime::{id, traits::FromId},
    utils::to_bool,
};

use super::{INSResponder, INSView, INSViewController};

/// A window that an app displays on the screen.
pub trait INSWindow: INSResponder {
    /* Creating a Window
     */

    /// Creates a titled window that contains the specified content view controller.
    fn tm_window_with_content_view_controller(content_view_controller: NSViewController) -> Self
    where
        Self: Sized + 'static,
    {
        unsafe {
            msg_send![
                Self::im_class(),
                windowWithContentViewController: content_view_controller
            ]
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
                self.im_self(),
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
                self.im_self(),
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
        unsafe { msg_send![self.im_self(), delegate] }
    }

    /// Sets the window’s delegate.
    ///
    /// # Arguments
    ///
    /// * `delegate` - The delegate object.
    fn ip_set_delegate(&self, delegate: NSWindow) {
        unsafe { msg_send![self.im_self(), setDelegate: delegate] }
    }

    /* Configuring the Window's Appearance
     */

    /// The main content view controller for the window.
    fn ip_content_view_controller(&self) -> NSViewController {
        unsafe { msg_send![self.im_self(), contentViewController] }
    }

    /* Configuring the Window's Appearance
     */

    /// Flags that describe the window’s current style, such as if it’s resizable or in full-screen mode.
    fn ip_style_mask(&self) -> NSWindowStyleMask {
        unsafe { msg_send![self.im_self(), styleMask] }
    }

    /// Sets the flags that describe the window’s current style, such as if it’s resizable or in full-screen mode.
    ///
    /// # Arguments
    ///
    /// * `style_mask` - The flags that describe the window’s current style, such as if it’s resizable or in full-screen mode.
    fn ip_set_style_mask(&mut self, style_mask: NSWindowStyleMask) {
        unsafe { msg_send![self.im_self(), setStyleMask: style_mask] }
    }

    /// Takes the window into or out of fullscreen mode,
    fn im_toggle_full_screen(&self, sender: id) {
        unsafe { msg_send![self.im_self(), toggleFullScreen: sender] }
    }

    /// A Boolean value that indicates whether the window is able to receive keyboard and mouse events even when some other window is being run modally.
    fn ip_works_when_modal(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), worksWhenModal]) }
    }

    /// The window’s alpha value.
    fn ip_alpha_value(&self) -> CGFloat {
        unsafe { msg_send![self.im_self(), alphaValue] }
    }

    /// Sets the window’s alpha value.
    ///
    /// # Arguments
    ///
    /// * `value` - The window’s alpha value.
    fn ip_set_alpha_value(&mut self, value: CGFloat) {
        unsafe { msg_send![self.im_self(), setAlphaValue: value] }
    }

    /// The color of the window’s background.
    fn ip_background_color(&self) -> NSColor {
        unsafe { NSColor::from_id(msg_send![self.im_self(), backgroundColor]) }
    }

    /// Sets the color of the window’s background.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the window’s background.
    fn ip_set_background_color(&mut self, color: NSColor) {
        unsafe { msg_send![self.im_self(), setBackgroundColor: color] }
    }

    /// The window’s color space.
    fn ip_color_space(&self) -> NSColorSpace {
        unsafe { NSColorSpace::from_id(msg_send![self.im_self(), colorSpace]) }
    }

    /// Sets the window’s color space.
    ///
    /// # Arguments
    ///
    /// * `color_space` - The window’s color space.
    fn ip_set_color_space(&mut self, color_space: NSColorSpace) {
        unsafe { msg_send![self.im_self(), setColorSpace: color_space] }
    }

    /// Sets a Boolean value that indicates whether the window’s depth limit can change to match the depth of the screen it’s on.
    fn im_set_dynamic_depth_limit(&self, flag: bool) {
        unsafe { msg_send![self.im_self(), setDynamicDepthLimit: flag] }
    }

    /// A Boolean value that indicates whether the window can hide when its application becomes hidden.
    fn ip_can_hide(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), canHide]) }
    }

    /// Sets a Boolean value that indicates whether the window can hide when its application becomes hidden.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window can hide when its application becomes hidden.
    fn ip_set_can_hide(&mut self, flag: bool) {
        unsafe { msg_send![self.im_self(), setCanHide: flag] }
    }

    /// A Boolean value that indicates whether the window is on the currently active space.
    fn ip_is_on_active_space(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isOnActiveSpace]) }
    }

    /// A Boolean value that indicates whether the window is removed from the screen when its application becomes inactive.
    fn ip_hides_on_deactivate(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), hidesOnDeactivate]) }
    }

    /// Sets a Boolean value that indicates whether the window is removed from the screen when its application becomes inactive.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window is removed from the screen when its application becomes inactive.
    fn ip_set_hides_on_deactivate(&mut self, flag: bool) {
        unsafe { msg_send![self.im_self(), setHidesOnDeactivate: flag] }
    }

    /// A value that identifies the window’s behavior in window collections.
    fn ip_collection_behavior(&self) -> NSWindowCollectionBehavior {
        unsafe { msg_send![self.im_self(), collectionBehavior] }
    }

    /// A Boolean value that indicates whether the window is opaque.
    fn ip_is_opaque(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isOpaque]) }
    }

    /// Sets a Boolean value that indicates whether the window is opaque.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window is opaque.
    fn ip_set_opaque(&mut self, flag: bool) {
        unsafe { msg_send![self.im_self(), setOpaque: flag] }
    }

    /// A Boolean value that indicates whether the window has a shadow.
    fn ip_has_shadow(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), hasShadow]) }
    }

    /// Sets a Boolean value that indicates whether the window has a shadow.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window has a shadow.
    fn ip_set_has_shadow(&mut self, flag: bool) {
        unsafe { msg_send![self.im_self(), setHasShadow: flag] }
    }

    /// Invalidates the window shadow so that it is recomputed based on the current window shape.
    fn im_invalidate_shadow(&self) {
        unsafe { msg_send![self.im_self(), invalidateShadow] }
    }

    /// Indicates whether the window calculates the thickness of a given border automatically.
    fn im_autorecalculates_content_border_thickness_for_edge(&self, edge: NSRectEdge) -> bool {
        unsafe {
            to_bool(msg_send![
                self.im_self(),
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
            msg_send![self.im_self(), setAutorecalculatesContentBorderThickness: flag forEdge: edge]
        }
    }

    /// Indicates the thickness of a given border of the window.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge of the window for which to return the thickness.
    fn im_content_border_thickness_for_edge(&self, edge: NSRectEdge) -> CGFloat {
        unsafe { msg_send![self.im_self(), contentBorderThicknessForEdge: edge] }
    }

    /// Specifies the thickness of a given border of the window.
    ///
    /// # Arguments
    ///
    /// * `thickness` - The thickness of the border.
    /// * `edge` - The edge of the window for which to set the thickness.
    fn im_set_content_border_thickness_for_edge(&self, thickness: CGFloat, edge: NSRectEdge) {
        unsafe { msg_send![self.im_self(), setContentBorderThickness: thickness forEdge: edge] }
    }

    /// A Boolean value that indicates whether the window prevents application termination when modal.
    fn ip_prevents_application_termination(&self) -> bool {
        unsafe {
            to_bool(msg_send![
                self.im_self(),
                preventsApplicationTerminationWhenModal
            ])
        }
    }

    /// An object that the window inherits its appearance from.
    fn ip_appearance_source(&self) -> id {
        unsafe { msg_send![self.im_self(), appearanceSource] }
    }

    /* Accessing Window Information
     */

    /// The depth limit of the window.
    fn ip_depth_limit(&self) -> NSWindowDepth {
        unsafe { msg_send![self.im_self(), depthLimit] }
    }

    /// Sets the depth limit of the window.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth limit of the window.
    fn ip_set_depth_limit(&mut self, depth: NSWindowDepth) {
        unsafe { msg_send![self.im_self(), setDepthLimit: depth] }
    }

    /// A Boolean value that indicates whether the window’s depth limit can change to match the depth of the screen it’s on.
    fn ip_has_dynamic_depth_limit(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), hasDynamicDepthLimit]) }
    }

    /// Returns the default depth limit for instances of NSWindow.
    fn tp_default_depth_limit() -> NSWindowDepth {
        unsafe { msg_send![Self::im_class(), defaultDepthLimit] }
    }

    /// The window number of the window’s window device.
    fn ip_window_number(&self) -> Int {
        unsafe { msg_send![self.im_self(), windowNumber] }
    }

    /// Returns the window numbers for all visible windows satisfying the specified options.
    fn tm_window_numbers_with_options(options: NSWindowNumberListOptions) -> NSArray<NSNumber> {
        unsafe {
            NSArray::from_id(msg_send![
                Self::im_class(),
                windowNumbersWithOptions: options
            ])
        }
    }

    /// A dictionary containing information about the window’s resolution, such as color, depth, and so on.
    fn ip_device_description(&self) -> NSDictionary<NSDeviceDescriptionKey, id> {
        unsafe { NSDictionary::from_id(msg_send![self.im_self(), deviceDescription]) }
    }

    /// A Boolean value that indicates whether the window can be displayed at the login window.
    fn ip_can_become_visible_without_login(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), canBecomeVisibleWithoutLogin]) }
    }

    /// A Boolean value that indicates the level of access other processes have to the window’s content.
    fn ip_sharing_type(&self) -> NSWindowSharingType {
        unsafe { msg_send![self.im_self(), sharingType] }
    }

    /// The window’s backing store type.
    fn ip_backing_type(&self) -> NSBackingStoreType {
        unsafe { msg_send![self.im_self(), backingType] }
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
        unsafe { msg_send![Self::im_class(), contentRectForFrameRect: frame styleMask: style] }
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
        unsafe { msg_send![Self::im_class(), frameRectForContentRect: content styleMask: style] }
    }

    /// Returns the minimum width a window’s frame rectangle must have for it to display a title, with a given window style.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the window.
    /// * `style` - The window style.
    fn tm_min_frame_width_with_title_style_mask(title: &str, style: NSWindowStyleMask) -> CGFloat {
        unsafe { msg_send![Self::im_class(), minFrameWidthWithTitle: title styleMask: style] }
    }

    /// Returns the window’s content rectangle with a given frame rectangle.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle of the window.
    fn im_content_rect_for_frame_rect(&self, frame: NSRect) -> NSRect {
        unsafe { msg_send![self.im_self(), contentRectForFrameRect: frame] }
    }

    /// Returns the window’s frame rectangle with a given content rectangle.
    fn im_frame_rect_for_content_rectim_frame_rect_for_content_rect(
        &self,
        content: NSRect,
    ) -> NSRect {
        unsafe { msg_send![self.im_self(), frameRectForContentRect: content] }
    }

    /* Managing Sheets
     */

    /// The sheet attached to the window.
    fn ip_attached_sheet(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![self.im_self(), attachedSheet]) }
    }

    /// The sheet attached to the window.
    fn ip_sheet(&mut self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isSheet]) }
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
        unsafe { msg_send![self.im_self(), beginSheet: sheet completionHandler: handler] }
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
        unsafe { msg_send![self.im_self(), beginCriticalSheet: sheet completionHandler: handler] }
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
        unsafe { msg_send![self.im_self(), endSheet: sheet] }
    }

    /// Ends a document-modal session and dismisses the specified sheet.
    ///
    /// # Arguments
    ///
    /// * `sheet` - The sheet to dismiss.
    /// * `code` - The return code of the sheet.
    fn im_end_sheet_with_return_code(&self, sheet: NSWindow, code: NSModalResponse) {
        unsafe { msg_send![self.im_self(), endSheet: sheet returnCode: code] }
    }

    /// The window to which the sheet is attached.
    fn ip_sheet_parent(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![self.im_self(), sheetParent]) }
    }

    /// An array of the sheets currently attached to the window.
    fn ip_sheets(&self) -> NSArray<NSWindow> {
        unsafe { NSArray::from_id(msg_send![self.im_self(), sheets]) }
    }

    /*Sizing Windows
     */

    /// The window’s frame rectangle in screen coordinates, including the title bar.
    fn ip_frame(&self) -> NSRect {
        unsafe { msg_send![self.im_self(), frame] }
    }

    /// Positions the bottom-left corner of the window’s frame rectangle at a given point in screen coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - The point at which to position the bottom-left corner of the window’s frame rectangle.
    fn im_set_frame_origin(&self, point: NSPoint) {
        unsafe { msg_send![self.im_self(), setFrameOrigin: point] }
    }

    /// Positions the top-left corner of the window’s frame rectangle at a given point in screen coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - The point at which to position the top-left corner of the window’s frame rectangle.
    fn im_set_frame_top_left_point(&self, point: NSPoint) {
        unsafe { msg_send![self.im_self(), setFrameTopLeftPoint: point] }
    }

    /// Modifies and returns a frame rectangle so that its top edge lies on a specific screen.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle to modify.
    /// * `screen` - The screen on which to position the top edge of the frame rectangle.
    fn im_constrain_frame_rect_to_screen(&self, frame: NSRect, screen: NSScreen) -> NSRect {
        unsafe { msg_send![self.im_self(), constrainFrameRect: frame toScreen: screen] }
    }

    /// Positions the window’s top-left to a given point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point at which to position the top-left corner of the window.
    fn im_cascade_top_left_from_point(&self, point: NSPoint) {
        unsafe { msg_send![self.im_self(), cascadeTopLeftFromPoint: point] }
    }

    /// Sets the origin and size of the window’s frame rectangle according to a given frame rectangle, thereby setting its position and size onscreen.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle to set.
    /// * `flag` - When `true` the window sends a `displayIfNeeded` message down its view hierarchy, thus redrawing all views.
    fn im_set_frame_display(&self, frame: NSRect, flag: bool) {
        unsafe { msg_send![self.im_self(), setFrame: frame display: flag] }
    }

    /// Sets the origin and size of the window’s frame rectangle, with optional animation, according to a given frame rectangle, thereby setting its position and size onscreen.
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame rectangle to set.
    /// * `flag` - When `true` the window sends a `displayIfNeeded` message down its view hierarchy, thus redrawing all views.
    /// * `animate` - `true` to perform the animation, whose duration is specified by `animation_resize_time`
    fn im_set_frame_display_animate(&self, frame: NSRect, flag: bool, animate: bool) {
        unsafe { msg_send![self.im_self(), setFrame: frame display: flag animate: animate] }
    }

    /// Specifies the duration of a smooth frame-size change.
    fn im_animation_resize_time(&self, frame: NSRect) -> NSTimeInterval {
        unsafe { msg_send![self.im_self(), animationResizeTime: frame] }
    }

    /// The window’s aspect ratio, which constrains the size of its frame rectangle to integral multiples of this ratio when the user resizes it.
    fn ip_aspect_ratio(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), aspectRatio] }
    }

    /// Sets the window’s aspect ratio.
    ///
    /// # Arguments
    ///
    /// * `ratio` - The aspect ratio to set.
    fn ip_set_aspect_ratio(&self, ratio: NSSize) {
        unsafe { msg_send![self.im_self(), setAspectRatio: ratio] }
    }

    /// The minimum size to which the window’s frame (including its title bar) can be sized.
    fn ip_min_size(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), minSize] }
    }

    /// Sets the minimum size to which the window’s frame (including its title bar) can be sized.
    ///
    /// # Arguments
    ///
    /// * `size` - The minimum size to which the window’s frame (including its title bar) can be sized.
    fn ip_set_min_size(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setMinSize: size] }
    }

    /// The maximum size to which the window’s frame (including its title bar) can be sized.
    fn ip_max_size(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), maxSize] }
    }

    /// Sets the maximum size to which the window’s frame (including its title bar) can be sized.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum size to which the window’s frame (including its title bar) can be sized.
    fn ip_set_max_size(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setMaxSize: size] }
    }

    /// A Boolean value that indicates whether the window is in a zoomed state.
    fn ip_zoomed(&self) -> bool {
        unsafe { msg_send![self.im_self(), isZoomed] }
    }

    /// This action method simulates the user clicking the zoom box by momentarily highlighting the button and then zooming the window.
    fn im_perform_zoom(&self, sender: id) {
        unsafe { msg_send![self.im_self(), performZoom: sender] }
    }

    /// Toggles the size and location of the window between its standard state (which the application provides as the best size to display the window’s data) and its user state (a new size and location the user may have set by moving or resizing the window).
    fn im_zoom(&self, sender: id) {
        unsafe { msg_send![self.im_self(), zoom: sender] }
    }

    /// The window’s resizing increments.
    fn ip_resize_increments(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), resizeIncrements] }
    }

    /// Sets the window’s resizing increments.
    ///
    /// # Arguments
    ///
    /// * `increments` - The resizing increments to set.
    fn ip_set_resize_increments(&self, increments: NSSize) {
        unsafe { msg_send![self.im_self(), setResizeIncrements: increments] }
    }

    /// A Boolean value that indicates whether the window tries to optimize user-initiated resize operations by preserving the content of views that have not changed.
    fn ip_preserves_content_during_live_resize(&self) -> bool {
        unsafe { msg_send![self.im_self(), preservesContentDuringLiveResize] }
    }

    /// Sets whether the window tries to optimize user-initiated resize operations by preserving the content of views that have not changed.
    ///
    /// # Arguments
    ///
    /// * `flag` - `true` to optimize user-initiated resize operations by preserving the content of views that have not changed.
    fn ip_set_preserves_content_during_live_resize(&self, flag: bool) {
        unsafe { msg_send![self.im_self(), setPreservesContentDuringLiveResize: flag] }
    }

    /// A Boolean value that indicates whether the window is being resized by the user.
    fn ip_in_live_resize(&self) -> bool {
        unsafe { msg_send![self.im_self(), inLiveResize] }
    }

    /// Sets a Boolean value that indicates whether the window is being resized by the user.
    ///
    /// # Arguments
    ///
    /// * `flag` - `true` if the window is being resized by the user.
    fn ip_set_in_live_resize(&self, flag: bool) {
        unsafe { msg_send![self.im_self(), setInLiveResize: flag] }
    }

    /* Sizing Content
     */

    /// The window’s content aspect ratio.
    fn ip_content_aspect_ratio(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), contentAspectRatio] }
    }

    /// Sets the window’s content aspect ratio.
    ///
    /// # Arguments
    ///
    /// * `content_aspect_ratio` - The window’s content aspect ratio.
    fn ip_set_content_aspect_ratio(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setContentAspectRatio: size] }
    }

    /// The minimum size of the window’s content view in the window’s base coordinate system.
    fn ip_content_min_size(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), contentMinSize] }
    }

    /// Sets the minimum size of the window’s content view in the window’s base coordinate system.
    ///
    /// # Arguments
    ///
    /// * `content_min_size` - The minimum size of the window’s content view in the window’s base coordinate system.
    fn ip_set_content_min_size(&self, content_min_size: NSSize) {
        unsafe { msg_send![self.im_self(), setContentMinSize: content_min_size] }
    }

    /// Sets the size of the window’s content view to a given size, which is expressed in the window’s base coordinate system.
    ///
    /// # Arguments
    ///
    /// * `content_size` - The size of the window’s content view in the window’s base coordinate system.
    fn im_set_content_size(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setContentSize: size] }
    }

    /// The maximum size of the window’s content view in the window’s base coordinate system.
    fn ip_content_max_size(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), contentMaxSize] }
    }

    /// Sets the maximum size of the window’s content view in the window’s base coordinate system.
    ///
    /// # Arguments
    ///
    /// * `content_max_size` - The maximum size of the window’s content view in the window’s base coordinate system.
    fn ip_set_content_max_size(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setContentMaxSize: size] }
    }

    /// The window’s content-view resizing increments.
    fn ip_content_resize_increments(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), contentResizeIncrements] }
    }

    /// Sets the window’s content-view resizing increments.
    ///
    /// # Arguments
    ///
    /// * `content_resize_increments` - The window’s content-view resizing increments.
    fn ip_set_content_resize_increments(&self, content_resize_increments: NSSize) {
        unsafe {
            msg_send![
                self.im_self(),
                setContentResizeIncrements: content_resize_increments
            ]
        }
    }

    /// A value used by Auto Layout constraints to automatically bind to the value of contentLayoutRect.
    fn ip_content_layout_guide(&self) -> id {
        unsafe { msg_send![self.im_self(), contentLayoutGuide] }
    }

    /// Sets a value used by Auto Layout constraints to automatically bind to the value of contentLayoutRect.
    ///
    /// # Arguments
    ///
    /// * `content_layout_guide` - A value used by Auto Layout constraints to automatically bind to the value of contentLayoutRect.
    fn ip_set_content_layout_guide(&self, content_layout_guide: id) {
        unsafe { msg_send![self.im_self(), setContentLayoutGuide: content_layout_guide] }
    }

    /// The area inside the window that is for non-obscured content, in window coordinates.
    fn ip_content_layout_rect(&self) -> NSRect {
        unsafe { msg_send![self.im_self(), contentLayoutRect] }
    }

    /// Sets the area inside the window that is for non-obscured content, in window coordinates.
    ///
    /// # Arguments
    ///
    /// * `content_layout_rect` - The area inside the window that is for non-obscured content, in window coordinates.
    fn ip_set_content_layout_rect(&self, content_layout_rect: NSRect) {
        unsafe { msg_send![self.im_self(), setContentLayoutRect: content_layout_rect] }
    }

    /// A maximum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_full_screen_tile_size(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), fullScreenTileSize] }
    }

    /// Sets a maximum size that is used to determine if a window can fit when it is in full screen in a tile.
    ///
    /// # Arguments
    ///
    /// * `full_screen_tile_size` - A maximum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_set_full_screen_tile_size(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setFullScreenTileSize: size] }
    }

    /// A minimum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_full_screen_tile_content_size(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), fullScreenTileContentSize] }
    }

    /// Sets a minimum size that is used to determine if a window can fit when it is in full screen in a tile.
    ///
    /// # Arguments
    ///
    /// * `full_screen_tile_content_size` - A minimum size that is used to determine if a window can fit when it is in full screen in a tile.
    fn ip_set_full_screen_tile_content_size(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setFullScreenTileContentSize: size] }
    }

    /* Managing Window Layers
     */

    /// Removes the window from the screen list, which hides the window.
    fn im_order_out(&self, sender: id) {
        unsafe { msg_send![self.im_self(), orderOut: sender] }
    }

    /// Moves the window to the back of its level in the screen list, without changing either the key window or the main window.
    fn im_order_back(&self, sender: id) {
        unsafe { msg_send![self.im_self(), orderBack: sender] }
    }

    /// Moves the window to the front of its level in the screen list, without changing either the key window or the main window.
    fn im_order_front(&self, sender: id) {
        unsafe { msg_send![self.im_self(), orderFront: sender] }
    }

    /// Moves the window to the front of its level, even if its application isn’t active, without changing either the key window or the main window.
    fn im_order_front_regardless(&self) {
        unsafe { msg_send![self.im_self(), orderFrontRegardless] }
    }

    /// Repositions the window’s window device in the window server’s screen list.
    fn im_order_window_relative_to(&self, place: NSWindowOrderingMode, other: Int) {
        unsafe { msg_send![self.im_self(), orderWindow: place relativeTo: other] }
    }

    /// The window level of the window.
    fn ip_window_level(&self) -> NSWindowLevel {
        unsafe { msg_send![self.im_self(), windowLevel] }
    }

    /// Sets the window level of the window.
    ///
    /// # Arguments
    ///
    /// * `level` - The window level of the window.
    fn ip_set_window_level(&self, level: NSWindowLevel) {
        unsafe { msg_send![self.im_self(), setWindowLevel: level] }
    }

    /* Managing Window Visibility and Occlusion State
     */

    /// A Boolean value that indicates whether the window is visible onscreen (even when it’s obscured by other windows).
    fn ip_visible(&self) -> bool {
        unsafe { msg_send![self.im_self(), isVisible] }
    }

    /// The occlusion state of the window.
    fn ip_occlusion_state(&self) -> NSWindowOcclusionState {
        unsafe { msg_send![self.im_self(), occlusionState] }
    }

    /* Managing Window Frames in User Defaults
     */

    /// Removes the frame data stored under a given name from the application’s user defaults.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the frame data to remove.
    fn tm_remove_frame_using_name(name: &NSWindowFrameAutosaveName) {
        unsafe { msg_send![Self::im_class(), removeFrameUsingName: name] }
    }

    /// Sets the window’s frame rectangle by reading the rectangle data stored under a given name from the defaults system.
    fn im_set_frame_using_name(&self, name: NSWindowFrameAutosaveName) -> bool {
        unsafe { msg_send![self.im_self(), setFrameUsingName: name] }
    }

    /// Sets the window’s frame rectangle by reading the rectangle data stored under a given name from the defaults system. Can operate on non-resizable windows.
    fn im_set_frame_using_name_force(&self, name: NSWindowFrameAutosaveName, force: bool) -> bool {
        unsafe { msg_send![self.im_self(), setFrameUsingName:name force:force] }
    }

    /// Saves the window’s frame rectangle in the user defaults system under a given name.
    fn im_save_frame_using_name(&self, name: NSWindowFrameAutosaveName) {
        unsafe { msg_send![self.im_self(), saveFrameUsingName: name] }
    }

    /// Sets the name AppKit uses to automatically save the window’s frame rectangle data in the defaults system.
    fn im_set_frame_autosave_name(&self, name: NSWindowFrameAutosaveName) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), setFrameAutosaveName: name]) }
    }

    /// The name used to automatically save the window’s frame rectangle data in the defaults system.
    fn ip_frame_autosave_name(&self) -> NSWindowFrameAutosaveName {
        unsafe { msg_send![self.im_self(), frameAutosaveName] }
    }

    /// A string representation of the window’s frame rectangle.
    fn ip_string_with_saved_frame(&self) -> NSWindowPersistableFrameDescriptor {
        unsafe { msg_send![self.im_self(), stringWithSavedFrame] }
    }

    /// Sets the window’s frame rectangle from a given string representation.
    fn im_set_frame_from_string(&self, string: NSWindowPersistableFrameDescriptor) {
        unsafe { msg_send![self.im_self(), setFrameFromString: string] }
    }

    /* Managing Key Status
     */

    /// A Boolean value that indicates whether the window is the key window for the application.
    fn ip_key_window(&self) -> bool {
        unsafe { msg_send![self.im_self(), isKeyWindow] }
    }

    /// A Boolean value that indicates whether the window can become the key window.
    fn ip_can_become_key_window(&self) -> bool {
        unsafe { msg_send![self.im_self(), canBecomeKeyWindow] }
    }

    /// Makes the window the key window.
    fn im_make_key_window(&self) {
        unsafe { msg_send![self.im_self(), makeKeyWindow] }
    }

    /// Moves the window to the front of the screen list, within its level, and makes it the key window; that is, it shows the window.
    fn im_make_key_and_order_front(&self, sender: id) {
        unsafe { msg_send![self.im_self(), makeKeyAndOrderFront: sender] }
    }

    /// Informs the window that it has become the key window.
    fn im_become_key_window(&self) {
        unsafe { msg_send![self.im_self(), becomeKeyWindow] }
    }

    /// Resigns the window’s key window status.
    fn im_resign_key_window(&self) {
        unsafe { msg_send![self.im_self(), resignKeyWindow] }
    }

    /* Managing Main Status
     */

    /// A Boolean value that indicates whether the window is the application’s main window.
    fn ip_main_window(&self) -> bool {
        unsafe { msg_send![self.im_self(), isMainWindow] }
    }

    /// A Boolean value that indicates whether the window can become the application’s main window.
    fn ip_can_become_main_window(&self) -> bool {
        unsafe { msg_send![self.im_self(), canBecomeMainWindow] }
    }

    /// Makes the window the main window.
    fn im_make_main_window(&self) {
        unsafe { msg_send![self.im_self(), makeMainWindow] }
    }

    /// Informs the window that it has become the main window.
    fn im_become_main_window(&self) {
        unsafe { msg_send![self.im_self(), becomeMainWindow] }
    }

    /// Resigns the window’s main window status.
    fn im_resign_main_window(&self) {
        unsafe { msg_send![self.im_self(), resignMainWindow] }
    }

    /// The window’s toolbar.
    fn ip_toolbar(&self) -> NSToolbar {
        unsafe { msg_send![self.im_self(), toolbar] }
    }

    /// Sets the window’s toolbar.
    ///
    /// # Arguments
    ///
    /// * `toolbar` - The window’s toolbar.
    fn ip_set_toolbar(&self, toolbar: NSToolbar) {
        unsafe { msg_send![self.im_self(), setToolbar: toolbar] }
    }

    /// Toggles the visibility of the window’s toolbar.
    fn ip_toggle_toolbar_shown(&self, sender: id) {
        unsafe { msg_send![self.im_self(), toggleToolbarShown: sender] }
    }

    /// Presents the toolbar customization user interface.
    fn ip_run_toolbar_customization_palette(&self, sender: id) {
        unsafe { msg_send![self.im_self(), runToolbarCustomizationPalette: sender] }
    }

    /* Managing Attached Windows
     */

    /// An array of the window’s attached child windows.
    fn ip_child_windows(&self) -> id {
        unsafe { msg_send![self.im_self(), childWindows] }
    }

    /// Attaches a child window to the window.
    fn add_child_window_ordered(&self, child: id, order: NSWindowOrderingMode) {
        unsafe { msg_send![self.im_self(), addChildWindow: child ordered: order] }
    }

    /// Detaches a given child window from the window.
    fn remove_child_window(&self, child: id) {
        unsafe { msg_send![self.im_self(), removeChildWindow: child] }
    }

    /// The parent window to which the window is attached as a child.
    fn ip_parent_window(&self) -> id {
        unsafe { msg_send![self.im_self(), parentWindow] }
    }

    /* Managing the Window Menu
     */

    /// A Boolean value that indicates whether the window is excluded from the application’s Windows menu.
    fn ip_excluded_from_windows_menu(&self) -> bool {
        unsafe { msg_send![self.im_self(), isExcludedFromWindowsMenu] }
    }

    /// Sets a Boolean value that indicates whether the window is excluded from the application’s Windows menu.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the window is excluded from the application’s Windows menu.
    fn ip_set_excluded_from_windows_menu(&self, flag: bool) {
        unsafe { msg_send![self.im_self(), setExcludedFromWindowsMenu: flag] }
    }

    /* Managing Cursor Rectangles
     */

    /// A Boolean value that indicates whether the window’s cursor rectangles are enabled.
    fn ip_are_cursor_rects_enabled(&self) -> bool {
        unsafe { msg_send![self.im_self(), areCursorRectsEnabled] }
    }

    /// Reenables cursor rectangle management within the window after a disableCursorRects message.
    fn im_enable_cursor_rects(&self) {
        unsafe { msg_send![self.im_self(), enableCursorRects] }
    }

    /// Disables all cursor rectangle management within the window.
    fn im_disable_cursor_rects(&self) {
        unsafe { msg_send![self.im_self(), disableCursorRects] }
    }

    /// Invalidates all cursor rectangles in the window.
    fn im_discard_cursor_rects(&self, view: id) {
        unsafe { msg_send![self.im_self(), discardCursorRectsForView: view] }
    }

    /// Clears the window’s cursor rectangles and the cursor rectangles of the NSView objects in its view hierarchy.
    fn im_reset_cursor_rects(&self) {
        unsafe { msg_send![self.im_self(), resetCursorRects] }
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
                msg_send![Self::im_class(), standardWindowButton: b forStyleMask: style],
            )
        }
    }

    /// Returns the window button of a given window button kind in the window’s view hierarchy.
    fn ip_standard_window_button(&self, b: NSWindowButton) -> NSButton {
        unsafe { msg_send![self.im_self(), standardWindowButton: b] }
    }

    /// A Boolean value that indicates whether the toolbar control button is currently displayed.
    fn ip_shows_toolbar_button(&self) -> bool {
        unsafe { msg_send![self.im_self(), showsToolbarButton] }
    }

    /// Sets a Boolean value that indicates whether the toolbar control button is displayed.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the toolbar control button is displayed.
    fn ip_set_shows_toolbar_button(&self, flag: bool) {
        unsafe { msg_send![self.im_self(), setShowsToolbarButton: flag] }
    }

    /// A Boolean value that indicates whether the title bar draws its background.
    fn ip_titlebar_appears_transparent(&self) -> bool {
        unsafe { msg_send![self.im_self(), titlebarAppearsTransparent] }
    }

    /// Sets a Boolean value that indicates whether the title bar draws its background.
    ///
    /// # Arguments
    ///
    /// * `flag` - A Boolean value that indicates whether the title bar draws its background.
    fn ip_set_titlebar_appears_transparent(&self, flag: bool) {
        unsafe { msg_send![self.im_self(), setTitlebarAppearsTransparent: flag] }
    }

    /// The style that determines the appearance and location of the toolbar in relation to the title bar.
    fn ip_toolbar_style(&self) -> NSWindowToolbarStyle {
        unsafe { msg_send![self.im_self(), toolbarStyle] }
    }

    /// Sets the style that determines the appearance and location of the toolbar in relation to the title bar.
    ///
    /// # Arguments
    ///
    /// * `style` - The style that determines the appearance and location of the toolbar in relation to the title bar.
    fn ip_set_toolbar_style(&self, style: NSWindowToolbarStyle) {
        unsafe { msg_send![self.im_self(), setToolbarStyle: style] }
    }

    /// The type of separator that the app displays between the title bar and content of a window.
    fn ip_titlebar_separator_style(&self) -> NSTitlebarSeparatorStyle {
        unsafe { msg_send![self.im_self(), titlebarSeparatorStyle] }
    }

    /// Sets the type of separator that the app displays between the title bar and content of a window.
    ///
    /// # Arguments
    ///
    /// * `style` - The type of separator that the app displays between the title bar and content of a window.
    fn ip_set_titlebar_separator_style(&self, style: NSTitlebarSeparatorStyle) {
        unsafe { msg_send![self.im_self(), setTitlebarSeparatorStyle: style] }
    }

    /// The direction the window’s title bar lays text out, either left to right or right to left.
    fn ip_titlebar_layout_direction(&self) -> NSUserInterfaceLayoutDirection {
        unsafe { msg_send![self.im_self(), titlebarLayoutDirection] }
    }

    /* Managing Window Tabs
     */

    /// A Boolean value that indicates whether the app can automatically organize windows into tabs.
    fn tp_allows_automatic_window_tabbing() -> bool {
        unsafe { to_bool(msg_send![Self::im_class(), allowsAutomaticWindowTabbing]) }
    }

    /// Sets a Boolean value that indicates whether the app can automatically organize windows into tabs.
    fn tm_set_allows_automatic_window_tabbing(allows_automatic_window_tabbing: bool) {
        unsafe {
            msg_send![
                Self::im_class(),
                setAllowsAutomaticWindowTabbing: allows_automatic_window_tabbing
            ]
        }
    }

    /* Converting Coordinates
     */

    /// The backing scale factor.
    fn ip_backing_scale_factor(&self) -> CGFloat {
        unsafe { msg_send![self.im_self(), backingScaleFactor] }
    }

    /// Returns a backing store pixel-aligned rectangle in window coordinates.
    fn im_backing_aligned_rect_options(&self, options: NSAlignmentOptions) -> NSRect {
        unsafe { msg_send![self.im_self(), backingAlignedRect: options] }
    }

    /// Converts a rectangle from its pixel-aligned backing store coordinate system to the window’s coordinate system.
    fn im_convert_rect_from_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.im_self(), convertRectFromBacking: rect] }
    }

    /// Converts a rectangle from the screen coordinate system to the window’s coordinate system.
    fn im_convert_rect_from_screen(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.im_self(), convertRectFromScreen: rect] }
    }

    /// Converts a point from its pixel-aligned backing store coordinate system to the window’s coordinate system.
    fn im_convert_point_from_backing(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.im_self(), convertPointFromBacking: point] }
    }

    /// Converts a point from the screen coordinate system to the window’s coordinate system.
    fn im_convert_point_from_screen(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.im_self(), convertPointFromScreen: point] }
    }

    /// Converts a rectangle from the window’s coordinate system to its pixel-aligned backing store coordinate system.
    fn im_convert_rect_to_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.im_self(), convertRectToBacking: rect] }
    }

    /// Converts a rectangle to the screen coordinate system from the window’s coordinate system.
    fn im_convert_rect_to_screen(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.im_self(), convertRectToScreen: rect] }
    }

    /// Converts a point from the window’s coordinate system to its pixel-aligned backing store coordinate system.
    fn im_convert_point_to_backing(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.im_self(), convertPointToBacking: point] }
    }

    /// Converts a point to the screen coordinate system from the window’s coordinate system.
    fn im_convert_point_to_screen(&self, point: NSPoint) -> NSPoint {
        unsafe { msg_send![self.im_self(), convertPointToScreen: point] }
    }

    /* Managing Titles
     */

    /// The string that appears in the title bar of the window or the path to the represented file.
    fn ip_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), title]) }
    }

    /// Sets the string that appears in the title bar of the window or the path to the represented file.
    ///
    /// # Arguments
    ///
    /// * `title` - The string that appears in the title bar of the window or the path to the represented file.
    fn ip_set_title(&self, title: NSString) {
        unsafe { msg_send![self.im_self(), setTitle: title] }
    }

    /// A secondary line of text that appears in the title bar of the window.
    fn ip_subtitle(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), subtitle]) }
    }

    /// Sets a secondary line of text that appears in the title bar of the window.
    ///
    /// # Arguments
    ///
    /// * `subtitle` - A secondary line of text that appears in the title bar of the window.
    fn ip_set_subtitle(&self, subtitle: NSString) {
        unsafe { msg_send![self.im_self(), setSubtitle: subtitle] }
    }

    /// A value that indicates the visibility of the window’s title and title bar buttons.
    fn ip_title_visibility(&self) -> NSWindowTitleVisibility {
        unsafe { msg_send![self.im_self(), titleVisibility] }
    }

    /// Sets a value that indicates the visibility of the window’s title and title bar buttons.
    ///
    /// # Arguments
    ///
    /// * `title_visibility` - A value that indicates the visibility of the window’s title and title bar buttons.
    fn ip_set_title_visibility(&self, title_visibility: NSWindowTitleVisibility) {
        unsafe { msg_send![self.im_self(), setTitleVisibility: title_visibility] }
    }

    /// Sets a given path as the window’s title, formatting it as a file-system path, and records this path as the window’s associated file.
    fn im_set_title_with_represented_filename(&self, path: NSString) {
        unsafe { msg_send![self.im_self(), setTitleWithRepresentedFilename: path] }
    }

    /// The path to the file of the window’s represented file.
    fn ip_represented_filename(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), representedFilename]) }
    }

    /// Sets the path to the file of the window’s represented file.
    ///
    /// # Arguments
    ///
    /// * `represented_filename` - The path to the file of the window’s represented file.
    fn ip_set_represented_filename(&self, represented_filename: NSString) {
        unsafe { msg_send![self.im_self(), setRepresentedFilename: represented_filename] }
    }

    /* Accessing Screen Information
     */

    /// The screen the window is on.
    fn ip_screen(&self) -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![self.im_self(), screen]) }
    }

    /// The deepest screen the window is on (it may be split over several screens).
    fn ip_deepest_screen(&self) -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![self.im_self(), deepestScreen]) }
    }

    /// A Boolean value that indicates whether the window context should be updated when the screen profile changes or when the window moves to a different screen.
    fn ip_displays_when_screen_profile_changes(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), displaysWhenScreenProfileChanges]) }
    }

    /// A Boolean value that indicates whether the window is movable by clicking and dragging anywhere in its background.
    fn ip_movable_by_window_background(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), movableByWindowBackground]) }
    }

    /// Sets a Boolean value that indicates whether the window is movable by clicking and dragging anywhere in its background.
    ///
    /// # Arguments
    ///
    /// * `movable_by_window_background` - A Boolean value that indicates whether the window is movable by clicking and dragging anywhere in its background.
    fn ip_set_movable_by_window_background(&self, movable_by_window_background: bool) {
        unsafe {
            msg_send![
                self.im_self(),
                setMovableByWindowBackground: movable_by_window_background
            ]
        }
    }

    /// A Boolean value that indicates whether the window can be dragged by clicking in its title bar or background.
    fn ip_movable(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isMovable]) }
    }

    /// Sets a Boolean value that indicates whether the window can be dragged by clicking in its title bar or background.
    ///
    /// # Arguments
    ///
    /// * `movable` - A Boolean value that indicates whether the window can be dragged by clicking in its title bar or background.
    fn ip_set_movable(&self, movable: bool) {
        unsafe { msg_send![self.im_self(), setMovable: movable] }
    }

    /// Sets the window’s location to the center of the screen.
    fn im_center(&self) {
        unsafe { msg_send![self.im_self(), center] }
    }

    /* Closing Windows
     */

    /// Simulates the user clicking the close button by momentarily highlighting the button and then closing the window.
    fn im_perform_close(&self, sender: id) {
        unsafe { msg_send![self.im_self(), performClose: sender] }
    }

    /// Removes the window from the screen.
    fn im_close(&self) {
        unsafe { msg_send![self.im_self(), close] }
    }

    /// A Boolean value that indicates whether the window is released when it receives the close message.
    fn ip_released_when_closed(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isReleasedWhenClosed]) }
    }

    /// Sets a Boolean value that indicates whether the window is released when it receives the close message.
    fn ip_set_released_when_closed(&self, released_when_closed: bool) {
        unsafe { msg_send![self.im_self(), setReleasedWhenClosed: released_when_closed] }
    }

    /* Minimizing Windows
     */

    /// A Boolean value that indicates whether the window is minimized.
    fn ip_miniaturized(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isMiniaturized]) }
    }

    /// Simulates the user clicking the minimize button by momentarily highlighting the button, then minimizing the window.
    fn im_perform_miniaturize(&self, sender: id) {
        unsafe { msg_send![self.im_self(), performMiniaturize: sender] }
    }

    /// Removes the window from the screen list and displays the minimized window in the Dock.
    fn im_miniaturize(&self, sender: id) {
        unsafe { msg_send![self.im_self(), miniaturize: sender] }
    }

    /// De-minimizes the window.
    fn im_deminiaturize(&self, sender: id) {
        unsafe { msg_send![self.im_self(), deminiaturize: sender] }
    }

    /// The custom miniaturized window image of the window.
    fn ip_miniwindow_image(&self) -> NSImage {
        unsafe { NSImage::from_id(msg_send![self.im_self(), miniwindowImage]) }
    }

    /// The title displayed in the window’s minimized window.
    fn ip_miniwindow_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), miniwindowTitle]) }
    }

    /* Getting the Dock Tile
     */

    /// The application’s Dock tile.
    fn ip_dock_tile(&self) -> NSDockTile {
        unsafe { NSDockTile::from_id(msg_send![self.im_self(), dockTile]) }
    }

    /* Printing Windows
     */

    /// Runs the Print panel, and if the user chooses an option other than canceling, prints the window (its frame view and all subviews).
    fn im_print(&self, sender: id) {
        unsafe { msg_send![self.im_self(), print: sender] }
    }

    /// Returns EPS data that draws the region of the window within a given rectangle.
    fn im_data_with_esp_inside_rect(&self, rect: NSRect) -> NSData {
        unsafe { NSData::from_id(msg_send![self.im_self(), dataWithEPSInsideRect: rect]) }
    }

    /// Returns PDF data that draws the region of the window within a given rectangle.
    fn im_data_with_pdf_inside_rect(&self, rect: NSRect) -> NSData {
        unsafe { NSData::from_id(msg_send![self.im_self(), dataWithPDFInsideRect: rect]) }
    }

    /* Handling Window Restoration
     */

    /// A Boolean value indicating whether the window configuration is preserved between application launches.
    fn ip_restorable(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isRestorable]) }
    }

    /// Sets a Boolean value indicating whether the window configuration is preserved between application launches.
    ///
    /// # Arguments
    ///
    /// * `restorable` - A Boolean value indicating whether the window configuration is preserved between application launches.
    fn ip_set_restorable(&self, restorable: bool) {
        unsafe { msg_send![self.im_self(), setRestorable: restorable] }
    }

    /// Disables snapshot restoration.
    fn im_disable_snapshot_restoration(&self) {
        unsafe { msg_send![self.im_self(), disableSnapshotRestoration] }
    }

    /// Enables snapshot restoration.
    fn im_enable_snapshot_restoration(&self) {
        unsafe { msg_send![self.im_self(), enableSnapshotRestoration] }
    }
}

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

/// A means to display additional content related to existing content on the screen.
pub trait INSPopover: INSResponder {
    /* Accessing a Popover’s Content View Controller
     */

    /// The view controller that manages the content of the popover.
    fn ip_content_view_controller(&self) -> NSViewController {
        unsafe { NSViewController::from_id(msg_send![self.im_self(), contentViewController]) }
    }

    /// Sets the view controller that manages the content of the popover.
    ///
    /// # Arguments
    ///
    /// * `view_controller` - The view controller that manages the content of the popover.
    fn ip_set_content_view_controller<Controller>(&self, view_controller: Controller)
    where
        Controller: INSViewController,
    {
        unsafe { msg_send![self.im_self(), setContentViewController: view_controller] }
    }

    /*  Managing a Popover's Position and Size */

    /// Specifies the behavior of the popover.
    fn ip_behavior(&self) -> NSPopoverBehavior {
        unsafe { msg_send![self.im_self(), behavior] }
    }

    /// Sets the behavior of the popover.
    ///
    /// # Arguments
    ///
    /// * `behavior` - The behavior of the popover.
    fn ip_set_behavior(&self, behavior: NSPopoverBehavior) {
        unsafe { msg_send![self.im_self(), setBehavior: behavior] }
    }

    /// Shows the popover anchored to the specified view.
    fn im_show_relative_to_rect_of_view_preferred_edge<V>(
        &self,
        rect: NSRect,
        view: V,
        edge: CGRectEdge,
    ) where
        V: INSView,
    {
        unsafe {
            msg_send![self.im_self(), showRelativeToRect: rect ofView: view preferredEdge: edge]
        }
    }

    /// The rectangle within the positioning view relative to which the popover should be positioned.
    fn ip_positioning_rect(&self) -> NSRect {
        unsafe { msg_send![self.im_self(), positioningRect] }
    }

    /* Managing a Popover's Appearance
     */

    /// The appearance of the popover.
    fn ip_appearance(&self) -> NSAppearance {
        unsafe { NSAppearance::from_id(msg_send![self.im_self(), appearance]) }
    }

    /// Sets the appearance of the popover.
    ///
    /// # Arguments
    ///
    /// * `appearance` - The appearance to use.
    fn im_set_appearance(&self, appearance: NSAppearance) {
        unsafe { msg_send![self.im_self(), setAppearance: appearance] }
    }

    /// The appearance that will be used when the popover is displayed onscreen.
    fn ip_effective_appearance(&self) -> NSAppearance {
        unsafe { NSAppearance::from_id(msg_send![self.im_self(), effectiveAppearance]) }
    }

    /// Specifies if the popover is to be animated.
    fn ip_animates(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), animates]) }
    }

    /// The content size of the popover.
    fn ip_content_size(&self) -> NSSize {
        unsafe { msg_send![self.im_self(), contentSize] }
    }

    /// Sets the content size of the popover.
    ///
    /// # Arguments
    ///
    /// * `size` - The size to use.
    fn ip_set_content_size(&self, size: NSSize) {
        unsafe { msg_send![self.im_self(), setContentSize: size] }
    }

    /// The display state of the popover.
    fn ip_shown(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isShown]) }
    }

    /// A Boolean value that indicates whether the window created by a popover's detachment is automatically created.
    fn ip_detached(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isDetached]) }
    }

    /* Closing a Popover
     */

    /// Attempts to close the popover.
    fn im_perform_close(&self, sender: id) {
        unsafe { msg_send![self.im_self(), performClose: sender] }
    }

    /// Forces the popover to close without consulting its delegate.
    fn im_close(&self) {
        unsafe { msg_send![self.im_self(), close] }
    }

    /// The delegate of the popover.
    fn ip_delegate(&self) -> id {
        unsafe { msg_send![self.im_self(), delegate] }
    }

    /// Sets the delegate of the popover.
    ///
    /// # Arguments
    ///
    /// * `delegate` - The delegate to use.
    fn im_set_delegate(&self, delegate: id) {
        unsafe { msg_send![self.im_self(), setDelegate: delegate] }
    }

    /* Initializers
     */

    /// Creates a new popover.
    fn im_init(&self) -> NSPopover {
        unsafe { msg_send![self.im_self(), init] }
    }

    /// Creates a new popover with `NSCoder`
    fn im_init_with_coder(&self, coder: NSCoder) -> NSPopover {
        unsafe { msg_send![self.im_self(), initWithCoder: coder] }
    }
}

/// A set of optional methods that a popover delegate can implement to provide additional or custom functionality.
pub trait PNSPopoverDelegate {
    /* Popover Window
     */

    /// Detaches the popover creating a window containing the content.
    ///
    /// # Arguments
    ///
    /// * `popover` - The popover to detach.
    ///
    /// # Returns
    ///
    /// The detached window.
    fn ip_detachable_window_for_popover(&self, popover: NSPopover) -> NSWindow;

    /* Popover Visibility
     */

    /// Allows a delegate to override a close request.
    ///
    /// # Arguments
    ///
    /// * `popover` - The popover that is about to close.
    fn im_popover_should_close(&self, popover: NSPopover) -> bool;

    /// Invoked when the popover will show.
    fn im_popover_will_show(&self) {}

    /// Invoked when the popover has been shown.
    fn im_popover_did_show(&self) {}

    ///Invoked when the popover is about to close.
    fn im_popover_will_close(&self) {}

    /// Invoked when the popover did close.
    fn im_popover_did_close(&self) {}

    /// Indicates that a popover has been released while it's in an implicitly detached state.
    fn im_popover_did_detach(&self) {}

    /// Returns a Boolean value that indicates whether a popover should detach from its positioning view and become a separate window.
    fn im_popover_should_detach(&self, popover: NSPopover) -> bool;
}
