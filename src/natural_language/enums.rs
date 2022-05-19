/// Constants for linguistic tagger enumeration specifying which tokens to omit and whether to join names.
#[derive(Debug)]
#[repr(u64)]
pub enum NLTaggerOptions {
    /// Omit tokens of type word (items considered to be words).
    OmitWords = 1 << 0,
    /// Omit tokens of type punctuation (all punctuation).
    OmitPunctuation = 1 << 1,
    /// Omit tokens of type whitespace (whitespace of all sorts).
    OmitWhitespace = 1 << 2,
    /// Omit tokens of type other (non-linguistic items, such as symbols).
    OmitOther = 1 << 3,
    /// Typically, multiple-word names will be returned as multiple tokens, following the standard tokenization practice of the tagger.
    JoinNames = 1 << 4,
    /// Contractions will be returned as one token.
    JoinContractions = 1 << 5,
}

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
#[derive(Debug)]
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
#[derive(Debug)]
#[repr(u64)]
pub enum NLTokenizerAttributes {
    /// The string contains numbers.
    Numeric = 1 << 0,
    /// The string contains symbols.
    Symbolic = 1 << 1,
    /// The string contains emoji.
    Emoji = 1 << 2,
}
