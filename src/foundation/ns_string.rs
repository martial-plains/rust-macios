use std::{
    ffi::{CStr, CString},
    fmt,
    marker::PhantomData,
    string::String,
};

use libc::{c_char, c_void};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::{Id, ShareId};

use crate::{
    foundation::NSComparisonResult,
    objective_c_runtime::{
        self, id,
        macros::interface_impl,
        nil,
        traits::{FromId, PNSObject, ToId},
    },
    utils::{to_bool, to_optional},
};

use super::{
    string::Encoding, Int, NSArray, NSCharacterSet, NSData, NSError, NSLocale, NSMutableString,
    NSRange, NSStringCompareOptions, NSStringEncodingConversionOptions, NSStringTransform, UInt,
    NSURL, UTF8_ENCODING,
};

/// Type for UTF-16 code units.
#[allow(non_camel_case_types)]
pub type unichar = u16;

/// This is a mapping to the Objective-C NSString class.
#[repr(C)]
pub struct NSString {
    /// The raw pointer to the Objective-C object.
    pub ptr: ShareId<Object>,
    marker: PhantomData<()>,
}

impl NSString {
    /// Returns the UTF8 bytes for this `NSString`.
    pub fn bytes(&self) -> *const c_char {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.ptr, UTF8String];
            bytes
        }
    }

    /// Convert this `NSString` into a `&str`.
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let bytes = self.bytes();

        unsafe {
            let bytes = CStr::from_ptr(bytes);
            bytes.to_str()
        }
    }
}

#[interface_impl(NSObject)]
impl NSString {
    /* Creating and Initializing Strings
     */

