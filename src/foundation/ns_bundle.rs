use crate::objective_c_runtime::macros::object;

use super::{traits::INSBundle, NSArray};

object! {
    /// A representation of the code and resources stored in a bundle directory on disk.
    unsafe pub struct NSBundle;
}

impl INSBundle for NSBundle {}

impl NSBundle {
    /// Returns the bundle object that contains the current executable.
    pub fn main_bundle() -> NSBundle {
        NSBundle::tp_main_bundle()
    }

    /// Returns an array of all of the application’s bundles that represent frameworks.
    pub fn all_frameworks() -> NSArray<NSBundle> {
        NSBundle::tp_all_frameworks()
    }

    /// Returns an array of all the application’s non-framework bundles.
    pub fn all_bundles() -> NSArray<NSBundle> {
        NSBundle::tp_all_bundles()
    }
}
