use std::ops::Range;

use rust_macios::{
    foundation::NSRange,
    natural_language::{
        nl_tag::{OrganizationName, PersonalName, PlaceName},
        nl_tag_scheme::NameType,
        NLTag, NLTagger, NLTaggerOptions, NLTokenUnit,
    },
    nsarray,
    objective_c_runtime::{nil, traits::PNSObject},
};

fn main() {
    unsafe {
        let text = "The American Red Cross was established in Washington, D.C., by Clara Barton.";

        let mut tagger = NLTagger::m_new().init_with_tag_schemes(nsarray![NameType.clone()]);
        tagger.set_string(Some(text.into()));

        let options = [
            NLTaggerOptions::OmitPunctuation,
            NLTaggerOptions::OmitWhitespace,
            NLTaggerOptions::JoinNames,
        ];
        let tags = nsarray![
            PersonalName.clone(),
            PlaceName.clone(),
            OrganizationName.clone()
        ];

        let tags = tags;

        tagger
            .clone()
            .enumerate_tags_in_range_unit_scheme_options_using_block(
                (0..text.len()).into(),
                NLTokenUnit::Word,
                &NameType,
                &options,
                move |tag: NLTag, token_range: NSRange, _| {
                    // Get the most likely tag, and print it if it's a named entity.
                    if tags.contains(tag.clone()) && tag.m_self() != nil {
                        let token_range: Range<usize> = token_range.into();
                        println!("{}: {}", &text[token_range], tag);
                    }

                    // Get multiple possible tags with their associated confidence scores.
                    let hypotheses = tagger
                        .tag_hypotheses_at_index_unit_scheme_maximum_count_token_range(
                            token_range.location,
                            NLTokenUnit::Word,
                            &NameType,
                            1,
                            None,
                        );

                    println!("{hypotheses}")
                },
            )
    }
}
