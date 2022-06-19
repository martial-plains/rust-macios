use objc::{msg_send, sel, sel_impl};

use crate::{foundation::NSString, objective_c_runtime::traits::FromId};

use super::INSControl;

/// Text the user can select or edit to send an action message to a target when the user presses the Return key.
pub trait INSTextField: INSControl {
    /* Creating Text Fields
     */

    /// Initializes a text field for use as a static label that uses the system default font, doesn’t wrap, and doesn’t have selectable text.
    fn tm_label_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), labelWithString: string]) }
    }

    /// Initializes a single-line editable text field for user input using the system default font and standard visual appearance.
    fn tm_text_field_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), textFieldWithString: string]) }
    }

    /// Initializes a text field for use as a multiline static label with selectable text that uses the system default font.
    fn tm_text_view_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), textViewWithString: string]) }
    }   
}
