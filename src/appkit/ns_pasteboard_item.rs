use super::interface_impl;

use crate::{object, objective_c_runtime::traits::PNSObject};

object! {
    unsafe pub struct NSPasteboardItem;
}

#[interface_impl(NSObject)]
impl NSPasteboardItem {}
