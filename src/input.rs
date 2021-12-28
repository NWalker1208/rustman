use std::io;
use std::io::{stdout, Write};

/// Gets a letter guess from the user.
/// Forces them to choose a letter not present in the list of guessed characters.
pub fn get_guess(already_guessed: &[char]) -> char {
    loop {
        print!("Guess a letter: ");
        stdout().flush().expect("Could not flush stdout");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        match guess.to_lowercase().trim().chars().next() {
            Some(c) => match c {
                'a'..='z' | 'A'..='Z' => {
                    if already_guessed.contains(&c) {
                        println!("You already guessed that letter!");
                    } else {
                        break c;
                    }
                }
                _ => println!("Please enter a letter!"),
            }
            None => println!("Please enter a letter!"),
        }
    }
}
