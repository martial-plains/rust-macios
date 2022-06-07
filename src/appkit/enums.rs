/// Constants that indicate whether a copy or print operation was successful,
/// was canceled, or failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSApplicationDelegateReply {
    /// Indicates the operation succeeded.
    Success = 0,
    /// Indicates the user cancelled the operation.
    Cancel = 1,
    /// Indicates an error occurred processing the operation.
    Failure = 2,
}

/// Constants for the types of events that responder objects can handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSEventType {
    /// The user pressed the left mouse button.
    LeftMouseDown = 1,
    /// The user released the left mouse button.
    LeftMouseUp = 2,
    /// The user pressed the right mouse button.
    RightMouseDown = 3,
    /// The user released the right mouse button.
    RightMouseUp = 4,
    /// The user moved the mouse in a way that caused the cursor to move onscreen.
    MouseMoved = 5,
    /// The user moved the mouse while holding down the left mouse button.
    LeftMouseDragged = 6,
    /// The user moved the mouse while holding down the right mouse button.
    RightMouseDragged = 7,
    /// The cursor entered a well-defined area, such as a view.
    MouseEntered = 8,
    /// The cursor exited a well-defined area, such as a view.
    MouseExited = 9,
    /// The user pressed a key on the keyboard.
    KeyDown = 10,
    /// The user released a key on the keyboard.
    KeyUp = 11,
    /// The event flags changed.
    FlagsChanged = 12,
    /// An AppKit-related event occurred.
    AppKitDefined = 13,
    /// A system-related event occurred.
    SystemDefined = 14,
    /// An app-defined event occurred.
    ApplicationDefined = 15,
    /// An event that provides execution time to periodic tasks.
    Periodic = 16,
    /// An event that updates the cursor.
    CursorUpdate = 17,

    /// The scroll wheel position changed.
    ScrollWheel = 22,

    /// The user touched a point on a tablet.
    TabletPoint = 23,
    /// A pointing device is near, but not touching, the associated tablet.
    TabletProximity = 24,

    /// The user pressed a tertiary mouse button.
    OtherMouseDown = 25,
    /// The user released a tertiary mouse button.
    OtherMouseUp = 26,
    /// The user moved the mouse while holding down a tertiary mouse button.
    OtherMouseDragged = 27,

    /// The user performed a nonspecific type of gesture.
    Gesture = 29,
    /// The user performed a pinch-open or pinch-close gesture.
    Magnify = 30,
    /// The user performed a swipe gesture.
    Swipe = 31,
    /// The user performed a rotate gesture.
    Rotate = 18,
    /// An event marking the beginning of a gesture.
    BeginGesture = 19,
    /// An event that marks the end of a gesture.
    EndGesture = 20,

    /// The user performed a smart-zoom gesture.
    SmartMagnify = 32,
    /// An event that initiates a Quick Look request.
    QuickLook = 33,
    /// An event that reports a change in pressure on a pressure-sensitive device.
    Pressure = 34, // 10.10.3, 64-bit-only
    /// The user touched a portion of the touch bar.
    DirectTouch = 37, // 10.10
    /// The user changed the mode of a connected device.
    ChangeMode = 38,
}

