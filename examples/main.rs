use std::ffi::CStr;

use apple_frameworks::foundation::{Array, NSNumber, UInt};

fn main() {
    let initial = vec![NSNumber::from(1), NSNumber::from(2), NSNumber::from(3)];

    for item in initial.iter() {
        println!("{}", item);
    }
}
