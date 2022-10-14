//! # Compression
//!
//! Leverage common compression algorithms for lossless data compression.
//!
//! ## Overview
//!
//! The Compression framework enables your app to provide lossless compression
//! when saving or sharing files and data. Compression is a process in which
//! you compress (encode) and decompress (decode) data. For example, a text
//! editor may save its files in a compressed format, and automatically
//! decompress the saved file when opened by the user.
#![allow(non_camel_case_types)]

use std::os::raw::c_int;

use libc::c_void;

use crate::kernel::{size_t, uint8_t};

/// A set of values used to represent the status of stream compression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum compression_status {
    /// Indicates an error with stream compression.
    ERROR = -1,
    /// Indicates the stream has consumed all data in the source buffer, or used all space in the destination buffer.
    OK = 0,
    /// Indicates the stream has read all input from the source, and written all output to the destination.
    END = 1,
}

/// A set of values used to represent a stream compression operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum compression_stream_flags {
    /// Indicates that the operation will add no further input blocks to the stream.
    COMPRESSION_STREAM_FINALIZE = 0x0001,
}

///A set of values used to represent a stream compression operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum compression_stream_operation {
    /// A constant indicating a compression operation.
    ENCODE = 0,
    /// A constant indicating a decompression operation.
    DECODE = 1,
}

/// A structure for values that represent compression algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum compression_algorithm {
    /// The LZFSE compression algorithm, that is recommended for use on Apple platforms.
    LZFSE = 0x801,
    /// The LZ4 compression algorithm, that is recommended for fast compression.
    LZ4 = 0x100,
    /// The LZ4 compression algorithm, without frame headers.
    LZ4_RAW = 0x101,
    /// The LZMA compression algorithm, that is recommended for high-compression ratio.
    LZMA = 0x306,
    /// The zlib compression algorithm, that is recommended for cross-platform compression.
    ZLIB = 0x205,
    /// The Brotli compression algorithm, which is recommended for text compression.
    BROTLI = 0xB02,
    /// The LZBITMAP compression algorithm, which is designed to exploit the vector instruction set of current CPUs.
    LZBITMAP = 0x702,
}

/// A structure representing a compression stream.
#[derive(Debug)]
#[repr(C)]
pub struct compression_stream {
    /// A pointer to the first byte of the destination buffer.
    pub dst_ptr: *mut uint8_t,
    /// The size, in bytes, of the destination buffer.
    pub dst_size: size_t,
    /// A pointer to the first byte of the source buffer.
    pub src_ptr: *const uint8_t,
    /// The size, in bytes, of the source buffer.
    pub src_size: size_t,
    /// The stream state object of the compression stream.
    pub state: *mut c_void,
}

