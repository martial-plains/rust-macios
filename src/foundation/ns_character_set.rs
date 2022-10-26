use objc::{msg_send, sel, sel_impl};

use crate::{
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{unichar, NSCoder, NSData, NSRange, NSString, UInt8};

object! {
    /// A character set containing the characters in Unicode General Categories L*, M*, and N*.
    unsafe pub struct NSCharacterSet;
}

#[interface_impl(NSObject)]
impl NSCharacterSet {
    /* Getting Standard Character Sets
     */

    /// A character set containing the characters in Unicode General Categories L*, M*, and N*.
    #[property]
    pub fn alphanumeric_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), alphanumericCharacterSet]) }
    }

    /// A character set containing the characters in Unicode General Category Lt.
    #[property]
    pub fn capitalized_letter_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![Self::m_class(), capitalizedLetterCharacterSet])
        }
    }

    /// A character set containing the characters in Unicode General Category Cc and Cf.
    #[property]
    pub fn control_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), controlCharacterSet]) }
    }

    /// A character set containing the characters in the category of Decimal Numbers.
    #[property]
    pub fn decimal_digit_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), decimalDigitCharacterSet]) }
    }

    /// A character set containing individual Unicode characters that can also be represented as composed character sequences (such as for letters with accents), by the definition of “standard decomposition” in version 3.2 of the Unicode character encoding standard.
    #[property]
    pub fn decomposable_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), decomposableCharacterSet]) }
    }

    /// A character set containing values in the category of Non-Characters or that have not yet been defined in version 3.2 of the Unicode standard.
    #[property]
    pub fn illegal_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), illegalCharacterSet]) }
    }

    /// A character set containing the characters in Unicode General Category L* & M*.
    #[property]
    pub fn letter_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), letterCharacterSet]) }
    }

    /// A character set containing the characters in Unicode General Category Ll.
    #[property]
    pub fn lowercase_letter_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), lowercaseLetterCharacterSet]) }
    }

    /// A character set containing the newline characters (U+000A ~ U+000D, U+0085, U+2028, and U+2029).
    #[property]
    pub fn newline_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), newlineCharacterSet]) }
    }

    /// A character set containing the characters in Unicode General Category M*.
    #[property]
    pub fn non_base_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), nonBaseCharacterSet]) }
    }

    /// A character set containing the characters in Unicode General Category P*.
    #[property]
    pub fn punctuation_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), punctuationCharacterSet]) }
    }

    /// A character set containing the characters in Unicode General Category S*.
    #[property]
    pub fn symbol_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), symbolCharacterSet]) }
    }

    /// A character set containing the characters in Unicode General Category Lu and Lt.
    #[property]
    pub fn uppercase_letter_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), uppercaseLetterCharacterSet]) }
    }

    /// A character set containing characters in Unicode General Category Z*, U+000A ~ U+000D, and U+0085.
    #[property]
    pub fn whitespace_and_newline_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![Self::m_class(), whitespaceAndNewlineCharacterSet])
        }
    }

    /// A character set containing the characters in Unicode General Category Zs and CHARACTER TABULATION (U+0009).
    #[property]
    pub fn whitespace_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), whitespaceCharacterSet]) }
    }

    /* Getting Character Sets for URL Encoding
     */

    /// Returns the character set for characters allowed in a fragment URL component.
    #[property]
    pub fn url_fragment_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![Self::m_class(), URLFragmentAllowedCharacterSet])
        }
    }

    /// Returns the character set for characters allowed in a host URL subcomponent.
    #[property]
    pub fn url_host_allowed_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), URLHostAllowedCharacterSet]) }
    }

    /// Returns the character set for characters allowed in a password URL subcomponent.
    #[property]
    pub fn url_password_allowed_character_set() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![Self::m_class(), URLPasswordAllowedCharacterSet])
        }
    }

    /// Returns the character set for characters allowed in a path URL component.
    #[property]
    pub fn url_path_allowed_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), URLPathAllowedCharacterSet]) }
    }

    /// Returns the character set for characters allowed in a query URL component.
    #[property]
    pub fn url_query_allowed_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), URLQueryAllowedCharacterSet]) }
    }

    /// Returns the character set for characters allowed in a user URL subcomponent.
    #[property]
    pub fn url_user_allowed_character_set() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), URLUserAllowedCharacterSet]) }
    }

    /* Creating a Custom Character Set
     */

    /// Initializing with coder
    #[method]
    pub fn init_with_coder(mut self, coder: NSCoder) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithCoder: coder]) }
    }

    /// Returns a character set containing the characters in a given string.
    #[method]
    pub fn character_set_with_characters_in_string(string: NSString) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                Self::m_class(),
                characterSetWithCharactersInString: string
            ])
        }
    }

    /// Returns a character set containing characters with Unicode values in a given range.
    #[method]
    pub fn character_set_with_range(range: NSRange) -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![Self::m_class(), characterSetWithRange: range]) }
    }

    /* Creating and Managing Character Sets as Bitmap Representations
     */

    /// Returns a character set containing characters determined by a given bitmap representation.
    #[method]
    pub fn character_set_with_bitmap_representation(data: NSData) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                Self::m_class(),
                characterSetWithBitmapRepresentation: data
            ])
        }
    }

    /// Returns a character set read from the bitmap representation stored in the file a given path.
    #[method]
    pub fn character_set_with_contents_of_file(path: NSString) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                Self::m_class(),
                characterSetWithContentsOfFile: path
            ])
        }
    }

    /// An NSData object encoding the receiver in binary format.
    #[property]
    pub fn bitmap_representation(&self) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), bitmapRepresentation]) }
    }

    /// A character set containing only characters that don’t exist in the receiver.
    #[property]
    pub fn inverted_set(&self) -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![self.m_self(), invertedSet]) }
    }

    /* Testing Set Membership
     */

    /// Returns a Boolean value that indicates whether a given character is in the receiver.
    #[method]
    pub fn character_is_member(&self, character: unichar) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), characterIsMember: character]) }
    }

    /// Returns a Boolean value that indicates whether the receiver has at least one member in a given character plane.
    #[method]
    pub fn has_member_in_plane(&self, plane: UInt8) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), hasMemberInPlane: plane]) }
    }

    /// Returns a Boolean value that indicates whether the receiver is a superset of another given character set.
    #[method]
    pub fn is_superset_of_set(&self, other: NSCharacterSet) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isSupersetOfSet: other]) }
    }

    /// Returns a Boolean value that indicates whether a given long character is a member of the receiver.
    #[method]
    pub fn long_character_is_member(&self, long_char: u32) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), longCharacterIsMember: long_char]) }
    }
}
