//The prelude is a module that is automatically imported.
//The prelude has common things that are used in Rust programs.
//for example:
// 1. println! macro
// 2. String type

//trait is a way to define shared behavior in an abstract way.

use std::io;  // prelude
use std::cmp::Ordering; 
use rand::Rng; // trait

fn main() {
    println!("Guess number game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    // ^^^^^^^^^^^^^^
    // rand::thread_rng() returns a random number generator instance
    // .gen_range(1..101) is a method call on that instance

    // let is used to create a variable
    // mut is used to make the variable mutable
    // :: is used to call a function from a module
    let mut guess: String = String::new(); 

    // io:Result OK or Err
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"  );

    // Rust allows shadowing
    // Shadowing means that you can declare a new variable with the same name as a previous variable
    // u32 is an unsigned 32-bit integer. Range: 0 to 4,294,967,295.
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    // => is used to return a value
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
