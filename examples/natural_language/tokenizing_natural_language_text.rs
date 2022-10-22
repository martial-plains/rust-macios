use std::ops::Range;

use rust_macios::{
    foundation::NSRange,
    natural_language::{NLTokenUnit, NLTokenizer},
    objective_c_runtime::traits::PNSObject,
};

fn main() {
    let text = "All human beings are born free and equal in dignity and rights.\
                    They are endowed with reason and conscience and should act towards one another in a spirit of brotherhood.";

    let mut tokenizer = NLTokenizer::m_new().init_with_unit(NLTokenUnit::Word);
    tokenizer.set_string(text.into());

    tokenizer.enumerate_tokens_in_range_using_block(
        (0..text.len()).into(),
        |range: NSRange, _, _| {
            let range: Range<usize> = range.into();
            println!("{}", &text[range])
        },
    )
}
