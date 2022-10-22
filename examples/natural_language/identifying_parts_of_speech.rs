use std::ops::Range;

use rust_macios::{
    foundation::{macros::nsarray, NSRange},
    natural_language::{
        nl_tag_scheme::LexicalClass, NLTag, NLTagger, NLTaggerOptions, NLTokenUnit,
    },
    objective_c_runtime::{nil, traits::PNSObject},
};

fn main() {
    unsafe {
        let text = "The ripe taste of cheese improves with age.";
        let mut tagger = NLTagger::m_new().init_with_tag_schemes(nsarray![LexicalClass.clone()]);
        tagger.set_string(Some(text.into()));

        let options = [
            NLTaggerOptions::OmitPunctuation,
            NLTaggerOptions::OmitWhitespace,
        ];

        tagger.enumerate_tags_in_range_unit_scheme_options_using_block(
            (0..text.len()).into(),
            NLTokenUnit::Word,
            &LexicalClass,
            &options,
            |tag: NLTag, token_range: NSRange, _| {
                if tag.m_self() != nil {
                    let token_range: Range<usize> = token_range.into();
                    println!("{}: {}", &text[token_range], tag)
                }
            },
        )
    }
}
