fn main() {
    let number = 6;

    // a bit cluttered, better to use a match construct (see chapter 6)
    if number % 4 == 0 {
        println!("number is divisble by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
