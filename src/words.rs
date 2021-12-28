use std::fs;
use std::io;

use rand;
use rand::Rng;

/// Stores a list of words.
pub struct WordList {
    words: Vec<String>,
}

impl WordList {
    /// Creates a new word list from a file.
    /// Each line of the file will be considered a word.
    pub fn new(filename: &str) -> Result<WordList, io::Error> {
        let mut words = Vec::new();
        let contents = fs::read_to_string(filename)?;

        for line in contents.lines() {
            words.push(line.to_string());
        }

        Ok(WordList { words })
    }

    /// Returns a random word from the list.
    pub fn random(&self) -> &str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.words.len());
        &self.words[index]
    }
}
