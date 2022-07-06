// by default the implicit imports of a module are called a `prelude`
// a `prelude` is the list of things that Rust automatically imports into every Rust program. It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.
// this part imports the standard io module
use rand::Rng;
use std::cmp::Ordering;
use std::io; // this Rng import is called a 'trait'

// cargo doc --open will open documentation on the browser
fn main() {
    println!("Guess the number!");

    // for range definitions (1..100) -> exclusive (1..=100) -> inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        // let apples = 5; // immutable
        // let mut bananas = 5; // mutable
        // <Type>::fn() <- this piece of code indicates that fn is owned by the type and not the instance
        let mut guess = String::new();

        io::stdin() // creates a new Stdin instance
            .read_line(&mut guess) // appends the content to the string, that's why it needs to be mutable, '&' represents a pass by reference
            .expect("Failed to read line"); // this is the message displayed in case of an error when we don't want to handle the error

        // this is called a shadow, we don't need to create a new variable with a different
        // parse creates a number and the correct number is provided by the type `u32`
        let guess: u32 = match guess.trim().parse() {
            // we can use match expression to handle the different values (this is called an enum)
            Ok(num) => num,
            Err(_) => {
                println!("The input is not a number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // this is called an arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break; // breaks the loop
            }
        }
    }
    // println!("You guessed: {guess}"); // this can also be written as println!("You guessed: {}", guess);
}
