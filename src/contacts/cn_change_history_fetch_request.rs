use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::macros::object;

use super::ICNFetchRequest;

object! {
    ///
    unsafe pub struct CNChangeHistoryFetchRequest;
}

impl ICNFetchRequest for CNChangeHistoryFetchRequest {}

#[interface_impl(CNFetchRequest)]
impl CNChangeHistoryFetchRequest {}
