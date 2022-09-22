use std::marker::{PhantomData, Sized};

use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// An immutable object that combines a contact property value with a label that describes that property.
    unsafe pub struct CNLabeledValue<ValueType> {
        marker: PhantomData<ValueType>,
    }
}

/// A trait containing all the methods for [`ICNLabeledValue`]"
pub trait ICNLabeledValue<ValueType>: PNSObject
where
    ValueType: PNSObject + FromId,
{
    /// Returns a new labeled value identifier.
    fn m_init_with_label_value(&mut self, label: &NSString, value: ValueType) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                initWithLabel:label.m_self()
                value:value
            ])
        }
    }

    /// Returns a new labeled value identifier.
    fn m_labeled_value_with_label_value(label: &NSString, value: ValueType) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), labeledValueWithLabel:label.m_self() value:value],
            )
        }
    }

    /* Getting the Label and Value
     */

    /// The label for a contact property value.
    fn p_label(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), label]) }
    }

    /// A contact property value.
    fn p_value(&self) -> ValueType {
        unsafe { ValueType::from_id(msg_send![self.m_self(), value]) }
    }

    /* Setting Labels and Values
     */

    /// Returns a labeled value object with an existing value and identifier.
    fn m_labeled_value_by_setting_label(&self, label: &NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![self.m_self(), labeledValueBySettingLabel: label.m_self()])
        }
    }

    /// Returns a labeled value object with the specified label and value with the existing identifier.
    fn m_labeled_value_by_setting_label_value(&self, label: &NSString, value: ValueType) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                labeledValueBySettingLabel:label.m_self()
                value:value
            ])
        }
    }

    /// Returns a new value for an existing label and identifier.
    fn m_labeled_value_by_setting_value(&self, value: ValueType) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), valueBySettingValue: value]) }
    }

    /* Localizing the Label and Value
     */

    /// Returns a localized string for the specified label.
    fn m_localized_string_for_label(label: &NSString) -> NSString {
        unsafe {
            NSString::from_id(msg_send![Self::m_class(), localizedStringForLabel: label.m_self()])
        }
    }

    /// A unique identifier for the labeled value object.
    fn p_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }
}

impl<ValueType> ICNLabeledValue<ValueType> for CNLabeledValue<ValueType> where
    ValueType: PNSObject + FromId
{
}

impl<ValueType> CNLabeledValue<ValueType>
where
    ValueType: PNSObject + FromId,
{
    /// Returns a new labeled value identifier.
    pub fn init_with_label_value(&mut self, label: &NSString, value: ValueType) -> Self {
        self.m_init_with_label_value(label, value)
    }

    /// Returns a new labeled value identifier.
    pub fn labeled_value_with_label_value(label: &NSString, value: ValueType) -> Self {
        Self::m_labeled_value_with_label_value(label, value)
    }

    /* Getting the Label and Value
     */

    /// The label for a contact property value.
    pub fn label(&self) -> NSString {
        self.p_label()
    }

    /// A contact property value.
    pub fn value(&self) -> ValueType {
        self.p_value()
    }

    /* Setting Labels and Values
     */

    /// Returns a labeled value object with an existing value and identifier.
    pub fn labeled_value_by_setting_label(&self, label: &NSString) -> Self {
        self.m_labeled_value_by_setting_label(label)
    }

    /// Returns a labeled value object with the specified label and value with the existing identifier.
    pub fn labeled_value_by_setting_label_value(&self, label: &NSString, value: ValueType) -> Self {
        self.m_labeled_value_by_setting_label_value(label, value)
    }

    /// Returns a new value for an existing label and identifier.
    pub fn labeled_value_by_setting_value(&self, value: ValueType) -> Self {
        self.m_labeled_value_by_setting_value(value)
    }

    /* Localizing the Label and Value
     */

    /// Returns a localized string for the specified label.
    pub fn localized_string_for_label(label: &NSString) -> NSString {
        Self::m_localized_string_for_label(label)
    }

    /// A unique identifier for the labeled value object.
    pub fn identifier(&self) -> NSString {
        self.p_identifier()
    }
}

extern "C" {
    /* Getting Common Labels
     */

    /// The label for identifying home information.
    pub static CNLabelHome: NSString;

    /// The label for identifying work information.
    pub static CNLabelWork: NSString;

    /// The label for the contact’s school.
    pub static CNLabelSchool: NSString;

    /// The label for identifying other information.
    pub static CNLabelOther: NSString;

    /// The label for identifying the contact's iCloud email information.
    pub static CNLabelEmailiCloud: NSString;

    /// The label for identifying URL information.
    pub static CNLabelURLAddressHomePage: NSString;

    /// The label for identifying the contact's anniversary date.
    pub static CNLabelDateAnniversary: NSString;
}