    /// Returns an empty string.
    #[method]
    pub fn string() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), string]) }
    }

    /// Returns an initialized NSString object that contains no characters.
    #[method]
    pub fn init(mut self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), init]) }
    }

    /// Returns an initialized NSString object containing a given number of bytes from a given
    /// buffer of bytes interpreted in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A buffer of bytes interpreted in the encoding specified by encoding.
    ///
    /// * `len` - The number of bytes to use from bytes.
    ///
    /// * `encoding` - The character encoding applied to bytes. For possible values, see NSStringEncoding.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length bytes from bytes interpreted using the encoding
    /// encoding. The returned object may be different from the original receiver.
    #[method]
    pub fn init_with_bytes_length_encoding(
        mut self,
        bytes: *const c_void,
        len: UInt,
        encoding: Encoding,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithBytes: bytes length: len encoding: encoding],
            )
        }
    }

    /// Returns an initialized NSString object that contains a given number of bytes from a given buffer of bytes interpreted in a given encoding, and optionally frees the buffer.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A buffer of bytes interpreted in the encoding specified by encoding.
    /// * `len` - The number of bytes to use from bytes.
    /// * `encoding` - The character encoding applied to bytes. For possible values, see NSStringEncoding.
    /// * `free_buffer` - If YES, bytes is freed after use.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length bytes from bytes interpreted using the encoding encoding. The returned object may be different from the original receiver.
    #[method]
    pub fn init_with_bytes_no_copy_length_encoding_free_when_done(
        mut self,
        bytes: *mut c_void,
        len: UInt,
        encoding: Encoding,
        free_buffer: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                initWithBytesNoCopy: bytes length: len encoding: encoding freeWhenDone: free_buffer
            ])
        }
    }

    /// Returns an initialized NSString object that contains a given number of characters from
    /// a given C array of UTF-16 code units.
    ///
    /// # Arguments
    ///
    /// * `characters` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `length` - A C array of UTF-16 code units; the value must not be NULL.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length characters from characters.
    #[method]
    pub fn init_with_characters_length(mut self, characters: *const unichar, len: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![self.m_self(), initWithCharacters: characters length: len])
        }
    }

    /// Returns an initialized NSString object that contains a given number of characters
    /// from a given C array of UTF-16 code units.
    ///
    /// # Arguments
    ///
    /// * `characters` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `length` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `free_buffer` - If YES, characters is freed after use.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length characters from characters.
    #[method]
    pub fn init_with_characters_no_copy_length_free_when_done(
        mut self,
        characters: unichar,
        length: UInt,
        free_buffer: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                initWithCharactersNoCopy: characters length: length freeWhenDone: free_buffer
            ])
        }
    }

    /// Returns an NSString object initialized by copying the characters from another given string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string from which to copy characters. This value must not be nil.
    ///
    /// # Returns
    ///
    /// An NSString object initialized by copying the characters from s.
    #[method]
    pub fn init_with_string<S>(mut self, s: S) -> Self
    where
        Self: Sized + FromId,
        S: INSString,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithString: s]) }
    }
    /// Returns an NSString object initialized using the characters in a given C array,
    /// interpreted according to a given encoding.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    /// * `encoding` - The encoding to use to interpret c_str. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// An NSString object initialized using the characters in c_str,
    /// interpreted according to encoding.
    #[method]
    pub fn init_with_cstring_encoding(mut self, c_str: *const c_char, encoding: Encoding) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![self.m_self(), initWithCString: c_str encoding: encoding])
        }
    }

    /// Returns an NSString object initialized using the characters in a given C array,
    /// interpreted according to a UTF8 string.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    ///
    /// # Returns
    ///
    /// An NSString object initialized using the characters in c_str,
    /// interpreted as a UTF8 string.
    #[method]
    pub fn init_with_utf8_string(mut self, c_str: *const c_char) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUTF8String: c_str]) }
    }

    /// Returns an NSString object initialized by converting given data into
    /// UTF-16 code units using a given encoding.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to convert into UTF-16 code units.
    /// * `encoding` - The encoding to use to interpret data. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// An NSString object initialized by converting data into UTF-16 code units using encoding.
    #[method]
    pub fn init_with_data_encoding(mut self, data: NSData, encoding: Encoding) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithData: data encoding: encoding]) }
    }

    /// Returns a localized string intended for display in a notification alert.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to use when looking up the string in the app’s Localizable.strings file.
    /// * `arguments` - An array of values to substitute for escaped characters in the string.
    #[method]
    pub fn localized_user_notification_string_for_key_arguments<T>(
        key: NSString,
        arguments: NSArray<T>,
    ) -> NSString
    where
        T: PNSObject,
    {
        unsafe {
            msg_send![Self::m_class(), localizedUserNotificationStringForKey:key arguments:arguments]
        }
    }

    /// Returns a string containing a given number of characters taken from a
    /// given C array of UTF-16 code units.
    ///
    /// # Arguments
    ///
    /// * `characters` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `length` - A C array of UTF-16 code units; the value must not be NULL.
    ///
    /// # Returns
    ///
    /// An NSString object containing length characters from characters.
    #[method]
    pub fn string_with_characters_length(characters: *const unichar, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![Self::m_class(), stringWithCharacters:characters length:length])
        }
    }

    /// Returns a string created by copying the characters from another given string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string from which to copy characters. This value must not be nil.
    ///
    /// # Returns
    ///
    /// An NSString object initialized by copying the characters from s.
    #[method]
    pub fn string_with_string(s: NSString) -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { msg_send![Self::m_class(), stringWithString: s] }
    }

    /// Returns a string containing the bytes in a given C array, interpreted
    /// according to a given encoding.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    /// * `encoding` - The encoding to use to interpret c_str. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// An NSString object containing the bytes in c_str, interpreted according to encoding.
    #[method]
    pub fn string_with_cstring_encoding(c_str: *const c_char, encoding: Encoding) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![Self::m_class(), stringWithCString:c_str encoding:encoding])
        }
    }

    /// Returns a string created by copying the data from a given C array of
    /// UTF8-encoded bytes.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    ///
    /// # Returns
    ///
    /// An NSString object containing the bytes in c_str, interpreted as a UTF8 string.
    #[method]
    pub fn string_with_utf8_string(c_str: *const c_char) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), stringWithUTF8String: c_str]) }
    }

    /* Creating and Initializing a String from a File
     */

    /// Returns a string created by reading data from the file at a given path interpreted using a given encoding.
    #[method]
    pub fn string_with_contents_of_file_encoding(
        path: &NSString,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![Self::m_class(), stringWithContentsOfFile: path.m_self() encoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /// Returns an [`NSString`] object initialized by reading data from the file at a given path using a given encoding.
    #[method]
    pub fn init_with_contents_of_file_encoding(
        &mut self,
        path: &NSString,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithContentsOfFile: path.m_self() encoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /// Returns an [`NSString`] object initialized by reading data from the file at a given path using a given encoding.
    #[method]
    pub fn string_with_contents_of_file_used_encoding(
        path: &NSString,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![Self::m_class(), stringWithContentsOfFile: path.m_self() usedEncoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /// Returns a string created by reading data from the file at a given path and returns by reference the encoding used to interpret the file.
    #[method]
    pub fn init_with_contents_of_file_used_encoding(
        &mut self,
        path: &NSString,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithContentsOfFile: path.m_self() usedEncoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /* Creating and Initializing a String from an URL
     */

    /// Returns a string created by reading data from a given URL interpreted using a given encoding.
    #[method]
    pub fn string_with_contents_of_url_encoding(
        path: &NSURL,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![Self::m_class(), stringWithContentsOfURL: path.m_self() encoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /// Returns an [`NSString`] object initialized by reading data from a given URL interpreted using a given encoding.
    #[method]
    pub fn init_with_contents_of_url_encoding(
        &mut self,
        path: &NSURL,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithContentsOfURL: path.m_self() encoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /// Returns a string created by reading data from a given URL and returns by reference the encoding used to interpret the data.
    #[method]
    pub fn string_with_contents_of_url_used_encoding(
        path: &NSURL,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![Self::m_class(), stringWithContentsOfURL: path.m_self() usedEncoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /// Returns an [`NSString`] object initialized by reading data from a given URL and returns by reference the encoding used to interpret the data.
    #[method]
    pub fn init_with_contents_of_url_used_encoding(
        &mut self,
        path: &NSURL,
        enc: Encoding,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let result = unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithContentsOfURL: path.m_self() usedEncoding: enc error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(result)
        }
    }

    /* Getting a String’s Length
     */

    /// The number of Unicode characters in this string.
    #[property]
    pub fn length(&self) -> UInt {
        unsafe { msg_send![self.m_self(), length] }
    }

    /// Returns the number of bytes required to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    ///
    /// # Returns
    ///
    /// The number of bytes required to store the receiver in the encoding enc in a
    /// non-external representation. The length does not include space for a terminating
    /// NULL character. Returns 0 if the specified encoding cannot be used to convert
    /// the receiver or if the amount of memory required for storing the results of the
    /// encoding conversion would exceed NSIntegerMax.
    #[method]
    pub fn length_of_bytes_using_encoding(&self, enc: Encoding) -> UInt {
        unsafe { msg_send![self.m_self(), lengthOfBytesUsingEncoding: enc] }
    }

    /// Returns the maximum number of bytes needed to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    ///
    /// # Returns
    ///
    /// The maximum number of bytes needed to store the receiver in encoding in a non-external
    /// representation. The length does not include space for a terminating NULL character.
    /// Returns 0 if the amount of memory required for storing the results of the encoding
    /// conversion would exceed NSIntegerMax.
    #[method]
    pub fn maximum_length_of_bytes_using_encoding(&self, enc: Encoding) -> Int {
        unsafe { msg_send![self.m_self(), maximumLengthOfBytesUsingEncoding: enc] }
    }

    /* Getting Characters and Bytes
     */

    /// Returns the character at a given UTF-16 code unit index.
    ///
    /// # Arguments
    ///
    /// * `index` - The character at the array position given by `index`.
    ///
    /// # Returns
    ///
    /// The character at the array position given by index.
    #[method]
    pub fn character_at_index(&self, index: UInt) -> char {
        unsafe { msg_send![self.m_self(), characterAtIndex: index] }
    }

    /// Copies characters from a given range in the receiver into a given buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Upon return, contains the characters from the receiver. buffer must be large enough to contain the characters in the range aRange (aRange.length*sizeof(unichar)).
    /// * `range` - The range of characters to copy.
    #[method]
    pub fn get_characters_range(&self, buffer: *mut unichar, range: NSRange) {
        unsafe { msg_send![self.m_self(), getCharacters: buffer range: range] }
    }

    /// Gets a given range of characters as bytes in a specified encoding.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Upon return, contains the bytes from the receiver. buffer must be large enough to contain the bytes in the range aRange (aRange.length*sizeof(char)).
    /// * `maxLength - The maximum number of bytes to get.
    /// * `usedLength` - Upon return, contains the actual number of bytes retrieved.
    /// * `encoding` - The encoding to use to interpret the bytes. For possible values, see `NSStringEncoding`.
    /// * `options` - The options to use for converting the receiver into the specified encoding. For possible values, see `NSStringEncodingConversionOptions`.
    /// * `range` - The range of characters to get.
    /// * `remainingRange` - Upon return, contains a range containing the remaining characters.
    #[method]
    #[allow(clippy::too_many_arguments)]
    pub fn get_bytes_max_length_used_length_encoding_options_range_remaining_range(
        &self,
        buffer: *mut c_void,
        max_length: Int,
        used_length: *mut Int,
        encoding: Encoding,
        options: NSStringEncodingConversionOptions,
        range: NSRange,
        remaining_range: NSRange,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), getBytes: buffer maxLength: max_length usedLength: used_length encoding: encoding options: options range: range remainingRange: remaining_range],
            )
        }
    }

    /* Getting C Strings
     */

    /// Returns a representation of the string as a C string using a given encoding.
    ///
    /// # Arguments
    ///
    /// * `encoding` - The encoding to use to interpret the receiver. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// A pointer to a C string containing the receiver. The pointer is owned by the string, and must not be freed by the caller.
    #[method]
    pub fn c_string_using_encoding(&self, encoding: Encoding) -> *const c_char {
        unsafe { msg_send![self.m_self(), cStringUsingEncoding: encoding] }
    }

    /// Returns a representation of the string as a C string.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Upon return, contains the characters from the receiver. buffer must be large enough to contain the characters in the receiver.
    /// * `maxLength` - The maximum number of bytes to write into buffer.
    /// * `encoding` - The encoding to use to interpret the receiver. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// `true` if the operation was successful, otherwise `false`. Returns `false` if conversion is not possible due to encoding errors or if buffer is too small.
    #[method]
    pub fn get_cstring_max_length_encoding(
        &self,
        buffer: *mut c_char,
        max_length: UInt,
        encoding: Encoding,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), getCString: buffer maxLength: max_length encoding: encoding],
            )
        }
    }

    /// A null-terminated UTF8 representation of the string.
    #[property]
    pub fn utf8_string(&self) -> *const c_char {
        unsafe { msg_send![self.m_self(), UTF8String] }
    }

    /* Identifying and Comparing Strings
     */

    /// Returns the result of invoking compare:options: with NSCaseInsensitiveSearch as the only option.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`.
    #[method]
    pub fn case_insensitive_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), caseInsensitiveCompare: string] }
    }

    /// Compares the string with a given string using a case-insensitive, localized, comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    #[method]
    pub fn localized_case_insensitive_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), localizedCaseInsensitiveCompare: string] }
    }

    /// Returns the result of invoking compare:options:range: with no options
    /// and the receiver’s full extent as the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string with which to compare the receiver.
    ///
    /// # Safety
    ///
    /// This value must not be nil. If this value is nil, the behavior is
    /// undefined and may change in future versions of macOS.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    #[method]
    pub fn compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), compare: string] }
    }

    /// Returns the result of invoking compare:options:range: with no options
    /// and the receiver’s full extent as the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string with which to compare the receiver.
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    #[method]
    pub fn localized_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), localizedCompare: string] }
    }

    /// Compares the string with the specified string using the given options.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    /// * `mask` - The mask to use when comparing the receiver and `string`.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    #[method]
    pub fn compare_options<S>(&self, string: S, mask: NSStringCompareOptions) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), compare: string options: mask] }
    }

    /// Compares the string with the specified string using the given options.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    /// * `mask` - The mask to use when comparing the receiver and `string`.
    /// * `range` - The range of the receiver to compare.
    #[method]
    pub fn compare_options_range<S>(
        &self,
        string: S,
        mask: NSStringCompareOptions,
        range: NSRange,
    ) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), compare: string options: mask range: range] }
    }

    /// Compares the string using the specified options and returns the lexical ordering for the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    /// * `mask` - The mask to use when comparing the receiver and `string`.
    /// * `range` - The range to compare.
    /// * `locale` - The locale to use when comparing the receiver and `string`.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    #[method]
    pub fn compare_options_range_locale<S>(
        &self,
        string: S,
        mask: NSStringCompareOptions,
        range: NSRange,
        locale: NSLocale,
    ) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe {
            msg_send![self.m_self(), compare: string options: mask range: range locale: locale]
        }
    }

    /// Compares strings as sorted by the Finder.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// # Returns
    ///
    /// The result of the comparison.
    #[method]
    pub fn localized_standard_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), localizedStandardCompare: string] }
    }

    /// Returns a Boolean value that indicates whether a given string matches the beginning characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string begins with the prefix, otherwise `false`.
    #[method]
    pub fn has_prefix<S>(&self, prefix: S) -> bool
    where
        S: INSString,
    {
        unsafe { to_bool(msg_send![self.m_self(), hasPrefix: prefix]) }
    }

    /// Returns a Boolean value that indicates whether a given string matches the ending characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string ends with the suffix, otherwise `false`.
    #[method]
    pub fn has_suffix<S>(&self, suffix: S) -> bool
    where
        S: INSString,
    {
        unsafe { to_bool(msg_send![self.m_self(), hasSuffix: suffix]) }
    }

    /// Returns a Boolean value that indicates whether a given string is equal to the receiver using a literal Unicode-based comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string is equal to the receiver, otherwise `false`.
    #[method]
    pub fn is_equal_to_string<S>(&self, string: S) -> bool
    where
        S: INSString,
    {
        unsafe { to_bool(msg_send![self.m_self(), isEqualToString: string]) }
    }

    /* Combining Strings
     */

    /// Returns a new string made by appending a given string to the receiver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append to the receiver. This value must not be nil.
    #[method]
    pub fn string_by_appending_string<S>(&self, string: S) -> NSString
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), stringByAppendingString: string] }
    }

    /// Returns a new string formed from the receiver by either removing characters from the end, or by appending as many occurrences as necessary of a given pad string.
    ///
    /// # Arguments
    ///
    /// * `new_length` - The number of characters to be contained in the new string.
    /// * `pad` - The string to use for padding.
    /// * `starting_at` - The index in `pad_string` from which to start padding.
    ///
    /// # Returns
    ///
    /// A new string formed from the receiver by either removing characters from the end, or by appending as many occurrences of `pad_string` as necessary.
    #[method]
    pub fn string_by_padding_to_length_with_string_starting_at_index<S>(
        &self,
        new_length: UInt,
        pad_string: S,
        starting_at: UInt,
    ) -> NSString
    where
        S: INSString,
    {
        unsafe {
            NSString::from_id(
                msg_send![self.m_self(), stringByPaddingToLength: new_length withString: pad_string startingAtIndex: starting_at],
            )
        }
    }

    /* Changing Case
     */

    /// A lowercase representation of the string.
    #[property]
    pub fn lowercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), lowercaseString]) }
    }

    /// Returns a version of the string with all letters converted to lowercase,
    /// taking into account the current locale.
    #[property]
    pub fn localized_lowercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedLowercaseString]) }
    }

    /// Returns a version of the string with all letters converted to
    /// lowercase, taking into account the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use when converting the letters to lowercase.
    ///
    /// # Returns
    ///
    /// A new string with all letters converted to lowercase.
    #[method]
    pub fn lowercase_string_with_locale(&self, locale: NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), lowercaseStringWithLocale: locale]) }
    }

    /// An uppercase representation of the string.
    #[property]
    pub fn uppercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), uppercaseString]) }
    }

    /// Returns a version of the string with all letters converted to uppercase,
    /// taking into account the current locale.
    #[property]
    pub fn localized_uppercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedUppercaseString]) }
    }

    /// Returns a version of the string with all letters converted to uppercase,
    /// taking into account the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use when converting the letters to uppercase.
    ///
    /// # Returns
    ///
    /// A new string with all letters converted to uppercase.
    #[method]
    pub fn uppercase_string_with_locale(&self, locale: NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), uppercaseStringWithLocale: locale]) }
    }

    /// A capitalized representation of the string.
    #[property]
    pub fn capitalized_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), capitalizedString]) }
    }

    /// Returns a capitalized representation of the receiver using the current
    /// locale.
    #[property]
    pub fn localized_capitalized_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedCapitalizedString]) }
    }

    /// Returns a capitalized representation of the receiver using the
    /// specified locale.
    #[method]
    pub fn capitalized_string_with_locale(&self, locale: NSLocale) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                capitalizedStringWithLocale: locale
            ])
        }
    }

    /* Dividing Strings
     */

    /// Returns an array containing substrings from the receiver that have been divided by a given separator.
    #[method]
    pub fn components_separated_by_string<S>(&self, separator: S) -> NSArray<NSString>
    where
        S: INSString,
    {
        unsafe {
            NSArray::from_id(msg_send![
                self.m_self(),
                componentsSeparatedByString: separator
            ])
        }
    }

    /// Returns an array containing substrings from the receiver that have been divided by characters in a given set.
    #[method]
    pub fn components_separated_by_characters_in_set(
        &self,
        separator: &NSCharacterSet,
    ) -> NSArray<NSString> {
        unsafe {
            NSArray::from_id(msg_send![
                self.m_self(),
                componentsSeparatedByCharactersInSet: separator.m_self()
            ])
        }
    }

    /// Returns a new string made by removing from both ends of the receiver characters contained in a given character set.
    #[method]
    pub fn string_by_trimming_characters_in_set(&self, set: &NSCharacterSet) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                stringByTrimmingCharactersInSet: set.m_self()
            ])
        }
    }

    /// Returns a new string containing the characters of the receiver from the one at a given index to the end.
    #[method]
    pub fn substring_from_index(&self, from: UInt) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), substringFromIndex: from]) }
    }

    /// Returns a string object containing the characters of the receiver that lie within a given range.
    #[method]
    pub fn substring_with_range(&self, range: NSRange) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), substringWithRange: range]) }
    }

    /// Returns a new string containing the characters of the receiver up to, but not including, the one at a given index.
    #[method]
    pub fn substring_to_index(&self, to: UInt) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), substringToIndex: to]) }
    }

    /* Normalizing Strings
     */

    /// A string made by normalizing the string’s contents using the Unicode Normalization Form D.
    #[property]
    pub fn decomposed_string_with_canonical_mapping(&self) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                decomposedStringWithCanonicalMapping
            ])
        }
    }

    /// A string made by normalizing the receiver’s contents using the Unicode Normalization Form KD.
    #[property]
    pub fn decomposed_string_with_compatibility_mapping(&self) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                decomposedStringWithCompatibilityMapping
            ])
        }
    }

    /// A string made by normalizing the string’s contents using the Unicode Normalization Form C.
    #[property]
    pub fn precomposed_string_with_canonical_mapping(&self) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                precomposedStringWithCanonicalMapping
            ])
        }
    }

    /// A string made by normalizing the receiver’s contents using the Unicode Normalization Form KC.
    #[property]
    pub fn precomposed_string_with_compatibility_mapping(&self) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                precomposedStringWithCompatibilityMapping
            ])
        }
    }

    /* Folding Strings
     */

    /// Creates a string suitable for comparison by removing the specified character distinctions from a string.
    #[method]
    pub fn string_by_folding_with_options_locale(
        &mut self,
        options: NSStringCompareOptions,
        locale: &NSLocale,
    ) -> NSString {
        unsafe {
            NSString::from_id(
                msg_send![self.m_self(), stringByFoldingWithOptions: options locale: locale.m_self() ],
            )
        }
    }

    /* Transforming Strings
     */

    /// Returns a new string made by appending to the receiver a given string.
    ///
    /// # Arguments
    ///
    /// * `transform` - The `StringTransform` to apply.
    /// * `reverse` - If `true`, the transformation is applied in reverse.
    #[method]
    pub fn string_by_applying_transform_reverse(
        &mut self,
        transform: NSStringTransform,
        reverse: bool,
    ) -> Option<NSString> {
        unsafe {
            to_optional(
                msg_send![self.m_self(), stringByApplyingTransform: transform reverse: reverse],
            )
        }
    }

    /* Finding Characters and Substrings
     */

    /// Returns a boolean value indicating whether the string contains a given string by performing a case-sensitive, locale-unaware search.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if `string` is contained in the receiver, otherwise `false`.
    #[method]
    pub fn contains_string<S>(&self, other: S) -> bool
    where
        S: INSString,
    {
        unsafe { to_bool(msg_send![self.m_self(), containsString: other]) }
    }

    /* Working with Encodings
     */

    /// Returns a zero-terminated list of the encodings string objects support in the application’s environment.
    #[property]
    pub fn available_string_encodings() -> *const Encoding {
        unsafe { msg_send![Self::m_class(), availableStringEncodings] }
    }

    /// Returns the C-string encoding assumed for any method accepting a C string as an argument.
    #[property]
    pub fn default_cstring_encoding() -> Encoding {
        unsafe { msg_send![Self::m_class(), defaultCStringEncoding] }
    }

    /// Returns a Boolean value that indicates whether the receiver can be converted to a given encoding without loss of information.
    #[method]
    pub fn can_be_converted_to_encoding(&self, encoding: Encoding) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), canBeConvertedToEncoding: encoding]) }
    }

    /// Returns an NSData object containing a representation of the receiver encoded using a given encoding.
    #[method]
    pub fn data_using_encoding(&self, encoding: Encoding) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), dataUsingEncoding: encoding]) }
    }
}

