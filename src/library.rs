use std::collections::HashSet;
use std::error::Error;
use std::rc::Rc;
use crate::word::Word;

use rand::seq::IteratorRandom;

pub struct Library {
    inner: Rc<LibraryInner>,
}

impl Library {
    pub fn from_fetcher(fetcher: Fetcher) -> Self {
        let fetched = fetcher.get().expect("Cannot find words :/");
        println!("Кількість слів у бібліотеці: {}", fetched.len());
        Library {
            inner: Rc::new(LibraryInner {
                words: fetched,
            }),
        }
    }

    pub fn get_random(&self) -> Option<Word> {
        self.inner.words.iter().choose(&mut rand::thread_rng()).cloned()
    }

    pub fn is_valid(&self, word: &Word) -> bool {
        self.inner.words.contains(word)
    }
}

impl Clone for Library {
    fn clone(&self) -> Self {
        Library {
            inner: self.inner.clone(),
        }
    }
}

struct LibraryInner {
    words: HashSet<Word>,

}

pub struct Fetcher {
    pub fetch: Box<dyn FnOnce() -> Result<HashSet<Word>, Box<dyn Error>>>,
}

impl Fetcher {
    pub fn get(self) -> Option<HashSet<Word>> {
        (self.fetch)().ok()
    }
}