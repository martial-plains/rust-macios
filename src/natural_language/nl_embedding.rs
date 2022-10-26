use block::{ConcreteBlock, IntoConcreteBlock};
use libc::{c_double, c_float};
use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSArray, NSDictionary, NSError, NSIndexSet, NSNumber, NSString, UInt, NSURL},
    objective_c_runtime::{
        macros::{interface_impl, object},
        nil,
        traits::{FromId, PNSObject},
    },
    utils::{to_bool, to_optional},
};

use super::NLLanguage;

/// The distance between two strings in a text embedding.
pub type NLDistance = c_double;

/// The means of calculating a distance between two locations in a text embedding.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(i64)]
pub enum NLDistanceType {
    /// A method of calculating distance by using cosine similarity.
    Cosine,
}

object! {
    /// A map of strings to vectors, which locates neighboring, similar strings.
    unsafe pub struct NLEmbedding;
}

#[interface_impl(NSObject)]
impl NLEmbedding {
    /* Creating a word embedding
     */

    /// Retrieves a word embedding for a given language.
    #[method]
    pub fn word_embedding_for_language(language: NLLanguage) -> Option<NLEmbedding> {
        unsafe {
            to_optional(msg_send![
                Self::m_class(),
                wordEmbeddingForLanguage: language
            ])
        }
    }

    /// Retrieves a word embedding for a given language and revision.
    #[method]
    pub fn word_embedding_for_language_revision(
        language: NLLanguage,
        revision: UInt,
    ) -> Option<NLEmbedding> {
        unsafe {
            to_optional(
                msg_send![Self::m_class(), wordEmbeddingForLanguage:language revision: revision],
            )
        }
    }

    /// Creates a word embedding from a model file.
    #[method]
    pub fn embedding_with_contents_of_url(url: &NSURL) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        let mut error = NSError::m_alloc();

        let ptr = unsafe {
            Self::from_id(
                msg_send![Self::m_class(), embeddingWithContentsOfURL:url.m_self() error: &mut error],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(ptr)
        }
    }

    /* Creating a sentence embedding
     */

    /// Retrieves a sentence embedding for a given language.
    #[method]
    pub fn sentence_embedding_for_language(language: NLLanguage) -> Option<NLEmbedding> {
        unsafe {
            to_optional(msg_send![
                Self::m_class(),
                sentenceEmbeddingForLanguage: language
            ])
        }
    }

    /// Retrieves a sentence embedding for a given language and revision.
    #[method]
    pub fn sentence_embedding_for_language_revision(
        language: NLLanguage,
        revision: UInt,
    ) -> Option<NLEmbedding> {
        unsafe {
            to_optional(msg_send![
                Self::m_class(),
                sentenceEmbeddingForLanguage: language revision: revision
            ])
        }
    }

    /* Finding strings and their distances in an embedding
     */

    /// Retrieves a limited number of strings near a string in the vocabulary.
    #[method]
    pub fn neighbors_for_string_maximum_count_distance_type(
        &self,
        string: &NSString,
        max_count: UInt,
        distance_type: NLDistanceType,
    ) -> NSArray<NSString> {
        unsafe {
            NSArray::from_id(
                msg_send![self.m_self(), neighborsForString:string.m_self() maximumCount: max_count distanceType: distance_type],
            )
        }
    }

    /// Retrieves a limited number of strings, within a radius of a string, in the vocabulary.
    #[method]
    pub fn neighbors_for_string_maximum_count_maximum_distance_distance_type(
        &self,
        string: &NSString,
        max_count: UInt,
        max_distance: NLDistance,
        distance_type: NLDistanceType,
    ) -> NSArray<NSString> {
        unsafe {
            NSArray::from_id(
                msg_send![self.m_self(), neighborsForString: string.m_self() maximumCount: max_count maximumDistance: max_distance distanceType: distance_type],
            )
        }
    }

    /// Retrieves a limited number of strings near a location in the vocabulary space.
    #[method]
    pub fn neighbors_for_vector_maximum_count_distance_type(
        &self,
        vector: &NSArray<NSNumber>,
        max_count: UInt,
        distance_type: NLDistanceType,
    ) -> NSArray<NSString> {
        unsafe {
            NSArray::from_id(
                msg_send![self.m_self(), neighborsForVector:vector.m_self() maximumCount: max_count distanceType: distance_type],
            )
        }
    }

    /// Retrieves a limited number of strings within a radius of a location in the vocabulary space.
    #[method]
    pub fn neighbors_for_vector_maximum_count_maximum_distance_distance_type(
        &self,
        vector: &NSArray<NSNumber>,
        max_count: UInt,
        max_distance: NLDistance,
        distance_type: NLDistanceType,
    ) -> NSArray<NSString> {
        unsafe {
            NSArray::from_id(
                msg_send![self.m_self(), neighborsForVector: vector.m_self() maximumCount: max_count maximumDistance: max_distance distanceType: distance_type],
            )
        }
    }

