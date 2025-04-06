use std::io;

//guessing game basic input/output
//init mutable variable guess bound to an empty String instance
//read input (read_line) into a reference of guess (&mut guess), ampersand means reference

//comment for test commit mac

fn main() {
    println!("Guess the number");
    println!("Input guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}