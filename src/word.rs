use std::fmt::{Display, Formatter};
use std::slice::Iter;
use arrayvec::ArrayVec;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Word(ArrayVec<char, 5>);

impl Word {
    pub fn char_at(&self, index: usize) -> &char {
        self.0.get(index).unwrap()
    }

    pub fn word(&self) -> String {
        self.0.iter().collect::<String>()
    }

    pub fn contains(&self, c: &char) -> bool {
        self.0.contains(c)
    }

    pub fn iter(&self) -> impl Iterator<Item=&char> {
        self.0.iter()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word())
    }
}

impl From<&str> for Word {
    fn from(s: &str) -> Self {
        let vec = s.chars().take(5).collect::<ArrayVec<_, 5>>();
        Word(vec)
    }
}