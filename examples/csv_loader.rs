

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use csv::{Reader, StringRecord};
use wordle_game::file_fetcher::fetcher_from_file;


fn main() -> Result<(), Box<dyn Error>> {
    let fetcher = fetcher_from_file(Path::new("words_ua.csv"));
    let data = fetcher.get().unwrap();
    println!("size: {}", data.len());
    println!("first 20:");
    for word in data.iter().take(20) {
        println!("  {word}");
    }
    Ok(())
}