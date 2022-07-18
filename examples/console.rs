use rust_macios::foundation::{NSUUID, NSLocale};

fn main() {
    println!("From Class: {}", NSLocale::current_locale());
}
