use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{Int, NSArray, NSDictionary, NSErrorDomain, NSErrorUserInfoKey, NSString, UInt};

object! {
    /// Information about an error condition including a domain, a domain-specific
    /// error code, and application-specific information.
    unsafe pub struct NSError;
}

#[interface_impl(NSObject)]
impl NSError {
    /* Creating Error Objects
     */

    /// Creates and initializes an NSError object for a given domain and code with a given userInfo dictionary.
    #[method]
    pub fn error_with_domain_code_user_info(
        domain: NSErrorDomain,
        code: Int,
        dict: NSDictionary<NSErrorUserInfoKey, id>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), errorWithDomain: domain code: code userInfo: dict],
            )
        }
    }

    /// Returns an NSError object initialized for a given domain and code with a given userInfo dictionary.
    #[method]
    pub fn init_with_domain_code_user_info(
        &mut self,
        domain: NSErrorDomain,
        code: Int,
        dict: NSDictionary<NSErrorUserInfoKey, id>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithDomain: domain code: code userInfo: dict],
            )
        }
    }

    /* Getting Error Properties
     */

    /// The error code.
    #[property]
    pub fn code(&self) -> Int {
        unsafe { msg_send![self.m_self(), code] }
    }

    /// A string containing the error domain.
    #[property]
    pub fn domain(&self) -> NSErrorDomain {
        unsafe { msg_send![self.m_self(), domain] }
    }

    /// The user info dictionary.
    #[property]
    pub fn user_info(&self) -> NSDictionary<NSErrorUserInfoKey, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), userInfo]) }
    }

    /* Getting a Localized Error Description */

    /// A string containing the localized description of the error.
    #[property]
    pub fn localized_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedDescription]) }
    }

    /// An array containing the localized titles of buttons appropriate for displaying in an alert panel.
    #[property]
    pub fn localized_recovery_options(&self) -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), localizedRecoveryOptions]) }
    }

    /// A string containing the localized recovery suggestion for the error.
    #[property]
    pub fn localized_recovery_suggestion(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedRecoverySuggestion]) }
    }

    /// A string containing the localized explanation of the reason for the error.
    #[property]
    pub fn localized_failure_reason(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedFailureReason]) }
    }

    /// The object in the user info dictionary corresponding to the NSRecoveryAttempterErrorKey key.
    #[property]
    pub fn recovery_attempter(&self) -> id {
        unsafe { msg_send![self.m_self(), recoveryAttempter] }
    }

    /// Implemented to attempt a recovery from an error noted in an application-modal dialog.
    #[method]
    pub fn attempt_recovery_from_error_option_index(
        &self,
        error: NSError,
        recovery_index: UInt,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), attemptRecoveryFromError: error optionIndex: recovery_index],
            )
        }
    }

    /// A string to display in response to an alert panel help anchor button being pressed.
    #[method]
    pub fn help_anchor(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), helpAnchor]) }
    }

    ///
    #[property]
    pub fn underlying_errors(&self) -> NSArray<NSError> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), underlyingErrors]) }
    }
}
