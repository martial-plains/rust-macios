use super::{NSPopover, NSWindow};

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
