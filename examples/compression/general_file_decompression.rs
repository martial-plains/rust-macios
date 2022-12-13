use std::env::temp_dir;

use rust_macios::{
    compression::{compression_algorithm, compression_decode_buffer, compression_encode_buffer},
    kernel::size_t,
};

fn main() {
    // Create the Source Data
    let source_string = r#"
    Lorem ipsum dolor sit amet consectetur adipiscing elit mi
    nibh ornare proin blandit diam ridiculus, faucibus mus
    dui eu vehicula nam donec dictumst sed vivamus bibendum
    aliquet efficitur. Felis imperdiet sodales dictum morbi
    vivamus augue dis duis aliquet velit ullamcorper porttitor,
    lobortis dapibus hac purus aliquam natoque iaculis blandit
    montes nunc pretium.
    "#;

    let source_buffer = source_string.as_bytes();

    // Create the Destination Buffer
    let mut dest_buffer = vec![0; source_string.len()];

    // Select a Compression Algorithm
    let algorithm = compression_algorithm::LZFSE;

    // Compress the Data
    let compressed_size = unsafe {
        compression_encode_buffer(
            dest_buffer.as_mut_ptr(),
            source_string.len() as size_t,
            source_buffer.as_ptr(),
            source_string.len() as size_t,
            std::ptr::null_mut(),
            algorithm,
        )
    };

    // When working with small files, the compression may fail and
    // ['compression_encode_buffer'] returns 0
    if compressed_size == 0 {
        // You may elect to handle this situation differently, for example, by
        // displaying a warning to the user that the compression failed.
        panic!("Encoding failed.")
    }

    // Write the Encoded Data to a File
    // The following code writes the encoded data to a file in the the app's temporary directory:
    let mut dir = temp_dir();

    let encoded_file_name = format!("{}.LZFSE", "stringEncoded");
    dir.push(&encoded_file_name);

    std::fs::write(&dir, dest_buffer).unwrap();

    // Read from encoded file
    let encoded_buf = std::fs::read(dir).unwrap();

    // Decompress the Data
    const DECODED_CAPACITY: usize = 8_000_000;

    let mut decoded_destination_buffer = [0; DECODED_CAPACITY];
    let decoded_char_count = unsafe {
        compression_decode_buffer(
            decoded_destination_buffer.as_mut_ptr(),
            DECODED_CAPACITY as size_t,
            encoded_buf.as_ptr(),
            encoded_buf.len() as size_t,
            std::ptr::null_mut(),
            algorithm,
        )
    };

    println!("{decoded_char_count} characters were decoded");
    println!(
        "Decoded message: {}",
        String::from_utf8_lossy(&decoded_destination_buffer)
    );
}
