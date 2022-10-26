use objc::{msg_send, sel, sel_impl};

use crate::{
    core_graphics::CGFloat,
    foundation::{
        Int, NSAlignmentOptions, NSDictionary, NSEdgeInsets, NSNotificationName, NSRect, NSString,
        NSTimeInterval,
    },
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

/// These constants are the keys for device description dictionaries.
pub type NSDeviceDescriptionKey = NSString;

use super::{interface_impl, object, NSColorSpace, NSWindowDepth};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u64)]
pub enum NSDisplayGamut {
    Srgb = 1,
    P3,
}

extern "C" {
    pub static NSScreenColorSpaceDidChangeNotification: NSNotificationName;
}

object! {
    /// An object that describes the attributes of a computer’s monitor or screen.
    unsafe pub struct NSScreen;
}

#[interface_impl(NSObject)]
impl NSScreen {
    /* Getting Screen Objects
     */

    /// Returns the screen object containing the window with the keyboard focus.
    #[property]
    pub fn main_screen() -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![Self::m_class(), mainScreen]) }
    }

    /// Returns a screen object representing the screen that can best represent color.
    #[property]
    pub fn deepest_screen() -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![Self::m_class(), deepestScreen]) }
    }

    /// Returns an array of screen objects representing all of the screens available on the system.
    #[property]
    pub fn screens() -> NSScreen {
        unsafe { NSScreen::from_id(msg_send![Self::m_class(), screens]) }
    }

    /*  Getting Screen Information
     */

    /// The current bit depth and colorspace information of the screen.
    #[property]
    pub fn depth(&self) -> NSWindowDepth {
        unsafe { msg_send![self.m_self(), depth] }
    }

    /// The dimensions and location of the screen.
    #[property]
    pub fn frame(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), frame] }
    }

    /// A zero-terminated array of the window depths supported by the screen.
    #[property]
    pub fn supported_window_depths(&self) -> *const NSWindowDepth {
        unsafe { msg_send![self.m_self(), supportedWindowDepths] }
    }

    /// The device dictionary for the screen.
    #[property]
    pub fn device_description(&self) -> NSDictionary<NSDeviceDescriptionKey, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), deviceDescription]) }
    }

    /// Returns the scaling factor from user space to device space on the screen.
    #[deprecated]
    #[method]
    pub fn user_space_scale_factor(&self) -> CGFloat {
        unsafe { msg_send![self.m_self(), userSpaceScaleFactor] }
    }

    /// The color space of the screen.
    #[property]
    pub fn color_space(&self) -> NSColorSpace {
        unsafe { NSColorSpace::from_id(msg_send![self.m_self(), colorSpace]) }
    }

    /// The localized name of the display.
    #[property]
    pub fn localized_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedName]) }
    }

    /// A Boolean value indicating whether the color space of the screen is capable of representing the specified display gamut.
    #[method]
    pub fn can_represent_display_gamut(&self, display_gamut: NSDisplayGamut) -> bool {
        unsafe {
            to_bool(msg_send![
                self.m_self(),
                canRepresentDisplayGamut: display_gamut
            ])
        }
    }

    /// Returns a Boolean value indicating whether each screen can have its own set of spaces.
    #[property]
    pub fn screens_have_separate_spaces() -> bool {
        unsafe { to_bool(msg_send![Self::m_class(), screensHaveSeparateSpaces]) }
    }

    /* Converting Between Screen and Backing Coordinates
     */

    /// Converts a rectangle in global screen coordinates to a pixel aligned rectangle.
    #[method]
    pub fn backing_aligned_rect_options(
        &self,
        rect: NSRect,
        options: NSAlignmentOptions,
    ) -> NSRect {
        unsafe { msg_send![self.m_self(), backingAlignedRect: rect options: options] }
    }

    /// The backing store pixel scale factor for the screen.
    #[property]
    pub fn backing_scale_factor(&self) -> CGFloat {
        unsafe { msg_send![self.m_self(), backingScaleFactor] }
    }

    /// Converts the rectangle from the device pixel aligned coordinates system of a screen.
    #[method]
    pub fn convert_rect_from_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.m_self(), convertRectFromBacking: rect] }
    }

    /// Converts the rectangle to the device pixel aligned coordinates system of a screen.
    #[method]
    pub fn convert_rect_to_backing(&self, rect: NSRect) -> NSRect {
        unsafe { msg_send![self.m_self(), convertRectToBacking: rect] }
    }

    /* Getting the Visible Portion of the Screen
     */

    /// The current location and dimensions of the visible screen.
    #[property]
    pub fn visible_frame(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), visibleFrame] }
    }

    /// The distances from the screen’s edges at which content isn’t obscured.
    #[property]
    pub fn safe_area_insets(&self) -> NSEdgeInsets {
        unsafe { msg_send![self.m_self(), safeAreaInsets] }
    }

    /* Getting Extended Dynamic Range Details
     */

    /// The maximum possible color component value for the screen when it's in extended dynamic range (EDR) mode.
    #[property]
    pub fn maximum_potential_extended_dynamic_range_color_component_value(&self) -> CGFloat {
        unsafe {
            msg_send![
                self.m_self(),
                maximumPotentialExtendedDynamicRangeColorComponentValue
            ]
        }
    }

    /// The current maximum color component value for the screen.
    #[property]
    pub fn maximum_extended_dynamic_range_color_component_value(&self) -> CGFloat {
        unsafe {
            msg_send![
                self.m_self(),
                maximumExtendedDynamicRangeColorComponentValue
            ]
        }
    }

    /// The current maximum color component value for reference rendering to the screen.
    #[property]
    pub fn maximum_reference_extended_dynamic_range_color_component_value(&self) -> CGFloat {
        unsafe {
            msg_send![
                self.m_self(),
                maximumReferenceExtendedDynamicRangeColorComponentValue
            ]
        }
    }

    /* Getting Variable Refresh Rate Details
     */

    /// The maximum number of frames per second that the screen supports.
    #[property]
    pub fn maximum_frames_per_second(&self) -> Int {
        unsafe { msg_send![self.m_self(), maximumFramesPerSecond] }
    }

    /// The shortest refresh interval that the screen supports.
    #[property]
    pub fn minimum_refresh_interval(&self) -> NSTimeInterval {
        unsafe { msg_send![self.m_self(), minimumRefreshInterval] }
    }

    /// The largest refresh interval that the screen supports.
    #[property]
    pub fn maximum_refresh_interval(&self) -> NSTimeInterval {
        unsafe { msg_send![self.m_self(), maximumRefreshInterval] }
    }

    /// The number of seconds between the screen’s supported update rates, for screens that support fixed update rates.
    #[property]
    pub fn display_update_granularity(&self) -> NSTimeInterval {
        unsafe { msg_send![self.m_self(), displayUpdateGranularity] }
    }

    /// The time of the last framebuffer update, expressed as the number of seconds since system startup.
    #[property]
    pub fn last_display_update_timestamp(&self) -> NSTimeInterval {
        unsafe { msg_send![self.m_self(), lastDisplayUpdateTimestamp] }
    }

    /* Instance Properties
     */

    #[property]
    pub fn auxiliary_top_left_area(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), auxiliaryTopLeftArea] }
    }

    #[property]
    pub fn auxiliary_top_right_area(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), auxiliaryTopRightArea] }
    }
}
