use rust_macios::{
    natural_language::{
        English, French, Italian, NLLanguageRecognizer, Portuguese, Spanish, Swedish,
    },
    nsarray, nsdictionary,
    objective_c_runtime::traits::PNSObject,
};

fn main() {
    // Create a language recognizer.
    let mut recognizer = NLLanguageRecognizer::m_new();
    recognizer.process_string(&"This is a test, min vän.".into());

    // Identify the dominant language.
    if let Some(language) = recognizer.dominant_language() {
        println!("{language}")
    } else {
        println!("Language not recognized")
    }

    // Generate up to two language hypotheses.
    let hypotheses = recognizer.language_hypotheses_with_maximum(2);
    println!("{hypotheses}");

    // Specify constraints for language identification.
    unsafe {
        recognizer.set_language_constraints(&nsarray![
            French.clone(),
            English.clone(),
            Swedish.clone(),
            Italian.clone(),
            Spanish.clone(),
            Portuguese.clone()
        ]);

        recognizer.set_language_hints(&nsdictionary! {
            French.clone() => 0.5.into(),
            English.clone() => 0.9.into(),
            Swedish.clone() => 0.8.into(),
            Italian.clone() => 0.6.into(),
            Spanish.clone() => 0.3.into(),
            Portuguese.clone() => 0.2.into(),
        })
    }

    let constrained_hypotheses = recognizer.language_hypotheses_with_maximum(2);
    println!("{constrained_hypotheses}");

    // Reset the recognizer to its initial state.
    recognizer.reset();

    // Process additional strings for language identification.
    recognizer.process_string(&"Este es un idioma diferente.".into());
}
