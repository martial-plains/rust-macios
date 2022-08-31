use rust_macios::foundation::{NSDecimal, NSLocale, NSNumber};

fn main() {
    let fake_number: NSDecimal = NSDecimal::from("one");

    println!(
        "{}",
        NSNumber::ns_decimal_is_not_a_number(&fake_number as *const _)
    );
}
