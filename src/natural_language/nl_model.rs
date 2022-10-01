use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    core_ml::MLModel,
    foundation::{NSArray, NSDictionary, NSError, NSNumber, NSString, UInt, NSURL},
    objective_c_runtime::{
        id,
        macros::object,
        nil,
        traits::{FromId, PNSObject},
    },
};

use super::NLModelConfiguration;

object! {
    /// A custom model trained to classify or tag natural language text.
    unsafe pub struct NLModel;
}

#[interface_impl(NSObject)]
impl NLModel {
    /* Creating a model
     */

    /// Creates a new natural language model based on the given Core ML model instance.
    #[method]
    pub fn model_with_mlmodel(model: &MLModel) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr = Self::from_id(
                msg_send![Self::m_class(), modelWithMLModel: model.m_self() error: &mut error],
            );

            if error.m_self() != nil {
                Err(error)
            } else {
                Ok(ptr)
            }
        }
    }

    /// Creates a new natural language model based on the given Core ML model instance.
    #[method]
    pub fn model_with_contents_of_url(url: &NSURL) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr = Self::from_id(
                msg_send![Self::m_class(), modelWithContentsOfURL: url.m_self() error: &mut error],
            );

            if error.m_self() != nil {
                Err(error)
            } else {
                Ok(ptr)
            }
        }
    }

    /* Making predictions
     */

    /// Predicts a label for the given input string.
    #[method]
    pub fn predicted_label_for_string(&self, string: &NSString) -> Option<NSString> {
        unsafe {
            let ptr: id = msg_send![self.m_self(), predictedLabelForString: string.m_self()];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// Predicts a label for each string in the given array.
    #[method]
    pub fn predicted_labels_for_tokens(&self, tokens: &NSArray<NSString>) -> NSArray<NSString> {
        unsafe {
            NSArray::from_id(msg_send![self.m_self(), predictedLabelsForTokens: tokens.m_self()])
        }
    }

    /// Predicts multiple possible labels for the given input string.
    #[method]
    pub fn predicted_label_hypotheses_for_string_maximum_count(
        &self,
        string: &NSString,
        max_count: UInt,
    ) -> NSDictionary<NSString, NSNumber> {
        unsafe {
            NSDictionary::from_id(
                msg_send![self.m_self(), predictedLabelHypothesesForString: string.m_self() maximumCount: max_count],
            )
        }
    }

    /// Predicts multiple possible labels for each token in the given array.
    #[method]
    pub fn predicted_label_hypotheses_for_tokens_maximum_count(
        &self,
        tokens: &NSArray<NSString>,
        max_count: UInt,
    ) -> NSArray<NSDictionary<NSString, NSNumber>> {
        unsafe {
            NSArray::from_id(
                msg_send![self.m_self(), predictedLabelHypothesesForTokens: tokens.m_self() maximumCount: max_count],
            )
        }
    }

    /* Inspecting a model
     */

    /// A configuration describing the natural language model.
    #[property]
    pub fn configuration(&self) -> NLModelConfiguration {
        unsafe { NLModelConfiguration::from_id(msg_send![self.m_self(), configuration]) }
    }
}
