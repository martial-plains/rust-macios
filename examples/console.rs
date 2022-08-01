use rust_macios::foundation::NSLocale;

fn main() {
    println!("From Class: {}", NSLocale::current_locale());
}
