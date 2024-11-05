//The prelude is a module that is automatically imported.
//The prelude has common things that are used in Rust programs.
//for example:
// 1. println! macro
// 2. String type

use std::io;  // prelude
use rand::Rng; // trait

fn main() {
    println!("Guess number game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    // let is used to create a variable
    // mut is used to make the variable mutable
    // :: is used to call a function from a module
    let mut guess: String = String::new(); 

    // io:Result OK or Err
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"  );

    println!("You guessed: {}", guess);
}
