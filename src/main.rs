use std::process::exit;

fn main() {
    match rustman::run("words.txt") {
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        },
        _ => (),
    }
}
