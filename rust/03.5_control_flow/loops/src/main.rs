fn main() {

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // we want result * 2 - arbitrary
        }
    };
    println!("The result is {result}");
}
