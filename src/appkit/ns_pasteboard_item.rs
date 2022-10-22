use objective_c_runtime_proc_macros::interface_impl;

use super::object;

use crate::objective_c_runtime::traits::PNSObject;

object! {
    unsafe pub struct NSPasteboardItem;
}

#[interface_impl(NSObject)]
impl NSPasteboardItem {}