    /// Passes the nearest strings of a string in the vocabulary to a block.
    #[method]
    pub fn enumerate_neighbors_for_string_maximum_count_distance_type_using_block<F>(
        &self,
        string: &NSString,
        max_count: UInt,
        distance_type: NLDistanceType,
        block: F,
    ) where
        F: IntoConcreteBlock<(NSString, NLDistance, *mut bool), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), enumerateNeighborsForString: string.m_self() maximumCount: max_count distanceType: distance_type usingBlock: block]
        }
    }

    /// Passes the nearest strings, within a radius of a string in the vocabulary, to a block.
    #[method]
    pub fn enumerate_neighbors_for_string_maximum_count_maximum_distance_distance_type_using_block<
        F,
    >(
        &self,
        string: &NSString,
        max_count: UInt,
        max_distance: NLDistance,
        distance_type: NLDistanceType,
        block: F,
    ) where
        F: IntoConcreteBlock<(NSString, NLDistance, *mut bool), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), enumerateNeighborsForString: string.m_self() maximumCount: max_count maximumDistance: max_distance distanceType: distance_type usingBlock: block]
        }
    }

    /// Passes the nearest strings of a location in the vocabulary space to a block.
    #[method]
    pub fn enumerate_neighbors_for_vector_maximum_count_distance_type_using_block<F>(
        &self,
        vector: &NSArray<NSNumber>,
        max_count: UInt,
        distance_type: NLDistanceType,
        block: F,
    ) where
        F: IntoConcreteBlock<(NSString, NLDistance, *mut bool), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), enumerateNeighborsForVector: vector.m_self() maximumCount: max_count distanceType: distance_type usingBlock: block]
        }
    }

    /// Passes the nearest strings, within a radius of a location in the vocabulary space, to a block.
    #[method]
    pub fn enumerate_neighbors_for_vector_maximum_count_maximum_distance_distance_type_using_block<
        F,
    >(
        &self,
        vector: &NSArray<NSNumber>,
        max_count: UInt,
        max_distance: NLDistance,
        distance_type: NLDistanceType,
        block: F,
    ) where
        F: IntoConcreteBlock<(NSString, NLDistance, *mut bool), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), enumerateNeighborsForVector: vector.m_self() maximumCount: max_count maximumDistance: max_distance distanceType: distance_type usingBlock: block]
        }
    }

    /// Calculates the distance between two strings in the vocabulary space.
    #[method]
    pub fn distance_between_string_and_string_distance_type(
        &self,
        first: &NSString,
        second: &NSString,
        distance_type: NLDistanceType,
    ) -> NLDistance {
        unsafe {
            msg_send![self.m_self(), distanceBetweenString: first.m_self() andString: second.m_self() distanceType: distance_type]
        }
    }

    /* Inspecting the vocabulary of an embedding
     */

    /// The number of dimensions in the vocabulary’s vector space.
    #[property]
    pub fn dimension(&self) -> UInt {
        unsafe { msg_send![self.m_self(), dimension] }
    }

    /// The number of words in the vocabulary.
    #[property]
    pub fn vocabulary_size(&self) -> UInt {
        unsafe { msg_send![self.m_self(), vocabularySize] }
    }

    /// The language of the text in the word embedding.
    #[property]
    pub fn language(&self) -> Option<NLLanguage> {
        unsafe { to_optional(msg_send![self.m_self(), language]) }
    }

    /// Requests a Boolean value that indicates whether the term is in the vocabulary.
    #[method]
    pub fn contains_string(&self, string: &NSString) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), containsString: string.m_self()]) }
    }

    /// Requests the vector for the given term.
    #[method]
    pub fn vector_for_string(&self, string: &NSString) -> NSArray<NSNumber> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), vectorForString: string.m_self()]) }
    }

    /// Copies a vector into the given a pointer to a float array.
    #[method]
    pub fn get_vector_for_string(&self, vector: &mut [c_float], string: &NSString) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), getVector: vector forString: string.m_self()]) }
    }

    /// The revision of the word embedding.
    #[property]
    pub fn revision(&self) -> UInt {
        unsafe { msg_send![self.m_self(), revision] }
    }

    /* Saving an embedding
     */

    /// Exports the word embedding contained within a Core ML model file at the given URL.
    #[method]
    pub fn write_embedding_for_dictionary_language_revision_to_url(
        dictionary: &NSDictionary<NSString, NSArray<NSNumber>>,
        language: NLLanguage,
        revision: UInt,
        url: &NSURL,
    ) -> Result<bool, NSError> {
        let mut error = NSError::m_alloc();

        let ptr = unsafe {
            to_bool(
                msg_send![Self::m_class(), writeEmbeddingForDictionary: dictionary.m_self() language: language revision: revision toURL: url.m_self() error: &mut error ],
            )
        };

        if error.m_self() != nil {
            Err(error)
        } else {
            Ok(ptr)
        }
    }

    /* Checking for Natural Language support
     */

    /// Retrieves the current version of a word embedding for the given language.
    #[method]
    pub fn current_revision_for_language(language: NLLanguage) -> UInt {
        unsafe { msg_send![Self::m_class(), currentRevisionForLanguage: language] }
    }

    /// Retrieves all version numbers of a word embedding for the given language.
    #[method]
    pub fn supported_revisions_for_language(language: NLLanguage) -> NSIndexSet {
        unsafe {
            NSIndexSet::from_id(msg_send![
                Self::m_class(),
                supportedRevisionsForLanguage: language
            ])
        }
    }

    /// Retrieves the current version of a sentence embedding for the given language.
    #[method]
    pub fn current_sentence_embedding_revision_for_language(language: NLLanguage) -> UInt {
        unsafe {
            msg_send![
                Self::m_class(),
                currentSentenceEmbeddingRevisionForLanguage: language
            ]
        }
    }

    /// Retrieves all version numbers of a sentence embedding for the given language.
    #[method]
    pub fn supported_sentence_embedding_revisions_for_language(language: NLLanguage) -> NSIndexSet {
        unsafe {
            NSIndexSet::from_id(msg_send![
                Self::m_class(),
                supportedSentenceEmbeddingRevisionsForLanguage: language
            ])
        }
    }
}
