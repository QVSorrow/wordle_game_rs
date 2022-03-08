use std::collections::HashSet;
use arrayvec::ArrayVec;
use crate::library::Library;
use crate::word::Word;
use itertools::Itertools;

pub struct Game {
    library: Library,
    word: Word,
    levels: ArrayVec<Word, 6>,
    is_won: bool,
    state: GameState,
}

impl Game {
    pub fn new(library: Library, word: Word) -> Self {
        Game {
            library,
            word,
            levels: ArrayVec::new(),
            is_won: false,
            state: GameState::default(),
        }
    }

    pub fn submit(&mut self, word: Word) {
        if self.is_end_game() {
            return;
        }
        if !self.library.is_valid(&word) {
            return;
        }
        self.levels.push(word);
        self.update_state();
    }

    pub fn state(&self) -> GameState {
        return self.state.clone();
    }

    pub fn is_end_game(&self) -> bool {
        self.is_won || self.levels.len() >= 6
    }

    pub fn is_victory(&self) -> bool {
        self.is_won
    }

    fn update_state(&mut self) {
        if self.levels.contains(&self.word) {
            self.is_won = true;
        }
        self.state = GameState::new(&self.levels, &self.word);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CharInfo {
    Wrong,
    Correct {
        correct_position: bool,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Empty,
    Char {
        c: char,
        info: CharInfo,
    },
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub used_chars: Vec<(char, bool)>,
    pub rows: ArrayVec<ArrayVec<Cell, 5>, 6>,
}

impl GameState {
    fn new(rows: &ArrayVec<Word, 6>, guess: &Word) -> Self {
        GameState {
            used_chars: rows.iter()
                .flat_map(|x| x.iter())
                .unique()
                .map(|&c| (c, guess.contains(&c)))
                .collect::<Vec<_>>(),
            rows: ArrayVec::from([
                cells_from(rows.get(0), guess),
                cells_from(rows.get(1), guess),
                cells_from(rows.get(2), guess),
                cells_from(rows.get(3), guess),
                cells_from(rows.get(4), guess),
                cells_from(rows.get(5), guess),
            ]),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            used_chars: vec![],
            rows: ArrayVec::from([
                ArrayVec::from([Cell::Empty; 5]),
                ArrayVec::from([Cell::Empty; 5]),
                ArrayVec::from([Cell::Empty; 5]),
                ArrayVec::from([Cell::Empty; 5]),
                ArrayVec::from([Cell::Empty; 5]),
                ArrayVec::from([Cell::Empty; 5]),
            ]),
        }
    }
}

fn cells_from(opt: Option<&Word>, guess: &Word) -> ArrayVec<Cell, 5> {
    match opt {
        None => ArrayVec::from([Cell::Empty; 5]),
        Some(word) => word.iter()
            .enumerate()
            .map(|(index, ch)| {
                let char_info = if guess.char_at(index) == ch {
                    CharInfo::Correct { correct_position: true }
                } else if guess.contains(ch) {
                    CharInfo::Correct { correct_position: false }
                } else {
                    CharInfo::Wrong
                };
                Cell::Char {
                    c: *ch,
                    info: char_info,
                }
            })
            .collect::<ArrayVec<_, 5>>(),
    }
}
