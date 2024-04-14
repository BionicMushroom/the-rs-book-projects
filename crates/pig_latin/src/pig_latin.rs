use icu::casemap::titlecase::{LeadingAdjustment, TitlecaseOptions, TrailingCase};
use icu::casemap::{CaseMapper, TitlecaseMapper};
use icu::locid::LanguageIdentifier;
use icu::properties::{maps, GeneralCategory};
use icu::segmenter::{GraphemeClusterSegmenter, WordSegmenter};

use itertools::Itertools;

pub trait Convertible {
    fn to_pig_latin(&self) -> String;
}

impl Convertible for &str {
    fn to_pig_latin(&self) -> String {
        let lang = LanguageIdentifier::default();
        let case_mapper = CaseMapper::new();
        let titlecase_mapper = TitlecaseMapper::new_with_mapper(&case_mapper);

        let mut titlecase_options = TitlecaseOptions::default();
        titlecase_options.trailing_case = TrailingCase::Unchanged;
        titlecase_options.leading_adjustment = LeadingAdjustment::None;

        let grapheme_cluster_segmenter = GraphemeClusterSegmenter::new();
        let mut pig_lating_string = String::with_capacity(self.len());

        let mut algorithm_data = AlgorithmData {
            lang: &lang,
            case_mapper: &case_mapper,
            titlecase_mapper: &titlecase_mapper,
            titlecase_options,
            grapheme_cluster_segmenter: &grapheme_cluster_segmenter,
            pig_latin_string: &mut pig_lating_string,
        };

        str_to_pig_latin(self, &mut algorithm_data);

        pig_lating_string
    }
}

impl Convertible for String {
    fn to_pig_latin(&self) -> String {
        self.as_str().to_pig_latin()
    }
}

struct AlgorithmData<'a> {
    lang: &'a LanguageIdentifier,
    case_mapper: &'a CaseMapper,
    titlecase_mapper: &'a TitlecaseMapper<&'a CaseMapper>,
    titlecase_options: TitlecaseOptions,
    grapheme_cluster_segmenter: &'a GraphemeClusterSegmenter,
    pig_latin_string: &'a mut String,
}

struct FirstVowelData {
    byte_index: usize,
    is_uppercase_or_titlecase: bool,
}

enum WordCasing {
    None,
    Capitalized,
    Uppercase,
}

struct WordData {
    first_vowel_data: Option<FirstVowelData>,
    casing: WordCasing,
}

fn str_to_pig_latin(string: &str, algorithm_data: &mut AlgorithmData) {
    for (start, end) in WordSegmenter::new_dictionary()
        .segment_str(string)
        .tuple_windows()
    {
        word_to_pig_latin(&string[start..end], algorithm_data);
    }
}

fn word_to_pig_latin(word: &str, algorithm_data: &mut AlgorithmData) {
    convert_word_to_pig_latin(word, algorithm_data, &get_word_data(word, algorithm_data));
}

fn get_word_data(word: &str, algorithm_data: &AlgorithmData) -> WordData {
    let mut first_vowel_data = Option::<FirstVowelData>::None;
    let mut is_uppercase = false;
    let mut is_capitalized = false;

    let mut grapheme_clusters = algorithm_data
        .grapheme_cluster_segmenter
        .segment_str(word)
        .tuple_windows();

    if let Some((start, end)) = grapheme_clusters.next() {
        let grapheme_cluster = &word[start..end];
        let mut chars = grapheme_cluster.chars();

        if let Some(first_char) = chars.next() {
            is_capitalized = is_uppercase_or_titlecase(first_char);
            is_uppercase = is_capitalized;

            is_uppercase &= chars.all(is_uppercase_or_titlecase);

            if grapheme_cluster.len() == 1 && is_vowel(first_char) {
                first_vowel_data = Some(FirstVowelData {
                    byte_index: 0,
                    is_uppercase_or_titlecase: is_capitalized,
                });
            }
        }
    }

    for (start, end) in grapheme_clusters {
        let grapheme_cluster = &word[start..end];
        let mut chars = grapheme_cluster.chars();

        if let Some(first_char) = chars.next() {
            let is_first_char_uppercase_or_titlecase = is_uppercase_or_titlecase(first_char);

            is_uppercase &= is_first_char_uppercase_or_titlecase;
            is_uppercase &= chars.all(is_uppercase_or_titlecase);

            if grapheme_cluster.len() == 1 && is_vowel(first_char) {
                first_vowel_data.get_or_insert(FirstVowelData {
                    byte_index: start,
                    is_uppercase_or_titlecase: is_first_char_uppercase_or_titlecase,
                });
            }
        }
    }

    WordData {
        first_vowel_data,
        casing: if is_uppercase {
            WordCasing::Uppercase
        } else if is_capitalized {
            WordCasing::Capitalized
        } else {
            WordCasing::None
        },
    }
}

