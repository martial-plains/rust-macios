use apple_frameworks::foundation::{Locale, String};
use apple_frameworks::natural_language::NLLanguageRecognizer;

fn main() {
    let local = Locale::current();
    println!("{}", local.locale_identifier());
}
