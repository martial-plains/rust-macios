use super::{
    macros::{interface_impl, object},
    traits::PNSObject,
};

object! {
    /// A simple container for a single C or Objective-C data item.
    unsafe pub struct NSValue;
}

#[interface_impl(NSObject)]
impl NSValue {}
