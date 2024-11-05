//The prelude is a module that is automatically imported.
//The prelude has common things that are used in Rust programs.
//for example:
// 1. println! macro
// 2. String type

use std::io; 


fn main() {
    println!("Guess number game!");

    // let foo = 1;
    // let bar = foo; //immutable variable
    // foo = 2; //error: cannot assign twice to immutable variable

    // let is used to create a variable
    // mut is used to make the variable mutable
    // :: is used to call a function from a module
    // :: is like static method in Java
    let mut guess: String = String::new(); 

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"  );

    println!("You guessed: {}", guess);
}
