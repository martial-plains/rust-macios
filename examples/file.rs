use apple_frameworks::natural_language::NLLanguageRecognizer;

fn main() {
    let recognizer = NLLanguageRecognizer::init();

    recognizer.process_string("こにちは");

    let language = recognizer.dominant_language();

    println!("{}", language);
}
