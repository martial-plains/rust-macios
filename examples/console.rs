use rust_macios::foundation::NSLocale;

fn main() {
    let ident = NSLocale::current_locale().locale_identifier();

    println!("{}", ident);
}