impl PNSObject for NSString {
    fn m_class<'a>() -> &'a Class {
        class!(NSString)
    }

    fn m_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

unsafe impl objective_c_runtime::Encode for NSString {
    fn encode() -> objc::Encoding {
        unsafe { objective_c_runtime::Encoding::from_str("@") }
    }
}

impl Default for NSString {
    fn default() -> Self {
        Self::string()
    }
}

impl fmt::Debug for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.p_description().as_str().unwrap())
    }
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.p_description().as_str().unwrap())
    }
}

impl Clone for NSString {
    fn clone(&self) -> Self {
        unsafe { msg_send![&*self.ptr, retain] }
    }
}

impl ToId for NSString {
    fn to_id(self) -> id {
        unsafe { &mut *self.m_self() }
    }
}

impl FromId for NSString {
    unsafe fn from_id(id: id) -> Self {
        NSString {
            ptr: Id::from_ptr(id),
            marker: PhantomData,
        }
    }
}

impl From<NSMutableString> for NSString {
    fn from(string: NSMutableString) -> Self {
        unsafe {
            let ptr: id = msg_send![NSString::m_class(), alloc];
            let ptr = msg_send![ptr, initWithString: string];
            NSString::from_id(ptr)
        }
    }
}

impl From<String> for NSString {
    /// Creates a new `NSString` from a `String`.
    fn from(s: String) -> Self {
        let c_string = CString::new(s.clone()).unwrap();
        NSString {
            ptr: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(
                    msg_send![nsstring, initWithBytes:c_string.into_raw() as *mut Object
                        length:s.len()
                        encoding:UTF8_ENCODING
                    ],
                )
            },

            marker: PhantomData,
        }
    }
}

