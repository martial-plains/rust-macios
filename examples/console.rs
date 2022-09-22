use rust_macios::foundation::{LatinToHiragana, NSString};

fn main() {
    println!(
        "{}",
        unsafe {
            NSString::from("Hiragana")
                .string_by_applying_transform_reverse(&*LatinToHiragana, false)
        }
        .unwrap()
    );
}
