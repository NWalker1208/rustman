use std::process::exit;

fn main() {
    match rustman::run("google-10000-english-usa-no-swears-medium.txt") {
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        }
        _ => (),
    }
}