impl From<&str> for NSString {
    /// Creates a new `NSString` from a `&str`.
    fn from(s: &str) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: CString::new(s).unwrap().into_raw() as *mut Object
                    length:s.len()
                    encoding:UTF8_ENCODING
                ],
            )
        };

        NSString {
            ptr: objc,
            marker: PhantomData,
        }
    }
}

impl From<&&str> for NSString {
    /// Creates a new `NSString` from a `&str`.
    fn from(s: &&str) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: CString::new(*s).unwrap().into_raw() as *mut Object
                    length:s.len()
                    encoding:UTF8_ENCODING
                ],
            )
        };

        NSString {
            ptr: objc,
            marker: PhantomData,
        }
    }
}

impl From<char> for NSString {
    /// Creates a new `NSString` from a `char`.
    fn from(c: char) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: c.encode_utf8(&mut [0; 4]).as_ptr()
                    length:1
                    encoding:UTF8_ENCODING
                ],
            )
        };

        NSString {
            ptr: objc,
            marker: PhantomData,
        }
    }
}

impl From<NSString> for &str {
    /// Converts a `NSString` to a `&str`.
    fn from(string: NSString) -> Self {
        unsafe {
            let ptr: *const c_char = msg_send![string.ptr, UTF8String];
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl From<(&str, Encoding)> for NSString {
    /// Creates a new `NSString` from a `&str` and an encoding.
    fn from((s, encoding): (&str, Encoding)) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr()
                length:s.len()
                encoding:encoding
            ])
        };

