use super::{
    object,
    traits::{INSResponder, INSView},
};

object! {
    /// The infrastructure for drawing, printing, and handling events in an app.
    unsafe pub struct NSView;
}

impl INSResponder for NSView {}

impl INSView for NSView {}
