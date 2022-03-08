use std::fmt::Display;
use std::io::stdin;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use itertools::Itertools;
use owo_colors::OwoColorize;
use crate::file_fetcher::fetcher_from_file;
use crate::game::{Cell, CharInfo, Game, GameState};
use crate::library::Library;
use crate::word::Word;

mod library;
mod game;
mod word;
mod file_fetcher;


fn main() {
    let fetcher = fetcher_from_file(Path::new("words_ua.csv"));
    println!("Fetching library...");
    let library = Library::from_fetcher(fetcher);
    let word = library.get_random().unwrap().clone();
    let mut game = Game::new(library.clone(), word.clone());
    println!("{}", base64::encode(format!("{word}")));

    let mut buffer = String::new();
    let mut stdin = stdin();

    loop {
        let state = game.state();
        print_game_state(&state);

        if game.is_end_game() {
            if game.is_victory() {
                println!("Вітаю з перемогою!");
            } else {
                println!("Нажаль не правильно. Слово - \"{}\". Успіхів у наступний раз!", word.word());
            }
            sleep(Duration::from_secs(3));
            break;
        }

        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let input = buffer.trim();
        if input.chars().count() != 5 {
            println!("Слово має складатися із 5 букв! ({})", input);
            continue;
        }
        let word = Word::from(input);
        if !library.is_valid(&word) {
            println!("Я не знаю слова: {} :(", input);
        }
        game.submit(word);
    }
}

fn print_cell(cell: &Cell) {
    match cell {
        Cell::Empty => print!(" "),
        Cell::Char { c, info } => {
            match info {
                CharInfo::Wrong => print!("{}", c.white()),
                CharInfo::Correct { correct_position } => {
                    if *correct_position {
                        print!("{}", c.green())
                    } else {
                        print!("{}", c.yellow())
                    }
                }
            }
        }
    }
}

fn print_game_state(state: &GameState) {
    for _ in 0..5 {
        println!();
    }

    println!("------------ Гра --------------");
    println!();

    println!("   ---------------------   ");
    for row in state.rows.iter() {
        print!("   ");
        for cell in row.iter() {
            print!("| ");
            print_cell(cell);
            print!(" ");
        }
        println!("|   ");
    }
    println!("   ---------------------   ");

    println!();

    if !state.used_chars.is_empty() {
        print!("Використані букви: ");
        for (ch, is_match) in state.used_chars.iter() {
            if *is_match {
                print!("{} ", ch.green());
            } else {
                print!("{} ", ch.red());
            }
        }
        println!();
    }
    println!();
    println!();
}
