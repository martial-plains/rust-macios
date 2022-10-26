use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::macros::{interface_impl, object},
};

use super::ICNGroup;

object! {
    /// A mutable object that represents a group of contacts.
    unsafe pub struct CNMutableGroup;
}

impl ICNGroup for CNMutableGroup {}

#[interface_impl(CNGroup)]
impl CNMutableGroup {
    /// The name of the group.
    #[property]
    pub fn set_name(&mut self, name: NSString) {
        unsafe { msg_send![self.m_self(), setName: name] }
    }
}
