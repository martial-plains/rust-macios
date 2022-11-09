use objc::{msg_send, sel, sel_impl};

use crate::{object, 
    objective_c_runtime::{
        id,
        macros::{interface_impl},
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{
    NSAttributedString, NSAttributedStringKey, NSDictionary, NSRange, NSRangePointer, NSString,
};

object! {
    /// An abstract class that declares an interface for objects that create,
    /// interpret, and validate the textual representation of values.
    unsafe pub struct NSFormatter;
}

#[interface_impl(NSObject)]
impl NSFormatter {
    /* Getting Textual Representations of Object Values
     */

    /// The default implementation of this method raises an exception.
    ///
    /// # Arguments
    ///
    /// * `obj` - The object for which a textual representation is returned.
    #[method]
    pub fn string_for_object_value(&self, obj: id) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), stringForObjectValue: obj]) }
    }

    /// The default implementation returns nil to indicate that the formatter object does not provide an attributed string.
    ///
    /// # Arguments
    ///
    /// * `obj` - The object for which a textual representation is returned.
    /// * `attrs` - The default attributes to use for the returned attributed string.
    #[method]
    pub fn attributed_string_for_object_value_with_default_attributes(
        &self,
        obj: id,
        attrs: NSDictionary<NSAttributedStringKey, id>,
    ) -> NSAttributedString {
        unsafe {
            NSAttributedString::from_id(
                msg_send![self.m_self(), attributedStringForObjectValue: obj withDefaultAttributes: attrs],
            )
        }
    }

    /// The default implementation of this method invokes string_for_object_value.
    ///
    /// # Arguments
    ///
    /// * `obj` - The object for which to return an editing string.
    #[method]
    pub fn editing_string_for_object_value(&self, obj: id) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), editingStringForObjectValue: obj]) }
    }

    /* Getting Object Values for Textual Representations
     */

    /// The default implementation of this method raises an exception.
    ///
    /// # Arguments
    ///
    /// * `obj` - If conversion is successful, upon return contains the object created from string.
    /// * `string` - The string to parse.
    /// * `error` - If non-nil, if there is a error during the conversion, upon return contains an NSString object that describes the problem.
    #[method]
    pub fn get_object_value_for_string_error_description(
        &self,
        obj: &mut id,
        string: NSString,
        error: &mut NSString,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), getObjectValue:obj forString:string errorDescription: error ],
            )
        }
    }

    /// Returns a Boolean value that indicates whether a partial string is valid.
    ///
    /// # Arguments
    ///
    /// * `partial_string` - The text currently in a cell.
    /// * `new_string` - If partial_string needs to be modified, upon return contains the replacement string.
    /// * `error` - If non-nil, if validation fails contains an [`NSString`] object that describes the problem.
    #[method]
    pub fn is_partial_string_valid_new_editing_string_error_description(
        &self,
        partial_string: NSString,
        new_string: &mut NSString,
        error: &mut NSString,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), isPartialStringValid: partial_string newEditingString: new_string errorDescription: error],
            )
        }
    }

    /// This method should be implemented in subclasses that want to validate user changes to a string in a
    /// field, where the user changes are not necessarily at the end of the string, and preserve the selection
    /// (or set a different one, such as selecting the erroneous part of the string the user has typed).
    #[method]
    pub fn is_partial_string_valid_proposed_selected_range_original_string_original_selected_range_error_description(
        &self,
        partial_string_ptr: NSString,
        proposed_sel_range_ptr: NSRangePointer,
        orig_string: NSString,
        orig_sel_range: NSRange,
        error: &mut NSString,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), isPartialStringValid: partial_string_ptr proposedSelectedRange: proposed_sel_range_ptr originalString: orig_string originalSelectedRange: orig_sel_range errorDescription: error],
            )
        }
    }
}