        NSString {
            ptr: objc,
            marker: PhantomData,
        }
    }
}

impl PartialEq for NSString {
    /// Checks if two `NSString`s are equal.
    fn eq(&self, other: &Self) -> bool {
        self.localized_compare(other.clone()) == NSComparisonResult::OrderedSame
    }
}

impl Eq for NSString {}

impl PartialEq<&str> for NSString {
    /// Checks if a `NSString` is equal to a `&str`.
    fn eq(&self, other: &&str) -> bool {
        self.as_str().unwrap() == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::foundation::{
        string::Encoding, string_transform::LatinToKatakana, NSComparisonResult,
    };

    use super::*;

    #[test]
    fn test_from_str() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_bytes() {
        let s = NSString::from("Hello, World!");
        let other = s.bytes();
        assert_eq!(s.bytes(), other);
    }

    #[test]
    fn test_bytes_len() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.length_of_bytes_using_encoding(Encoding::UTF8), 13);
    }

    #[test]
    fn test_as_str() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_to_string() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.to_string(), "Hello, World!".to_string());
    }

    #[test]
    fn test_length() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.length(), 13);
    }

    #[test]
    fn test_contains() {
        let s = NSString::from("Hello, World!");
        assert!(s.contains_string(NSString::from("Hello")));
        assert!(!s.contains_string(NSString::from("Goodbye")));
    }

    #[test]
    fn test_character_at() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.character_at_index(0), 'H');
        assert_eq!(s.character_at_index(1), 'e');
        assert_eq!(s.character_at_index(2), 'l');
        assert_eq!(s.character_at_index(3), 'l');
        assert_eq!(s.character_at_index(4), 'o');
        assert_eq!(s.character_at_index(5), ',');
        assert_eq!(s.character_at_index(6), ' ');
        assert_eq!(s.character_at_index(7), 'W');
        assert_eq!(s.character_at_index(8), 'o');
        assert_eq!(s.character_at_index(9), 'r');
        assert_eq!(s.character_at_index(10), 'l');
        assert_eq!(s.character_at_index(11), 'd');
        assert_eq!(s.character_at_index(12), '!');
    }

    #[test]
    fn test_has_prefix() {
        let s = NSString::from("Hello, World!");
        assert!(s.has_prefix(NSString::from("Hello")));
        assert!(!s.has_prefix(NSString::from("Goodbye")));
    }

    #[test]
    fn test_has_suffix() {
        let s = NSString::from("Hello, World!");
        assert!(s.has_suffix(NSString::from("World!")));
        assert!(!s.has_suffix(NSString::from("Goodbye")));
    }

    #[test]
    fn test_is_equal_to() {
        let s = NSString::from("Hello, World!");
        assert!(s.is_equal_to_string(NSString::from("Hello, World!")));
        assert!(!s.is_equal_to_string(NSString::from("Goodbye, World!")));
    }

    #[test]
    fn test_length_of_bytes() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.length_of_bytes_using_encoding(Encoding::UTF8), 13);
    }

    #[test]
    fn test_maximum_length_of_bytes() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.maximum_length_of_bytes_using_encoding(Encoding::UTF8), 39);
    }

    #[test]
    fn test_case_insensitive_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.case_insensitive_compare(NSString::from("hello, world!")),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.case_insensitive_compare(NSString::from("goodbye, world!")),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.compare(NSString::from("Hello, World!")),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.compare(NSString::from("Goodbye, World!")),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_standard_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.localized_standard_compare(NSString::from("Hello, World!")),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localized_standard_compare(NSString::from("Goodbye, World!")),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_applying_transform() {
        let mut s = NSString::from("Katakana!");
        let transform = unsafe { LatinToKatakana };
        assert_eq!(
            s.string_by_applying_transform_reverse(transform, false)
                .unwrap(),
            "カタカナ!"
        );
    }

    #[test]
    fn test_im_case_insensitive_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.case_insensitive_compare(NSString::from("hello, world!")),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.case_insensitive_compare(NSString::from("goodbye, world!")),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_string_by_padding_to_length_with_string_starting_at_index() {
        let s = NSString::from("Hello, World!");

        assert_eq!(
            s.string_by_padding_to_length_with_string_starting_at_index(20, NSString::from("."), 0),
            "Hello, World!......."
        );
    }
}
