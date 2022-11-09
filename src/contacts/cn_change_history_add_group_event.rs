use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    object,
    objective_c_runtime::{macros::interface_impl, traits::FromId},
};

use super::{CNGroup, ICNChangeHistoryEvent};

object! {
    ///
    unsafe pub struct CNChangeHistoryAddGroupEvent;
}

impl ICNChangeHistoryEvent for CNChangeHistoryAddGroupEvent {}

#[interface_impl(CNChangeHistoryEvent)]
impl CNChangeHistoryAddGroupEvent {
    ///
    #[property]
    pub fn group(&self) -> CNGroup {
        unsafe { CNGroup::from_id(msg_send![self.m_self(), group]) }
    }

    ///
    #[property]
    pub fn container_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), containerIdentifier]) }
    }
}
