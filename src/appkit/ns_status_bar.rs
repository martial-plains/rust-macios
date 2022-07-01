use super::{object, traits::INSStatusBar};

object! {
     /// An individual element displayed in the system menu bar.
    unsafe pub struct NSStatusBar;
}

impl INSStatusBar for NSStatusBar {}
