use std::marker::PhantomData;

use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSData,
    object,
    objective_c_runtime::traits::{FromId, PNSObject},
};

object! {
    ///
    unsafe pub struct CNFetchResult<ValueType> {
        marker: PhantomData<ValueType>,
    }
}

///
pub trait ICNFetchResult<ValueType>: PNSObject
where
    ValueType: PNSObject + FromId,
{
    ///
    fn p_current_history_token(&self) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), currentHistoryToken]) }
    }

    ///
    fn p_value(&self) -> ValueType {
        unsafe { ValueType::from_id(msg_send![self.m_self(), value]) }
    }
}

impl<ValueType> ICNFetchResult<ValueType> for CNFetchResult<ValueType> where
    ValueType: PNSObject + FromId
{
}

impl<ValueType> CNFetchResult<ValueType>
where
    ValueType: PNSObject + FromId,
{
    ///
    pub fn current_history_token(&self) -> NSData {
        self.p_current_history_token()
    }

    ///
    pub fn value(&self) -> ValueType {
        self.p_value()
    }
}
