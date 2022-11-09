use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    object,
    objective_c_runtime::{macros::interface_impl, traits::FromId},
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
