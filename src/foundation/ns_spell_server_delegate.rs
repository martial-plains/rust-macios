use std::sync::Once;

use objc::{
    class,
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};

use crate::objective_c_runtime::{id, traits::PNSObject};

use super::{
    Int, NSArray, NSDictionary, NSOrthography, NSRange, NSSpellServer, NSString,
    NSTextCheckingResult, NSTextCheckingTypes, UInt, NSSPELLSERVER_PTR,
};

/// The optional methods implemented by the delegate of a spell server.
pub trait PNSSpellServerDelegate
where
    Self: PNSObject,
{
    /* Check Grammar and Spelling in Strings
     */

    /// Gives the delegate the opportunity to analyze both the spelling and grammar simultaneously, which is more efficient.
    #[allow(clippy::too_many_arguments)]
    fn spell_server_check_string_offset_types_options_orthography_word_count(
        &self,
        _sender: NSSpellServer,
        _string_to_check: NSString,
        _offset: UInt,
        _checking_types: NSTextCheckingTypes,
        _options: NSDictionary<NSString, id>,
        _orthography: NSOrthography,
        _word_count: Int,
    ) -> NSArray<NSTextCheckingResult> {
        NSArray::default()
    }

    /// Gives the delegate the opportunity to suggest guesses to the sender for the correct spelling of the given misspelled word in the specified language.
    fn spell_server_suggest_guesses_for_word_in_language(
        &self,
        _sender: NSSpellServer,
        _word: NSString,
        _language: NSString,
    ) -> NSArray<NSString> {
        NSArray::default()
    }

    /// Gives the delegate the opportunity to customize the grammatical analysis of a given string.
    fn spell_server_check_grammar_in_string_language_details(
        &self,
        _sender: NSSpellServer,
        _string_to_check: NSString,
        _language: NSString,
        _details: NSArray<NSDictionary<NSString, id>>,
    ) -> NSRange {
        NSRange::default()
    }

    /// Asks the delegate to search for a misspelled word in a given string, using the specified language, and marking the first misspelled word found by returning its range within the string.
    fn spell_server_find_misspelled_word_in_string_language_word_count_count_only(
        &self,
        _sender: NSSpellServer,
        _string_to_check: NSString,
        _language: NSString,
        _word_count: Int,
        _count_only: bool,
    ) -> NSRange {
        NSRange::default()
    }

    /* Managing the Spelling Dictionary
     */

    /// Notifies the delegate that the sender has removed the specified word from the user’s list of acceptable words in the specified language.
    fn spell_server_did_forget_word_in_language(
        &self,
        _sender: NSSpellServer,
        _word: NSString,
        _language: NSString,
    ) {
    }

    /// Notifies the delegate that the sender has added the specified word to the user’s list of acceptable words in the specified language.
    fn spell_server_did_learn_word_in_language(
        &self,
        _sender: NSSpellServer,
        _word: NSString,
        _language: NSString,
    ) {
    }

    fn spell_server_suggest_completions_for_partial_word_range_in_string_language(
        &self,
        _sender: NSSpellServer,
        _range: NSRange,
        _string: NSString,
        _language: NSString,
    ) -> NSArray<NSString> {
        NSArray::default()
    }

    fn spell_server_record_response_to_correction_for_word_language(
        &self,
        _sender: NSSpellServer,
        _response: UInt,
        _correction: NSString,
        _word: NSString,
        _language: NSString,
    ) {
    }
}

fn ns_spell_server<T>(this: &mut Object) -> &mut T {
    unsafe {
        let ns_spell_server_ptr: usize = *this.get_ivar(NSSPELLSERVER_PTR);
        let ns_spell_server = ns_spell_server_ptr as *mut T;
        &mut *ns_spell_server
    }
}

extern "C" fn spell_server_check_string_offset_types_options_orthography_word_count<
    T: PNSSpellServerDelegate,
>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    string_to_check: NSString,
    offset: UInt,
    checking_types: NSTextCheckingTypes,
    options: NSDictionary<NSString, *mut Object>,
    orthography: NSOrthography,
    word_count: Int,
) -> NSArray<NSTextCheckingResult> {
    ns_spell_server::<T>(this)
        .spell_server_check_string_offset_types_options_orthography_word_count(
            sender,
            string_to_check,
            offset,
            checking_types,
            options,
            orthography,
            word_count,
        )
}

extern "C" fn spell_server_suggest_guesses_for_word_in_language<T: PNSSpellServerDelegate>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    word: NSString,
    language: NSString,
) -> NSArray<NSString> {
    ns_spell_server::<T>(this)
        .spell_server_suggest_guesses_for_word_in_language(sender, word, language)
}

