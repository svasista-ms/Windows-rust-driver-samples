use core::slice;

extern crate alloc;

use alloc::string::String;
use wdk::println;
use wdk_sys::PUNICODE_STRING;

// Function to print a PUNICODE_STRING
pub fn print_punicode_string(puni_str: PUNICODE_STRING) {
    let uni_str = unsafe { &*puni_str }; // Safely dereference the pointer
    let len = (uni_str.Length / 2) as usize; // Length is in bytes, divide by 2 for UTF-16
    let buffer_slice = unsafe { slice::from_raw_parts(uni_str.Buffer, len) };

    let mut output = String::new();

    for &wide_char in buffer_slice {
        if wide_char == 0 { // Check for null terminator
            break;
        }
        // Convert UTF-16 to char and append to output string
        if let Some(character) = core::char::from_u32(wide_char as u32) {
            output.push(character);
        }
    }

    // At this point, `output` contains the converted string.
    // Printing in a no_std environment might be limited to debug interfaces or similar.
    // For demonstration, we'll just return the string.
    // In an actual no_std environment, you might write it to a debug interface or log.
    println!("{}", output); // Replace this with your no_std environment's logging mechanism
}