use rust_macios::{
    foundation::{NSLocale},
};

fn main() {
    let locale = NSLocale::iso_country_codes();

    println!("{:?}", locale);
}
