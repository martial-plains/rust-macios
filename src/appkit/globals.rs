use libc::{c_char, c_int};

use crate::foundation::Int;

use super::{NSColorSpaceName, NSDeviceDescriptionKey, NSWindowDepth, NSWindowLevel};

extern "C" {
    /// Called by the main function to create and run the application.
    ///
    /// # Arguments
    ///
    /// * `argc` - The number of arguments passed to the program.
    /// * `argv` - The arguments passed to the program.
    ///
    /// # Returns
    ///
    /// This method never returns a result code. Instead, it calls the exit
    /// function to exit the application and terminate the process. If you want
    /// to determine why the application exited, you should look at the result
    /// code from the exit function instead.
    pub fn NSApplicationMain(argc: c_int, argv: *const *const c_char) -> c_int;
}

extern "C" {
    /// Returns the bits per pixel for the specified window depth.
    pub fn NSBitsPerPixelFromDepth(depth: NSWindowDepth) -> Int;

    /// Returns the bits per sample for the specified window depth.
    pub fn NSBitsPerSampleFromDepth(depth: NSWindowDepth) -> Int;

    /// Returns the name of the color space corresponding to the passed window depth.
    pub fn NSColorSpaceFromDepth(depth: NSWindowDepth) -> NSColorSpaceName;

    /// Returns the number of color components in the specified color space.
    pub fn NSNumberOfColorComponents(colorSpace: NSColorSpaceName) -> Int;

    /// Returns whether the specified window depth is planar.
    pub fn NSPlanarFromDepth(depth: NSWindowDepth) -> bool;
}

extern "C" {
    /// The corresponding value is an NSNumber object containing an integer that gives the bit depth of the window’s raster image (2-bit, 8-bit, and so forth).
    pub static NSDeviceBitsPerSample: NSDeviceDescriptionKey;

    /// The corresponding value is an NSString object giving the name of the window’s color space.
    pub static NSDeviceColorSpaceName: NSDeviceDescriptionKey;

    /// If there is a corresponding value, this indicates that the display device is a printer.
    pub static NSDeviceIsPrinter: NSDeviceDescriptionKey;

    /// If there is a corresponding value, this indicates that the display device is a screen.
    pub static NSDeviceIsScreen: NSDeviceDescriptionKey;

    /// The corresponding value is an NSValue object containing a value of type NSSize that describes the window’s raster resolution in dots per inch (dpi).
    pub static NSDeviceResolution: NSDeviceDescriptionKey;

    /// The corresponding value is an NSValue object containing a value of type NSSize that gives the size of the window’s frame rectangle.
    pub static NSDeviceSize: NSDeviceDescriptionKey;
}

extern "C" {
    /// The level for the dock.
    #[deprecated]
    pub static NSDockWindowLevel: NSWindowLevel;
    /// Useful for floating palettes.
    pub static NSFloatingWindowLevel: NSWindowLevel;
    /// Reserved for the application’s main menu.
    pub static NSMainMenuWindowLevel: NSWindowLevel;
    /// The level for a modal panel.
    pub static NSModalPanelWindowLevel: NSWindowLevel;
    /// The default level for NSWindow objects.
    pub static NSNormalWindowLevel: NSWindowLevel;
    /// The level for a pop-up menu.
    pub static NSPopUpMenuWindowLevel: NSWindowLevel;
    /// The level for a screen saver.
    pub static NSScreenSaverWindowLevel: NSWindowLevel;
    /// The level for a status window.
    pub static NSStatusWindowLevel: NSWindowLevel;
    /// Reserved for submenus. Synonymous with NSTornOffMenuWindowLevel, which is preferred.
    pub static NSSubmenuWindowLevel: NSWindowLevel;
    /// The level for a torn-off menu. Synonymous with NSSubmenuWindowLevel.
    pub static NSTornOffMenuWindowLevel: NSWindowLevel;
}
