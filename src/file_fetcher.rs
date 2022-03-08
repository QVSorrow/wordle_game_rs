use std::collections::HashSet;
use std::path::PathBuf;
use crate::library::Fetcher;
use crate::word::Word;

pub fn fetcher_from_file(file_path: impl Into<PathBuf>) -> Fetcher {
    let path = file_path.into();
    Fetcher {
        fetch: Box::new(move || {
            let path = path.as_path();
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_path(path)?;
            let index = parser::build_index(&mut reader)?;
            let set = reader.records()
                .filter_map(|r| r.ok())
                .filter_map(|str_record| {
                    let record = parser::Record::from(&str_record, &index);
                    record.valid_word().map(|s| Word::from(s))
                })
                .collect::<HashSet<_>>();
            Ok(set)
        }),
    }
}


mod parser {
    use std::fs::File;
    use std::str::FromStr;
    use csv::{Reader, StringRecord};

    #[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
    enum PartOfLanguage {
        /// іменник
        Noun,
        /// займенник
        Pronoun,
        /// присудкове слово
        PredicateWord,
        /// частка
        Part,
        /// дієслово
        Verb,
        /// прислівник
        Adverb,
        /// дієприслівник
        AdAdverb,
        /// прикметник
        Adjective,
        /// прийменник
        Preposition,
        /// числівник
        Numeral,
        /// сполучник
        Conjunction,
        /// вигук
        Exclamation,
        /// вставне слово
        Parenthesis,
        /// чоловіче ім`я
        MaleName,
        /// жіноче ім`я
        FemaleName,
    }

    impl FromStr for PartOfLanguage {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let part = match s {
                "іменник" => PartOfLanguage::Noun,
                "займенник" => PartOfLanguage::Pronoun,
                "присудкове слово" => PartOfLanguage::PredicateWord,
                "частка" => PartOfLanguage::Part,
                "прислівник" => PartOfLanguage::Adverb,
                "дієприслівник" => PartOfLanguage::AdAdverb,
                "прикметник" => PartOfLanguage::Adjective,
                "прийменник" => PartOfLanguage::Preposition,
                "дієслово" => PartOfLanguage::Verb,
                "числівник" => PartOfLanguage::Numeral,
                "сполучник" => PartOfLanguage::Conjunction,
                "вигук" => PartOfLanguage::Exclamation,
                "вставне слово" => PartOfLanguage::Parenthesis,
                "чоловіче ім`я" => PartOfLanguage::MaleName,
                "жіноче ім`я" => PartOfLanguage::FemaleName,
                _ => return Err(format!("Unknown PartOfLanguage: {}", s)),
            };
            Ok(part)
        }
    }

    pub(crate) struct Record<'w> {
        word: &'w str,
        part: PartOfLanguage,
        is_main_form: bool,
        is_infinitive: bool,
    }

    impl<'w> Record<'w> {
        pub(crate) fn from(s: &'w StringRecord, index: &HeaderIndex) -> Self {
            Record {
                word: s.get(index.word_binary).unwrap(),
                part: PartOfLanguage::from_str(s.get(index.part_of_language).unwrap()).unwrap(),
                is_main_form: s.get(index.is_main_form).unwrap() == "1",
                is_infinitive: s.get(index.is_infinitive).unwrap() == "1",
            }
        }

        fn is_length_valid(&self) -> bool {
            self.word.chars().count() == 5
        }

        fn is_noun_valid(&self) -> bool {
            self.is_main_form &&
                self.word.chars().take(1).all(|c| c.is_lowercase())
        }

        pub(crate) fn is_valid(&self) -> bool {
            let valid = match self.part {
                PartOfLanguage::Noun => self.is_noun_valid(),
                _ => false,
            };
            valid && self.is_length_valid()
        }

        pub(crate) fn valid_word(&self) -> Option<&str> {
            if self.is_valid() {
                Some(self.word)
            } else {
                None
            }
        }
    }

    pub(crate) struct HeaderIndex {
        word_binary: usize,
        part_of_language: usize,
        is_main_form: usize,
        is_infinitive: usize,
        number: usize,
    }

    pub(crate) fn build_index(reader: &mut Reader<File>) -> csv::Result<HeaderIndex> {
        let headers = reader.headers()?;
        let [word_index, is_main_form_index, part_of_language_index, is_infinitive_index, number_index] = indexes(
            headers,
            ["word_binary", "is_main_form", "part_of_language", "is_infinitive", "number"],
        ).unwrap();
        Ok(HeaderIndex {
            word_binary: word_index,
            part_of_language: part_of_language_index,
            is_main_form: is_main_form_index,
            is_infinitive: is_infinitive_index,
            number: number_index,
        })
    }


    fn indexes<const N: usize>(record: &StringRecord, words: [&str; N]) -> Option<[usize; N]> {
        let mut array = [0; N];
        for (index, word) in words.iter().enumerate() {
            if let Some(i) = record.iter().enumerate()
                .find(|(_, v)| v == word)
                .map(|(i, _)| i) {
                array[index] = i;
            } else {
                return None;
            }
        }
        return Some(array);
    }

}