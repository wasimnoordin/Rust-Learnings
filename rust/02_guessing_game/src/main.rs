// use -> bring type you want into scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert String guess to int for cmp, shadowing value prev guess with new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // success -> matches first arm
            Err(_) => continue, // fail (not able to conv to number) -> matches second arm, go to next loop iteration
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit after guessing secret number
            }
        }
    }
}

/*

mut => mutable variable
:: => new is an associated function of type String
stdin() => handle user inout
read_line() => gets input from user, needs to be mutable to append
            => returns Result enum which can be in multiple possible states
& => reference, (default: immutable)
expect() => if enum is Err (and not Ok), causes project to crash

cargo_lock => auto-generated file describing all versions of dependencies when you build for first time
Rng => trait, defines methods that RNGs implement, must be in scope
thread_rng() => a RNG we are using

cmp => compare two values
match => consists of arms (patterns to match against), like switch
trim() => remove whitespace at start and end
parse() => convert from String to another type (i.e. u32 int)

*/
