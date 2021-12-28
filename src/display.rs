use crate::state::State;

/// Displays the current game state.
pub fn display_state(state: &State) {
    draw_hangman(6 - state.wrong_guesses());
    draw_secret_word(state.secret_word(), state.guesses());
    draw_guessed_letters(state.guesses());
}

/// Draws the hangman. The user can have up to 6 wrong guesses.
/// (head, body, left arm, right arm, left leg, right leg)
fn draw_hangman(guesses_remaining: u8) {
    println!("\
---------
|       |
|       {}
|      {}{}{}
|      {} {}
|
-----------------",
             if guesses_remaining < 6 { "O" } else { " " },
             if guesses_remaining < 4 { "/" } else { " " },
             if guesses_remaining < 5 { "|" } else { " " },
             if guesses_remaining < 3 { "\\" } else { " " },
             if guesses_remaining < 2 { "/" } else { " " },
             if guesses_remaining < 1 { "\\" } else { " " });
}

/// Draws the secret word, with only the guessed letters revealed.
fn draw_secret_word(secret_word: &str, guessed_letters: &[char]) {
    print!("Word: ");
    for c in secret_word.chars() {
        if guessed_letters.contains(&c) {
            print!("{}", c);
        } else {
            print!("_");
        }
    }
    print!("\n");
}

/// Draws the letters that have been guessed.
fn draw_guessed_letters(guessed_letters: &[char]) {
    print!("Guessed letters: ");
    for (i, c) in guessed_letters.iter().enumerate() {
        if i > 0 { print!(", ") }
        print!("{}", c);
    }
    print!("\n");
}
