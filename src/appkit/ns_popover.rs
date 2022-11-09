use objc::{class, msg_send, sel, sel_impl};

use crate::{
    core_graphics::CGRectEdge,
    foundation::{NSCoder, NSRect, NSSize},
    object,
    objective_c_runtime::{id, traits::FromId},
    utils::to_bool,
};

use super::{
    interface_impl, ns_appearance::NSAppearance, INSResponder, INSView, INSViewController,
    NSPopoverBehavior, NSViewController,
};

object! {
    /// A means to display additional content related to existing content on the screen.
    unsafe pub struct NSPopover;
}

impl NSPopover {
    /// Creates a new popover.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSPopover), new]) }
    }
}

impl Default for NSPopover {
    fn default() -> Self {
        Self::new()
    }
}

impl INSResponder for NSPopover {}

#[interface_impl(NSResponder)]
impl NSPopover {
    /* Accessing a Popover’s Content View Controller
     */

    /// The view controller that manages the content of the popover.
    #[property]
    pub fn content_view_controller(&self) -> NSViewController {
        unsafe { NSViewController::from_id(msg_send![self.m_self(), contentViewController]) }
    }

    /// Sets the view controller that manages the content of the popover.
    ///
    /// # Arguments
    ///
    /// * `view_controller` - The view controller that manages the content of the popover.
    #[property]
    pub fn set_content_view_controller<Controller>(&self, view_controller: Controller)
    where
        Controller: INSViewController,
    {
        unsafe { msg_send![self.m_self(), setContentViewController: view_controller] }
    }

    /*  Managing a Popover's Position and Size */

    /// Specifies the behavior of the popover.
    #[property]
    pub fn behavior(&self) -> NSPopoverBehavior {
        unsafe { msg_send![self.m_self(), behavior] }
    }

    /// Sets the behavior of the popover.
    ///
    /// # Arguments
    ///
    /// * `behavior` - The behavior of the popover.
    #[property]
    pub fn set_behavior(&self, behavior: NSPopoverBehavior) {
        unsafe { msg_send![self.m_self(), setBehavior: behavior] }
    }

    /// Shows the popover anchored to the specified view.
    #[method]
    pub fn show_relative_to_rect_of_view_preferred_edge<V>(
        &self,
        rect: NSRect,
        view: V,
        edge: CGRectEdge,
    ) where
        V: INSView,
    {
        unsafe {
            msg_send![self.m_self(), showRelativeToRect: rect ofView: view preferredEdge: edge]
        }
    }

    /// The rectangle within the positioning view relative to which the popover should be positioned.
    #[property]
    pub fn positioning_rect(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), positioningRect] }
    }

    /* Managing a Popover's Appearance
     */

    /// The appearance of the popover.
    #[property]
    pub fn appearance(&self) -> NSAppearance {
        unsafe { NSAppearance::from_id(msg_send![self.m_self(), appearance]) }
    }

    /// Sets the appearance of the popover.
    ///
    /// # Arguments
    ///
    /// * `appearance` - The appearance to use.
    #[method]
    pub fn set_appearance(&self, appearance: NSAppearance) {
        unsafe { msg_send![self.m_self(), setAppearance: appearance] }
    }

    /// The appearance that will be used when the popover is displayed onscreen.
    #[property]
    pub fn effective_appearance(&self) -> NSAppearance {
        unsafe { NSAppearance::from_id(msg_send![self.m_self(), effectiveAppearance]) }
    }

    /// Specifies if the popover is to be animated.
    #[property]
    pub fn animates(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), animates]) }
    }

    /// The content size of the popover.
    #[property]
    pub fn content_size(&self) -> NSSize {
        unsafe { msg_send![self.m_self(), contentSize] }
    }

    /// Sets the content size of the popover.
    ///
    /// # Arguments
    ///
    /// * `size` - The size to use.
    #[property]
    pub fn set_content_size(&self, size: NSSize) {
        unsafe { msg_send![self.m_self(), setContentSize: size] }
    }

    /// The display state of the popover.
    #[property]
    pub fn shown(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isShown]) }
    }

    /// A Boolean value that indicates whether the window created by a popover's detachment is automatically created.
    #[property]
    pub fn detached(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isDetached]) }
    }

    /* Closing a Popover
     */

    /// Attempts to close the popover.
    #[method]
    pub fn perform_close(&self, sender: id) {
        unsafe { msg_send![self.m_self(), performClose: sender] }
    }

    /// Forces the popover to close without consulting its delegate.
    #[method]
    pub fn close(&self) {
        unsafe { msg_send![self.m_self(), close] }
    }

    /// The delegate of the popover.
    #[property]
    pub fn delegate(&self) -> id {
        unsafe { msg_send![self.m_self(), delegate] }
    }

    /// Sets the delegate of the popover.
    ///
    /// # Arguments
    ///
    /// * `delegate` - The delegate to use.
    #[method]
    pub fn set_delegate(&self, delegate: id) {
        unsafe { msg_send![self.m_self(), setDelegate: delegate] }
    }

    /* Initializers
     */

    /// Creates a new popover.
    #[method]
    pub fn init(&self) -> NSPopover {
        unsafe { msg_send![self.m_self(), init] }
    }

    /// Creates a new popover with `NSCoder`
    #[method]
    pub fn init_with_coder(&self, coder: NSCoder) -> NSPopover {
        unsafe { msg_send![self.m_self(), initWithCoder: coder] }
    }
}
