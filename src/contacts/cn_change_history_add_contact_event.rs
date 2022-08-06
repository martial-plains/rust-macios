use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    objective_c_runtime::{macros::object, traits::FromId},
};

use super::{CNContact, ICNChangeHistoryEvent};

object! {
    ///
    unsafe pub struct CNChangeHistoryAddContactEvent;
}

impl ICNChangeHistoryEvent for CNChangeHistoryAddContactEvent {}

#[interface_impl(CNChangeHistoryEvent)]
impl CNChangeHistoryAddContactEvent {
    ///
    #[property]
    pub fn contact(&self) -> CNContact {
        unsafe { CNContact::from_id(msg_send![self.m_self(), contact]) }
    }

    ///
    #[property]
    pub fn container_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), containerIdentifier]) }
    }
}
