/// Stores the current state of the game.
pub struct State {
    secret_word: String,
    guesses: Vec<char>,
    wrong_guesses: u8,
}

/// Denotes whether the game has ended, and if the user has won or lost.
pub enum Victory {
    Win,
    Lose,
    None,
}

impl State {
    /// Creates a new game state.
    pub fn new(secret_word: &str) -> State {
        State {
            secret_word: secret_word.to_lowercase(),
            guesses: Vec::new(),
            wrong_guesses: 0,
        }
    }

    /// Returns the list of guesses.
    pub fn guesses(&self) -> &[char] {
        &self.guesses
    }

    /// Checks if a letter has been guessed.
    pub fn has_guessed(&self, letter: char) -> bool {
        self.guesses.contains(&letter)
    }

    /// Returns the number of misses.
    pub fn wrong_guesses(&self) -> u8 {
        self.wrong_guesses
    }

    /// Returns the victory state.
    pub fn victory(&self) -> Victory {
        if self.wrong_guesses() >= 6 {
            Victory::Lose
        } else {
            for letter in self.secret_word.chars() {
                if !self.guesses.contains(&letter) {
                    return Victory::None;
                }
            }
            Victory::Win
        }
    }

    /// Returns the secret word.
    pub fn secret_word(&self) -> &str {
        &self.secret_word
    }

    /// Makes a guess and returns true if the letter is in the word and has not been guessed.
    /// Otherwise, returns false.
    pub fn guess(&mut self, c: char) -> bool {
        if self.has_guessed(c) {
            false
        } else {
            self.guesses.push(c);
            if self.secret_word.contains(c) {
                true
            } else {
                self.wrong_guesses += 1;
                false
            }
        }
    }
}
