use crate::foundation::{Int, NSString};

/// Constants that specify color space names.
pub type NSColorSpaceName = NSString;

/// A set of button return values for modal dialogs.
pub type NSModalResponse = Int;

/// The standard window levels in macOS.
pub type NSWindowLevel = Int;

/// The type of a window’s frame autosave name.
pub type NSWindowFrameAutosaveName = NSString;

/// The type of a window’s frame descriptor.
pub type NSWindowPersistableFrameDescriptor = NSString;

/// Values that specify the reason for the `NSPopoverWillCloseNotification` notification.
pub type NSPopoverCloseReasonValue = NSString;

/// Named images, defined by the system or you, for use in your app.
pub type NSImageName = NSString;

/// The name of the storyboard file.
pub type NSStoryboardName = NSString;
