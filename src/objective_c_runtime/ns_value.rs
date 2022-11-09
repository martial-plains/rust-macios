use crate::object;

use super::{macros::interface_impl, traits::PNSObject};

object! {
    /// A simple container for a single C or Objective-C data item.
    unsafe pub struct NSValue;
}

#[interface_impl(NSObject)]
impl NSValue {}
