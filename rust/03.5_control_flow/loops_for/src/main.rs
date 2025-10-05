fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // better to use a for loop as specifying an index with a while loop, without changing the while condition will result in panic when the code runs (index out of bound, etc.)
}