fn convert_word_to_pig_latin(word: &str, algorithm_data: &mut AlgorithmData, word_data: &WordData) {
    match word_data.first_vowel_data {
        None => algorithm_data.pig_latin_string.push_str(word),
        Some(FirstVowelData { byte_index: 0, .. }) => {
            algorithm_data.pig_latin_string.push_str(word);

            match word_data.casing {
                WordCasing::Uppercase => algorithm_data.pig_latin_string.push_str("-HAY"),
                _ => algorithm_data.pig_latin_string.push_str("-hay"),
            }
        }
        Some(FirstVowelData {
            byte_index: vowel_byte_index,
            is_uppercase_or_titlecase: is_vowel_uppercase_or_titlecase,
        }) => {
            let (second_pig_latin_part_str, first_pig_latin_part_str) =
                word.split_at(vowel_byte_index);

            let mut second_pig_latin_part_string =
                String::with_capacity(second_pig_latin_part_str.len() + 2);
            second_pig_latin_part_string += second_pig_latin_part_str;
            second_pig_latin_part_string += "ay";

            match word_data.casing {
                WordCasing::None => {
                    if is_vowel_uppercase_or_titlecase {
                        algorithm_data.pig_latin_string.push_str(
                            algorithm_data
                                .titlecase_mapper
                                .titlecase_segment_to_string(
                                    first_pig_latin_part_str,
                                    algorithm_data.lang,
                                    algorithm_data.titlecase_options,
                                )
                                .as_str(),
                        );
                        algorithm_data.pig_latin_string.push('-');
                        algorithm_data.pig_latin_string.push_str(
                            algorithm_data
                                .case_mapper
                                .lowercase_to_string(
                                    &second_pig_latin_part_string,
                                    algorithm_data.lang,
                                )
                                .as_str(),
                        );
                    } else {
                        algorithm_data
                            .pig_latin_string
                            .push_str(first_pig_latin_part_str);
                        algorithm_data.pig_latin_string.push('-');
                        algorithm_data.pig_latin_string.push_str(
                            algorithm_data
                                .case_mapper
                                .lowercase_to_string(
                                    &second_pig_latin_part_string,
                                    algorithm_data.lang,
                                )
                                .as_str(),
                        );
                    }
                }
                WordCasing::Capitalized => {
                    algorithm_data.pig_latin_string.push_str(
                        algorithm_data
                            .titlecase_mapper
                            .titlecase_segment_to_string(
                                first_pig_latin_part_str,
                                algorithm_data.lang,
                                algorithm_data.titlecase_options,
                            )
                            .as_str(),
                    );
                    algorithm_data.pig_latin_string.push('-');
                    algorithm_data.pig_latin_string.push_str(
                        algorithm_data
                            .case_mapper
                            .lowercase_to_string(&second_pig_latin_part_string, algorithm_data.lang)
                            .as_str(),
                    );
                }
                WordCasing::Uppercase => {
                    algorithm_data.pig_latin_string.push_str(
                        algorithm_data
                            .case_mapper
                            .uppercase_to_string(first_pig_latin_part_str, algorithm_data.lang)
                            .as_str(),
                    );
                    algorithm_data.pig_latin_string.push('-');
                    algorithm_data.pig_latin_string.push_str(
                        algorithm_data
                            .case_mapper
                            .uppercase_to_string(&second_pig_latin_part_string, algorithm_data.lang)
                            .as_str(),
                    );
                }
            }
        }
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a'
        || c == 'A'
        || c == 'e'
        || c == 'E'
        || c == 'i'
        || c == 'I'
        || c == 'o'
        || c == 'O'
        || c == 'u'
        || c == 'U'
}

fn is_uppercase_or_titlecase(c: char) -> bool {
    maps::general_category().get(c) != GeneralCategory::LowercaseLetter
}
