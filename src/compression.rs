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

use std::os::raw::c_int;

use libc::{c_void, ssize_t};

/// A set of values used to represent the status of stream compression.
#[allow(non_camel_case_types)]
#[derive(Debug)]
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
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u64)]
pub enum compression_stream_operation {
    /// Indicates that the operation will add no further input blocks to the stream.
    COMPRESSION_STREAM_FINALIZE = 0x0001,
}

/// A set of values used to represent stream compression flags.
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u64)]
pub enum compression_stream_flags {
    /// A constant indicating that the stream should not flush the output buffer when it reaches the end of the input buffer.
    NO_FLUSH = 0,
    /// A constant indicating that the stream should flush the output buffer when it reaches the end of the input buffer.
    FLUSH = 1,
}

/// A structure for values that represent compression algorithms.
#[allow(non_camel_case_types)]
#[derive(Debug)]
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
    ///
    COMPRESSION_BROTLI = 0xB02,
}

/// A structure representing a compression stream.
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct compression_stream {
    /// A pointer to the first byte of the destination buffer.
    pub dst: *mut u8,
    /// The size, in bytes, of the destination buffer.
    pub dst_size: ssize_t,
    /// A pointer to the first byte of the source buffer.
    pub src: *const u8,
    /// The size, in bytes, of the source buffer.
    pub src_size: ssize_t,
    /// The stream state object of the compression stream.
    pub state: *mut c_void,
}

extern "C" {
    /// Initializes a compression stream for either compression or decompression.
    ///
    /// # Arguments
    ///
    /// * `stream` - Pointer to an allocated `compression_stream` structure.
    /// * `operation` - A constant of type compression_stream_operation used to indicate the stream operation.
    /// * `algorithm` - A constant of type `compression_algorithm` to select the algorithm: `COMPRESSION_LZ4`, `COMPRESSION_ZLIB`, `COMPRESSION_LZMA`, or `COMPRESSION_LZFSE`.
    ///
    /// # Returns
    /// A value of type `compression_status`, interpreted as follows:
    ///
    /// * `COMPRESSION_STATUS_OK` - The stream was successfully initialized.
    /// * `COMPRESSION_STATUS_ERROR` - This means an error occured.
    pub fn compression_stream_init(
        stream: *mut compression_stream,
        operation: compression_stream_operation,
        algorithm: compression_algorithm,
    ) -> compression_status;

    /// Performs compression or decompression using an initialized compression stream structure.
    ///
    /// # Arguments
    ///
    /// * `stream` - A pointer to an allocated and fully initialized `compression_stream` structure.
    /// * `flags` - A constant of type `compression_stream_flags`; this should be `COMPRESSION_STREAM_FINALIZE` if there is no further input data, or 0 otherwise.
    ///
    /// # Returns
    ///
    /// A value of type `compression_status`, interpreted as follows:
    ///
    /// * `COMPRESSION_STATUS_OK` - Processing was successful, but the stream may produce more output. Call the function again with updated parameters.
    /// * `COMPRESSION_STATUS_ERROR` - Processing was successful, and the stream will produce no more output (this only occurs if flags is set to `COMPRESSION_STREAM_FINALIZE`).
    /// * `COMPRESSION_STATUS_END` - An error occurred.
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
    /// * `COMPRESSION_STATUS_OK` - The function successfully destroyed the stream.
    /// * `COMPRESSION_STATUS_ERROR` - An error occurred.
    pub fn compression_stream_destroy(stream: *mut compression_stream) -> compression_status;

    /// Returns the required compression scratch buffer size for the selected algorithm.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - Set to the desired algorithm: `COMPRESSION_LZ4`, `COMPRESSION_ZLIB`, `COMPRESSION_LZMA`, or `COMPRESSION_LZFSE`.
    ///
    /// # Returns
    ///
    /// Size in bytes.
    pub fn compression_encode_scratch_buffer_size(algorithm: compression_algorithm) -> ssize_t;

    /// Compresses the contents of a source buffer into a destination buffer.
    ///
    /// # Arguments
    ///
    /// * `dst_buffer` - Pointer to the buffer that receives the compressed data.
    /// * `dst_size` - Size of the destination buffer in bytes.
    /// * `src` - Pointer to a buffer containing all of the source data.
    /// * `src_size` - Size of the data in the source buffer in bytes.
    /// * `scratch_buffer` - If scratch_buffer is not nil, this parameter is a pointer to a buffer that the function uses for scratch purposes. The size of this buffer must be at least the size returned by a previous call to `compression_encode_scratch_buffer_size`.If `scratch_buffer` is `nil`, the function creates and manages its own scratch space, but with a possible performance hit.
    /// * `algorithm` - Set to the desired algorithm: `COMPRESSION_LZ4`, `COMPRESSION_ZLIB`, `COMPRESSION_LZMA`, or `COMPRESSION_LZFSE`.
    ///
    /// # Returns
    ///
    /// The number of bytes written to the destination buffer after compressing
    ///  the input. If the funtion can't compress the entire input to fit into
    /// the provided destination buffer, or an error occurs, 0 is returned.
    pub fn compression_encode_buffer(
        dst_buffer: *mut u8,
        dst_size: ssize_t,
        src: *const u8,
        src_size: ssize_t,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> ssize_t;

    /// Returns the required decompression scratch buffer size for the selected algorithm.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - Set to the desired algorithm: `COMPRESSION_LZ4`, `COMPRESSION_ZLIB`, `COMPRESSION_LZMA`, or `COMPRESSION_LZFSE`.
    ///
    /// # Returns
    ///
    /// Size in bytes.
    pub fn compression_decode_scratch_buffer_size(algorithm: compression_algorithm) -> ssize_t;

    /// Decompresses the contents of a source buffer into a destination buffer.
    ///
    /// # Arguments
    ///
    /// * `dst_buffer` - Pointer to the buffer that receives the decompressed data.
    /// * `dst_size` - Size of the destination buffer in bytes.
    /// * `src` - Pointer to a buffer containing all of the source data.
    /// * `src_size` - Size of the data in the source buffer in bytes.
    /// * `scratch_buffer` - If scratch_buffer is not nil, this parameter is a pointer to a buffer that the function uses for scratch purposes. The size of this buffer must be at least the size returned by a previous call to `compression_decode_scratch_buffer_size`.If `scratch_buffer` is `nil`, the function creates and manages its own scratch space, but with a possible performance hit.
    /// * `algorithm` - Set to the desired algorithm: `COMPRESSION_LZ4`, `COMPRESSION_ZLIB`, `COMPRESSION_LZMA`, or `COMPRESSION_LZFSE`.
    ///
    /// # Returns
    ///
    /// The number of bytes written to the destination buffer after decompressing the input. If there is not enough space in the destination buffer to hold the entire decompressed output, the function writes the first dst_size bytes to the buffer and returns dst_size. Note that this behavior differs from that of `compression_encode_buffer`.
    pub fn compression_decode_buffer(
        dst_buffer: *mut u8,
        dst_size: ssize_t,
        src: *const u8,
        src_size: ssize_t,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> ssize_t;
}
