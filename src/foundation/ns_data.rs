use block::IntoConcreteBlock;
use libc::c_void;
use objc::{msg_send, sel, sel_impl};

use crate::{
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{NSError, NSRange, NSString, UInt, NSURL};

/// Options for methods used to read data objects.
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum NSDataReadingOptions {
    MappedIfSafe = 1 << 0,
    /// A hint indicating the file should not be stored in the file-system caches.
    Uncached = 1 << 1,
    /// Hint to map the file in if possible.
    MappedAlways = 1 << 3,
}

/// Options for methods used to write data objects.
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum NSDataWritingOptions {
    /// An option to write data to an auxiliary file first and then replace the original file with the auxiliary file when the write completes.
    Atomic = 1,
    /// An option that attempts to write data to a file and fails with an error if the destination file already exists.
    WithoutOverwriting = 2,
    /// An option to not encrypt the file when writing it out.
    FileProtectionNone = 0x10000000,
    /// An option to make the file accessible only while the device is unlocked.
    FileProtectionComplete = 0x20000000,
    /// An option to allow the file to be accessible after a user first unlocks the device.
    FileProtectionMask = 0xf0000000,
    /// An option to allow the file to be accessible while the device is unlocked or the file is already open.
    FileProtectionCompleteUnlessOpen = 0x30000000,
    /// An option the system uses when determining the file protection options that the system assigns to the data.
    FileProtectionCompleteUntilFirstUserAuthentication = 0x40000000,
}

/// Options for method used to search data objects.
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum NSDataSearchOptions {
    /// Search from the end of the data object.
    SearchBackwards = 1,
    /// Search is limited to start (or end, if searching backwards) of the data object.
    SearchAnchored = 2,
}

/// Options to modify the decoding algorithm used to decode Base64 encoded data.
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum NSDataBase64DecodingOptions {
    None = 0,
    /// Modify the decoding algorithm so that it ignores unknown non-Base-64 bytes, including line ending characters.
    IgnoreUnknownCharacters = 1,
}

/// Options for methods used to Base64 encode data.
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum NSDataBase64EncodingOptions {
    None = 0,
    /// Set the maximum line length to 64 characters, after which a line ending is inserted.
    SixtyFourCharacterLineLength = 1,
    /// Set the maximum line length to 76 characters, after which a line ending is inserted.
    SeventySixCharacterLineLength = 1 << 1,
    /// When a maximum line length is set, specify that the line ending to insert should include a carriage return.
    EndLineWithCarriageReturn = 1 << 4,
    /// When a maximum line length is set, specify that the line ending to insert should include a line feed.
    EndLineWithLineFeed = 1 << 5,
}

/// An algorithm that indicates how to compress or decompress data.
#[derive(Debug, Clone, Copy)]
#[repr(i64)]
pub enum NSDataCompressionAlgorithm {
    /// The algorithm offers faster speed and generally achieves better compression than [`NSDataCompressionAlgorithm::Zlib`]. However, it is slower than [`NSDataCompressionAlgorithm::LZ4`] and doesn’t compress as well as [`NSDataCompressionAlgorithm::LZMA`].
    LZFSE = 0,
    /// The LZ4 compression algorithm, recommended for fast compression.
    LZ4,
    /// The LZMA compression algorithm, recommended for high-compression ratio.
    LZMA,
    /// The zlib compression algorithm, recommended for cross-platform compression.
    Zlib,
}

object! {
    /// A static byte buffer in memory.
    unsafe pub struct NSData;
}

#[interface_impl(NSObject)]
impl NSData {
    /* Creating Data
     */

    /// Creates an empty data object.
    #[method]
    pub fn data() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), data]) }
    }

    /// Creates a data object containing a given number of bytes copied from a given buffer.
    #[method]
    pub fn data_with_bytes_length(bytes: *const c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dataWithBytes: bytes length: length]) }
    }

    /// Creates a data object that holds a given number of bytes from a given buffer.
    #[method]
    pub fn data_with_bytes_no_copy_length(bytes: *mut c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![Self::m_class(), dataWithBytesNoCopy: bytes
                                                                  length: length])
        }
    }

    /// Creates a data object that holds a given number of bytes from a given buffer.
    #[method]
    pub fn data_with_bytes_no_copy_length_free_when_done(
        bytes: *mut c_void,
        length: UInt,
        b: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), dataWithBytesNoCopy:bytes length:length freeWhenDone:b],
            )
        }
    }

    /// Creates a data object containing the contents of another data object.
    #[method]
    pub fn data_with_data(data: NSData) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dataWithData: data]) }
    }

    /// Initializes a data object filled with a given number of bytes copied from a given buffer.
    #[method]
    pub fn init_with_bytes_length(&mut self, bytes: *const c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithBytes: bytes length: length]) }
    }

    /// Initializes a data object filled with a given number of bytes of data from a given buffer.
    #[method]
    pub fn init_with_bytes_no_copy_length(&mut self, bytes: *mut c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![self.m_self(), initWithBytesNoCopy: bytes length: length])
        }
    }

    /// Initializes a data object filled with a given number of bytes of data from a given buffer, with a custom deallocator block.
    #[method]
    pub fn init_with_bytes_no_copy_length_deallocator<F>(
        &mut self,
        bytes: *mut c_void,
        length: UInt,
        deallocator: F,
    ) -> Self
    where
        Self: Sized + FromId,
        F: IntoConcreteBlock<(*mut c_void, UInt), Ret = ()> + 'static,
    {
        let deallocator = deallocator.into_concrete_block();
        let deallocator = deallocator.copy();

        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithBytesNoCopy: bytes length: length deallocator: deallocator],
            )
        }
    }

    /// Initializes a newly allocated data object by adding the given number of bytes from the given buffer.
    #[method]
    pub fn init_with_bytes_no_copy_length_free_when_done(
        &mut self,
        bytes: *mut c_void,
        length: UInt,
        b: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithBytesNoCopy: bytes length: length freeWhenDone:b],
            )
        }
    }

    /// Initializes a data object with the contents of another data object.
    #[method]
    pub fn init_with_data(&mut self, data: &NSData) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithData: data.m_self()]) }
    }

    /*Reading Data from a File
     */

    /// Creates a data object by reading every byte from the file at a given path.
    #[method]
    pub fn data_with_contents_of_file(path: &NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dataWithContentsOfFile: path.m_self()]) }
    }

    /// Creates a data object by reading every byte from the file at a given path.
    #[method]
    pub fn data_with_contents_of_file_options(
        path: &NSString,
        read_options_mask: NSDataReadingOptions,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        unsafe {
            let ptr = Self::from_id(
                msg_send![Self::m_class(), dataWithContentsOfFile: path.m_self() options: read_options_mask error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /// Creates a data object containing the data from the location specified by a given URL.
    #[method]
    pub fn data_with_contents_of_url(url: &NSURL) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dataWithContentsOfURL: url.m_self()]) }
    }

    /// Creates a data object containing the data from the location specified by a given URL.
    #[method]
    pub fn data_with_contents_of_url_options(
        url: &NSURL,
        read_options_mask: NSDataReadingOptions,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();
        unsafe {
            let ptr = Self::from_id(
                msg_send![Self::m_class(), dataWithContentsOfURL: url.m_self() options: read_options_mask error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /// Initializes a data object with the content of the file at a given path.
    #[method]
    pub fn init_with_contents_of_file(&mut self, path: &NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithContentsOfFile: path.m_self()]) }
    }

    /// Initializes a data object with the content of the file at a given path.
    #[method]
    pub fn init_with_contents_of_file_options(
        &mut self,
        path: &NSString,
        read_options_mask: NSDataReadingOptions,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        unsafe {
            let ptr = Self::from_id(
                msg_send![self.m_self(), initWithContentsOfFile: path.m_self() options: read_options_mask error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /// Initializes a data object with the data from the location specified by a given URL.
    #[method]
    pub fn init_with_contents_of_url(&mut self, url: &NSURL) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithContentsOfURL: url.m_self()]) }
    }

    /// Initializes a data object with the data from the location specified by a given URL.
    #[method]
    pub fn init_with_contents_of_url_options(
        &mut self,
        url: &NSURL,
        read_options_mask: NSDataReadingOptions,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        unsafe {
            let ptr = Self::from_id(
                msg_send![self.m_self(), initWithContentsOfURL: url.m_self() options: read_options_mask error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /* Writing Data to a File
     */

    /// Writes the data object's bytes to the file specified by a given path.
    #[method]
    pub fn write_to_file_atomically(&self, path: &NSString, use_auxiliary_file: bool) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), writeToFile: path.m_self() atomically: use_auxiliary_file],
            )
        }
    }

    /// Writes the data object’s bytes to the file specified by a given path.
    #[method]
    pub fn write_to_file_options(
        &self,
        path: &NSString,
        write_options: NSDataWritingOptions,
    ) -> Result<bool, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr = to_bool(
                msg_send![self.m_self(), writeToFile: path.m_self() options: write_options error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /// Writes the data object's bytes to the location specified by a given URL.
    #[method]
    pub fn write_to_url_atomically(&self, url: &NSURL, atomically: bool) -> bool {
        unsafe {
            to_bool(msg_send![self.m_self(), writeToURL: url.m_self() atomically: atomically])
        }
    }

    /// Writes the data object's bytes to the location specified by a given URL.
    #[method]
    pub fn write_to_url_options(
        &self,
        url: &NSURL,
        write_options: NSDataWritingOptions,
    ) -> Result<bool, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr = to_bool(
                msg_send![self.m_self(), writeToURL: url.m_self() options: write_options error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /* Encoding and Decoding Base64 Representations
     */

    /// Initializes a data object with the given Base64 encoded data.
    #[method]
    pub fn init_with_base64_encoded_data_options(
        &mut self,
        base64_data: &NSData,
        options: NSDataBase64DecodingOptions,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithBase64EncodedData: base64_data.m_self() options: options],
            )
        }
    }

    /// Initializes a data object with the given Base64 encoded string.
    #[method]
    pub fn init_with_base64_encoded_string_options(
        &mut self,
        base64_string: &NSString,
        options: NSDataBase64DecodingOptions,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithBase64EncodedString: base64_string.m_self() options: options],
            )
        }
    }

    /// Creates a Base64, UTF-8 encoded data object from the string using the given options.
    #[method]
    pub fn base64_encoded_data_with_options(&self, options: NSDataBase64EncodingOptions) -> NSData {
        unsafe {
            NSData::from_id(msg_send![
                self.m_self(),
                base64EncodedDataWithOptions: options
            ])
        }
    }

    /// Creates a Base64 encoded string from the string using the given options.
    #[method]
    pub fn base64_encoded_string_with_options(
        &self,
        options: NSDataBase64EncodingOptions,
    ) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                base64EncodedStringWithOptions: options
            ])
        }
    }

    /* Accessing Underlying Bytes

    */

    /// A pointer to the data object's contents.
    #[property]
    pub fn bytes(&self) -> *const c_void {
        unsafe { msg_send![self.m_self(), bytes] }
    }

    /// Enumerates each range of bytes in the data object using a block.
    #[method]
    pub fn enumerate_byte_ranges_using_block<F>(&self, block: F)
    where
        F: IntoConcreteBlock<(*const c_void, NSRange, *mut bool), Ret = ()> + 'static,
    {
        unsafe {
            let block = block.into_concrete_block();
            let block = block.copy();

            msg_send![self.m_self(), enumerateByteRangesUsingBlock: block]
        }
    }

    /// Copies a number of bytes from the start of the data object into a given buffer.
    #[method]
    pub fn get_bytes_length(&self, buffer: *mut c_void, length: UInt) {
        unsafe { msg_send![self.m_self(), getBytes: buffer length: length] }
    }

    /// Copies a range of bytes from the data object into a given buffer.
    #[method]
    pub fn get_bytes_range(&self, buffer: *mut c_void, range: NSRange) {
        unsafe { msg_send![self.m_self(), getBytes: buffer range: range] }
    }

    /* Finding Data
     */

    /// Returns a new data object containing the data object's bytes that fall within the limits specified by a given range.
    #[method]
    pub fn subdata_with_range(&self, range: NSRange) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), subdataWithRange: range]) }
    }

    /// Finds and returns the range of the first occurrence of the given data, within the given range, subject to given options.
    #[method]
    pub fn range_of_data_options_range(
        &self,
        data_to_find: &NSData,
        mask: NSDataSearchOptions,
        search_range: NSRange,
    ) -> NSRange {
        unsafe {
            msg_send![self.m_self(), rangeOfData: data_to_find.m_self() options: mask range:search_range]
        }
    }

    /// Returns a Boolean value indicating whether this data object is the same as another.
    #[method]
    pub fn is_equal_to_data(&self, other: &NSData) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isEqualToData: other.m_self()]) }
    }

    /// The number of bytes contained by the data object.
    #[method]
    pub fn length(&self) -> UInt {
        unsafe { msg_send![self.m_self(), length] }
    }

    /* Compressing and Decompressing Data
     */

    /// Returns a new data object by compressing the data object’s bytes.
    #[method]
    pub fn compressed_data_using_algorithm(
        &self,
        algorithm: NSDataCompressionAlgorithm,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        unsafe {
            let ptr = Self::from_id(
                msg_send![self.m_self(), compressedDataUsingAlgorithm: algorithm error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /// Returns a new data object by decompressing data object’s bytes.
    #[method]
    pub fn decompressed_data_using_algorithm(
        &self,
        algorithm: NSDataCompressionAlgorithm,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        unsafe {
            let ptr = Self::from_id(
                msg_send![self.m_self(), decompressedDataUsingAlgorithm: algorithm error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }
}
