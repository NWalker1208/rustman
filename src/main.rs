use std::process::exit;
use std::env;

fn main() {
    let path = env::args().skip(1).next();
    let path = path.unwrap_or(String::from("words.txt"));

    match rustman::run(&path) {
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        }
        _ => (),
    }
}
