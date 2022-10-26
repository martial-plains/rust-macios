use super::{interface_impl, object};

use crate::objective_c_runtime::traits::PNSObject;

object! {
    unsafe pub struct NSPasteboardItem;
}

#[interface_impl(NSObject)]
impl NSPasteboardItem {}
