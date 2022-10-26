use crate::objective_c_runtime::macros::{interface_impl, object};

use super::ICNFetchRequest;

object! {
    ///
    unsafe pub struct CNChangeHistoryFetchRequest;
}

impl ICNFetchRequest for CNChangeHistoryFetchRequest {}

#[interface_impl(CNFetchRequest)]
impl CNChangeHistoryFetchRequest {}
