use std::io;
use game::Game;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    println!("Please input your name.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    trim_newline(&mut guess);

    let mut v = vec!["fred"];

    v.push(&guess);

    let game = Game::begin::<Vec<&str>>("Nevsky", &v);

    println!("{:?}", game);
}