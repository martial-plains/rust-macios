use objc::{msg_send, sel, sel_impl};

use crate::{foundation::NSString, objective_c_runtime::traits::FromId, object};

use super::{interface_impl,  INSControl, INSResponder, INSView};

object! {
    /// Text the user can select or edit to send an action message to a target when the user presses the Return key.
    unsafe pub struct NSTextField;
}

impl INSResponder for NSTextField {}

impl INSView for NSTextField {}

impl INSControl for NSTextField {}

#[interface_impl(NSControl)]
impl NSTextField {
    /* Creating Text Fields
     */

    /// Initializes a text field for use as a static label that uses the system default font, doesn’t wrap, and doesn’t have selectable text.
    #[method]
    pub fn label_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), labelWithString: string]) }
    }

    /// Initializes a single-line editable text field for user input using the system default font and standard visual appearance.
    #[method]
    pub fn text_field_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), textFieldWithString: string]) }
    }

    /// Initializes a text field for use as a multiline static label with selectable text that uses the system default font.
    #[method]
    pub fn text_view_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), textViewWithString: string]) }
    }
}
