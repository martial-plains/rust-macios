use crate::{
    appkit::ns_window::NSWindow,
    foundation::{NSRect, NSSize},
};

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
