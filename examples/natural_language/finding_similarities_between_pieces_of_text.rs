use rust_macios::{
    foundation::NSString,
    natural_language::{English, NLDistanceType, NLEmbedding},
};

fn main() {
    unsafe {
        // Find similar words
        if let Some(embedding) = NLEmbedding::word_embedding_for_language(English.clone()) {
            let word = "bicycle";

            let vector = embedding.vector_for_string(&word.into());
            println!("{vector}");

            let specific_distance = embedding.distance_between_string_and_string_distance_type(
                &word.into(),
                &"motorcycle".into(),
                NLDistanceType::Cosine,
            );

            println!("{specific_distance}");

            embedding.enumerate_neighbors_for_string_maximum_count_distance_type_using_block(
                &word.into(),
                5,
                NLDistanceType::Cosine,
                |neighbor: NSString, distance: f64, _| {
                    println!("{neighbor}: {distance}");
                },
            )
        }

        // Find similar sentences
        if let Some(sentence_embedding) =
            NLEmbedding::sentence_embedding_for_language(English.clone())
        {
            let sentence = "This is a sentence.";

            let vector = sentence_embedding.vector_for_string(&sentence.into());
            println!("{vector}");

            let distance = sentence_embedding.distance_between_string_and_string_distance_type(
                &sentence.into(),
                &"That is a sentence.".into(),
                NLDistanceType::Cosine,
            );

            println!("{distance}");
        }
    }
}
