use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{
        Int, NSArray, NSDictionary, NSError, NSErrorDomain, NSErrorUserInfoKey, NSString, UInt,
    },
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::INSDictionary;

/// Information about an error condition including a domain, a domain-specific
/// error code, and application-specific information.
pub trait INSError: PNSObject {
    /* Creating Error Objects
     */

    /// Creates and initializes an NSError object for a given domain and code with a given userInfo dictionary.
    fn tm_error_with_domain_code_user_info<Dictionary>(
        domain: NSErrorDomain,
        code: Int,
        dict: Dictionary,
    ) -> Self
    where
        Self: Sized + FromId,
        Dictionary: INSDictionary<NSErrorUserInfoKey, id>,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::im_class(), errorWithDomain: domain code: code userInfo: dict],
            )
        }
    }

    /// Returns an NSError object initialized for a given domain and code with a given userInfo dictionary.
    fn im_init_with_domain_code_user_info<Dictionary>(
        &mut self,
        domain: NSErrorDomain,
        code: Int,
        dict: Dictionary,
    ) -> Self
    where
        Self: Sized + FromId,
        Dictionary: INSDictionary<NSErrorUserInfoKey, id>,
    {
        unsafe {
            Self::from_id(
                msg_send![self.im_self(), initWithDomain: domain code: code userInfo: dict],
            )
        }
    }

    /* Getting Error Properties
     */

    /// The error code.
    fn ip_code(&self) -> Int {
        unsafe { msg_send![self.im_self(), code] }
    }

    /// A string containing the error domain.
    fn ip_domain(&self) -> NSErrorDomain {
        unsafe { msg_send![self.im_self(), domain] }
    }

    /// The user info dictionary.
    fn ip_user_info(&self) -> NSDictionary<NSErrorUserInfoKey, id> {
        unsafe { NSDictionary::from_id(msg_send![self.im_self(), userInfo]) }
    }

    /* Getting a Localized Error Description */

    /// A string containing the localized description of the error.
    fn ip_localized_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), localizedDescription]) }
    }

    /// An array containing the localized titles of buttons appropriate for displaying in an alert panel.
    fn ip_localized_recovery_options(&self) -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![self.im_self(), localizedRecoveryOptions]) }
    }

    /// A string containing the localized recovery suggestion for the error.
    fn ip_localized_recovery_suggestion(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), localizedRecoverySuggestion]) }
    }

    /// A string containing the localized explanation of the reason for the error.
    fn ip_localized_failure_reason(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), localizedFailureReason]) }
    }

    /// The object in the user info dictionary corresponding to the NSRecoveryAttempterErrorKey key.
    fn ip_recovery_attempter(&self) -> id {
        unsafe { msg_send![self.im_self(), recoveryAttempter] }
    }

    /// Implemented to attempt a recovery from an error noted in an application-modal dialog.
    fn im_attempt_recovery_from_error_option_index(
        &self,
        error: NSError,
        recovery_index: UInt,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.im_self(), attemptRecoveryFromError: error optionIndex: recovery_index],
            )
        }
    }

    /// A string to display in response to an alert panel help anchor button being pressed.
    fn im_help_anchor(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), helpAnchor]) }
    }

    ///
    fn ip_underlying_errors(&self) -> NSArray<NSError> {
        unsafe { NSArray::from_id(msg_send![self.im_self(), underlyingErrors]) }
    }
}
