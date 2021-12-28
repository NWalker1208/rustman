use state::State;
use state::Victory;
use words::WordList;

/// Word list creation and selection.
pub mod words;

/// Game state.
pub mod state;

/// User input.
pub mod input;

/// Command line output of game state.
pub mod display;

/// Runs a game of hangman using words from the given file.
pub fn run(word_file: &str) -> Result<(), &'static str> {
    let words = WordList::new(word_file).map_err(|_| "Failed to load word list")?;
    let mut state = State::new(words.random());

    while matches!(state.victory(), Victory::None) {
        display::display_state(&state);
        let guess = input::get_guess(state.guesses());
        if state.guess(guess) {
            println!("Correct!\n");
        } else {
            println!("Incorrect!\n");
        }
    }

    display::display_state(&state);
    match state.victory() {
        Victory::Win => println!("You win!"),
        _ => println!("You lose! The word was {}.", state.secret_word())
    }

    Ok(())
}
