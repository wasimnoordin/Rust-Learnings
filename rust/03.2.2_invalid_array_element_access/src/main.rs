use std::io;

fn main() {

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number.");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // if index > 4, Rust will panic at runtime as compiler won't know what values user will enter when code is run later
    // example of checks for invalid memory being accessed
}