extern "C" fn spell_server_check_grammar_in_string_language_details<T: PNSSpellServerDelegate>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    string_to_check: NSString,
    language: NSString,
    details: NSArray<NSDictionary<NSString, id>>,
) -> NSRange {
    ns_spell_server::<T>(this).spell_server_check_grammar_in_string_language_details(
        sender,
        string_to_check,
        language,
        details,
    )
}

extern "C" fn spell_server_find_misspelled_word_in_string_language_word_count_count_only<
    T: PNSSpellServerDelegate,
>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    string_to_check: NSString,
    language: NSString,
    word_count: Int,
    count_only: bool,
) -> NSRange {
    ns_spell_server::<T>(this)
        .spell_server_find_misspelled_word_in_string_language_word_count_count_only(
            sender,
            string_to_check,
            language,
            word_count,
            count_only,
        )
}

extern "C" fn spell_server_did_forget_word_in_language<T: PNSSpellServerDelegate>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    word: NSString,
    language: NSString,
) {
    ns_spell_server::<T>(this).spell_server_did_forget_word_in_language(sender, word, language)
}

extern "C" fn spell_server_did_learn_word_in_language<T: PNSSpellServerDelegate>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    word: NSString,
    language: NSString,
) {
    ns_spell_server::<T>(this).spell_server_did_learn_word_in_language(sender, word, language)
}

extern "C" fn spell_server_suggest_completions_for_partial_word_range_in_string_language<
    T: PNSSpellServerDelegate,
>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    range: NSRange,
    string: NSString,
    language: NSString,
) -> NSArray<NSString> {
    ns_spell_server::<T>(this)
        .spell_server_suggest_completions_for_partial_word_range_in_string_language(
            sender, range, string, language,
        )
}

extern "C" fn spell_server_record_response_to_correction_for_word_language<
    T: PNSSpellServerDelegate,
>(
    this: &mut Object,
    _: Sel,
    sender: NSSpellServer,
    response: UInt,
    correction: NSString,
    word: NSString,
    language: NSString,
) {
    ns_spell_server::<T>(this).spell_server_record_response_to_correction_for_word_language(
        sender, response, correction, word, language,
    )
}

pub fn register_ns_spell_server_delegate_class<T>() -> *const Class
where
    T: PNSSpellServerDelegate,
{
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSTNSSpellServerDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(NSSPELLSERVER_PTR);

        decl.add_method(
            sel!(spellServer:checkString:offset:types:options:orthography:wordCount:),
            spell_server_check_string_offset_types_options_orthography_word_count::<T>
                as extern "C" fn(
                    &mut Object,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) -> NSArray<NSTextCheckingResult>,
        );

        decl.add_method(
            sel!(spellServer:suggestGuessesForWord:inLanguage:),
            spell_server_suggest_guesses_for_word_in_language::<T>
                as extern "C" fn(&mut Object, _, _, _, _) -> NSArray<NSString>,
        );

        decl.add_method(
            sel!(spellServer:checkGrammarInString:language:details:),
            spell_server_check_grammar_in_string_language_details::<T>
                as extern "C" fn(&mut Object, _, _, _, _, _) -> NSRange,
        );

        decl.add_method(
            sel!(spellServer:findMisspelledWordInString:language:wordCount:countOnly:),
            spell_server_find_misspelled_word_in_string_language_word_count_count_only::<T>
                as extern "C" fn(&mut Object, _, _, _, _, _, _) -> NSRange,
        );

        decl.add_method(
            sel!(spellServer:didForgetWord:inLanguage:),
            spell_server_did_forget_word_in_language::<T> as extern "C" fn(&mut Object, _, _, _, _),
        );

        decl.add_method(
            sel!(spellServer:didLearnWord:inLanguage:),
            spell_server_did_learn_word_in_language::<T> as extern "C" fn(&mut Object, _, _, _, _),
        );

        decl.add_method(
            sel!(spellServer:suggestCompletionsForPartialWordRange:inString:language:
            ),
            spell_server_suggest_completions_for_partial_word_range_in_string_language::<T>
                as extern "C" fn(&mut Object, _, _, _, _, _) -> NSArray<NSString>,
        );

        decl.add_method(
            sel!(spellServer:recordResponse:toCorrection:forWord:language:),
            spell_server_record_response_to_correction_for_word_language::<T>
                as extern "C" fn(&mut Object, _, _, _, _, _, _),
        );

        DELEGATE_CLASS = decl.register();
    });

    unsafe { DELEGATE_CLASS }
}