extern "C" {
    /// Initializes a compression stream for either compression or decompression.
    ///
    /// # Arguments
    ///
    /// * `stream` - Pointer to an allocated [`compression_stream`] structure.
    /// * `operation` - A constant of type [`compression_stream_operation`] used to indicate the stream operation.
    /// * `algorithm` - A constant of type [`compression_algorithm`] to select the algorithm: [`compression_algorithm::LZ4`], [`compression_algorithm::ZLIB`], [`compression_algorithm::LZMA`], or [`compression_algorithm::LZFSE`].
    ///
    /// # Returns
    /// A value of type `compression_status`, interpreted as follows:
    ///
    /// * [`compression_status::OK`] - The stream was successfully initialized.
    /// * [`compression_status::ERROR`] - This means an error occured.
    pub fn compression_stream_init(
        stream: *mut compression_stream,
        operation: compression_stream_flags,
        algorithm: compression_algorithm,
    ) -> compression_status;

    /// Performs compression or decompression using an initialized compression stream structure.
    ///
    /// # Arguments
    ///
    /// * `stream` - A pointer to an allocated and fully initialized [`compression_stream`] structure.
    /// * `flags` - A constant of type [`compression_stream_flags`]; this should be [`compression_stream_flags::COMPRESSION_STREAM_FINALIZE`] if there is no further input data, or 0 otherwise.
    ///
    /// # Returns
    ///
    /// A value of type `compression_status`, interpreted as follows:
    ///
    /// * [`compression_status::OK`] - Processing was successful, but the stream may produce more output. Call the function again with updated parameters.
    /// * [`compression_status::ERROR`] - Processing was successful, and the stream will produce no more output (this only occurs if flags is set to [`compression_stream_flags::COMPRESSION_STREAM_FINALIZE`]).
    /// * [`compression_status::END`] - An error occurred.
    pub fn compression_stream_process(
        stream: *mut compression_stream,
        status: c_int,
    ) -> compression_status;

    /// Frees any memory allocated by stream initialization function.
    ///
    /// # Arguments
    ///
    /// * `stream` - A pointer to an allocated and initialized `compression_stream` structure.
    ///
    /// # Returns
    ///
    /// A value of type `compression_status`, interpreted as follows:
    ///
    /// * [`compression_status::OK`] - The function successfully destroyed the stream.
    /// * [`compression_status::ERROR`] - An error occurred.
    pub fn compression_stream_destroy(stream: *mut compression_stream) -> compression_status;

    /// Returns the required compression scratch buffer size for the selected algorithm.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - Set to the desired algorithm: [`compression_algorithm::LZ4`], [`compression_algorithm::ZLIB`], [`compression_algorithm::LZMA`], or [`compression_algorithm::LZFSE`].
    ///
    /// # Returns
    ///
    /// Size in bytes.
    pub fn compression_encode_scratch_buffer_size(algorithm: compression_algorithm) -> size_t;

    /// Compresses the contents of a source buffer into a destination buffer.
    ///
    /// # Arguments
    ///
    /// * `dst_buffer` - Pointer to the buffer that receives the compressed data.
    /// * `dst_size` - Size of the destination buffer in bytes.
    /// * `src` - Pointer to a buffer containing all of the source data.
    /// * `src_size` - Size of the data in the source buffer in bytes.
    /// * `scratch_buffer` - If scratch_buffer is not [`crate::objective_c_runtime::nil`], this parameter is a pointer to a buffer that the function uses for scratch purposes. The size of this buffer must be at least the size returned by a previous call to [`compression_encode_scratch_buffer_size`].If `scratch_buffer` is [`crate::objective_c_runtime::nil`], the function creates and manages its own scratch space, but with a possible performance hit.
    /// * `algorithm` - Set to the desired algorithm: [`compression_algorithm::LZ4`], [`compression_algorithm::ZLIB`], [`compression_algorithm::LZMA`], or [`compression_algorithm::LZFSE`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_macios::compression::{compression_algorithm, compression_encode_buffer};
    ///
    /// let source_string = r#"
    ///     Lorem ipsum dolor sit amet consectetur adipiscing elit mi
    ///     nibh ornare proin blandit diam ridiculus, faucibus mus
    ///     dui eu vehicula nam donec dictumst sed vivamus bibendum
    ///     aliquet efficitur. Felis imperdiet sodales dictum morbi
    ///     vivamus augue dis duis aliquet velit ullamcorper porttitor,
    ///     lobortis dapibus hac purus aliquam natoque iaculis blandit
    ///     montes nunc pretium."#;
    ///
    /// let source_buffer = source_string.as_bytes();
    ///
    /// let mut dest_buffer = vec![0; source_string.len()];
    ///
    /// let algorithm = compression_algorithm::LZFSE;
    ///
    /// let compressed_size = unsafe {
    ///     compression_encode_buffer(
    ///         dest_buffer.as_mut_ptr(),
    ///         source_string.len() as isize,
    ///         source_buffer.as_ptr(),
    ///         source_string.len() as isize,
    ///         std::ptr::null_mut(),
    ///         algorithm,
    ///     )
    /// };
    ///
    /// assert!(compressed_size != 0, "Encoding failed.");
    /// ```
    ///
    /// # Returns
    ///
    /// The number of bytes written to the destination buffer after compressing
    ///  the input. If the funtion can't compress the entire input to fit into
    /// the provided destination buffer, or an error occurs, 0 is returned.
    pub fn compression_encode_buffer(
        dst_buffer: *mut uint8_t,
        dst_size: size_t,
        src: *const uint8_t,
        src_size: size_t,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> size_t;

    /// Returns the required decompression scratch buffer size for the selected algorithm.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - Set to the desired algorithm: [`compression_algorithm::LZ4`], [`compression_algorithm::ZLIB`], [`compression_algorithm::LZMA`], or [`compression_algorithm::LZFSE`].
    ///
    /// # Returns
    ///
    /// Size in bytes.
    pub fn compression_decode_scratch_buffer_size(algorithm: compression_algorithm) -> size_t;

    /// Decompresses the contents of a source buffer into a destination buffer.
    ///
    /// # Arguments
    ///
    /// * `dst_buffer` - Pointer to the buffer that receives the decompressed data.
    /// * `dst_size` - Size of the destination buffer in bytes.
    /// * `src` - Pointer to a buffer containing all of the source data.
    /// * `src_size` - Size of the data in the source buffer in bytes.
    /// * `scratch_buffer` - If scratch_buffer is not nil, this parameter is a pointer to a buffer that the function uses for scratch purposes. The size of this buffer must be at least the size returned by a previous call to `compression_decode_scratch_buffer_size`.If `scratch_buffer` is `nil`, the function creates and manages its own scratch space, but with a possible performance hit.
    /// * `algorithm` - Set to the desired algorithm: [`compression_algorithm::LZ4`], [`compression_algorithm::ZLIB`], [`compression_algorithm::LZMA`], or [`compression_algorithm::LZFSE`].
    ///
    /// # Returns
    ///
    /// The number of bytes written to the destination buffer after decompressing the input. If there is not enough space in the destination buffer to hold the entire decompressed output, the function writes the first dst_size bytes to the buffer and returns dst_size. Note that this behavior differs from that of `compression_encode_buffer`.
    pub fn compression_decode_buffer(
        dst_buffer: *mut uint8_t,
        dst_size: size_t,
        src: *const uint8_t,
        src_size: size_t,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> size_t;
}
