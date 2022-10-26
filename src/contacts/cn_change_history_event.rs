use objc::{msg_send, sel, sel_impl};

use crate::objective_c_runtime::{
    id,
    macros::{interface_impl, object},
    traits::PNSObject,
};

object! {
    ///
    unsafe pub struct CNChangeHistoryEvent;
}

#[interface_impl(NSObject)]
impl CNChangeHistoryEvent {
    ///
    #[property]
    pub fn accept_event_visitor(&self, visitor: id) {
        unsafe { msg_send![self.m_self(), acceptEventVisitor: visitor] }
    }
}
