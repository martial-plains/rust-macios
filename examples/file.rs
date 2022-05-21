use apple_frameworks::foundation::Locale;
use apple_frameworks::natural_language::NLLanguageRecognizer;

fn main() {
    let locale = Locale::init_with_locale_identifier("en_US");

    println!("{}", locale);
}
