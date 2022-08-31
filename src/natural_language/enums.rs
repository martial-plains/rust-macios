/// The different types of a natural language model.
#[derive(Debug)]
#[repr(i64)]
pub enum NLModelType {
    /// A classifier model type that tags text at the phrase, sentence, paragraph, or higher level.
    Classifier,
    /// A sequence model type that tags text at the token level.
    Sequence,
}

/// Constants representing linguistic units.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i64)]
pub enum NLTokenUnit {
    /// An individual word.
    Word,
    /// An individual sentence.
    Sentence,
    /// An individual paragraph.
    Paragraph,
    /// The document in its entirety.
    Document,
}

/// Hints about the contents of the string for the tokenizer.
#[derive(Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum NLTokenizerAttributes {
    /// Doesn't contain any special attributes.
    None = 0,
    /// The string contains numbers.
    Numeric = 1 << 0,
    /// The string contains symbols.
    Symbolic = 1 << 1,
    /// The string contains emoji.
    Emoji = 1 << 2,
}
