use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSArray, NSData, NSDictionary, NSError, NSString, NSURL},
    object,
    objective_c_runtime::{
        macros::interface_impl,
        nil,
        traits::{FromId, PNSObject},
    },
    utils::{to_bool, to_optional},
};

use super::NLLanguage;

object! {
    /// A collection of terms and their labels, which take precedence over a word tagger.
    unsafe pub struct NLGazetteer;
}

#[interface_impl(NSObject)]
impl NLGazetteer {
    /* Creating a Gazetteer
     */

    /// Creates a Natural Language gazetteer from a model created with the Create ML framework.
    #[method]
    pub fn init_with_contents_of_url(url: &NSURL) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr = Self::from_id(
                msg_send![Self::m_class(), initWithContentsOfURL:url.m_self() error: &mut error],
            );

            if error.m_self() != nil {
                Err(error)
            } else {
                Ok(ptr)
            }
        }
    }

    /// Creates a gazetteer from a data instance.
    #[method]
    pub fn init_with_data(&mut self, data: &NSData) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr = Self::from_id(
                msg_send![self.m_self(), initWithData: data.m_self() error: &mut error],
            );

            if error.m_self() != nil {
                Err(error)
            } else {
                Ok(ptr)
            }
        }
    }

    /// Creates a gazetteer from a set of labels for terms represented by a dictionary.
    #[method]
    pub fn init_with_dictionary_language(
        &mut self,
        dictionary: &NSDictionary<NSString, NSArray<NSString>>,
        language: Option<NLLanguage>,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let language = match language {
            Some(value) => value.m_self(),
            None => nil,
        };

        let mut error = NSError::m_alloc();

        unsafe {
            let ptr = Self::from_id(
                msg_send![self.m_self(),  initWithDictionary:dictionary.m_self() language: language error: &mut error],
            );

            if error.m_self() != nil {
                Err(error)
            } else {
                Ok(ptr)
            }
        }
    }

    /// Creates a gazetteer from a set of labels for terms represented by a dictionary and saves the gazetteer to a file.
    #[method]
    pub fn write_gazetteer_for_dictionary_language_to_url(
        dictionary: &NSDictionary<NSString, NSArray<NSString>>,
        language: Option<NLLanguage>,
        url: &NSURL,
    ) -> Result<bool, NSError> {
        let mut error = NSError::m_alloc();

        unsafe {
            let ptr = to_bool(
                msg_send![Self::m_class(),  writeGazetteerForDictionary:dictionary.m_self() language: language toURL: url.m_self() error: &mut error],
            );

            if error.m_self() != nil {
                Err(error)
            } else {
                Ok(ptr)
            }
        }
    }

    /* Looking Up Labels for Terms
     */

    /// Retrieves the label for the given term.
    #[method]
    pub fn label_for_string(&self, string: &NSString) -> Option<NSString> {
        unsafe { to_optional(msg_send![self.m_self(), labelForString: string.m_self()]) }
    }

    /* Inspecting a Gazetteer
     */

    /// The gazetteer represented as a data instance.
    #[property]
    pub fn data(&self) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), data]) }
    }

    /// The gazetteer represented as a data instance.
    #[property]
    pub fn set_data(&mut self, data: NSData) -> NSData {
        unsafe { msg_send![self.m_self(), setData: data] }
    }

    /// The language of the gazetteer.
    #[property]
    pub fn language(&self) -> Option<NLLanguage> {
        unsafe { to_optional(msg_send![self.m_self(), language]) }
    }
}
