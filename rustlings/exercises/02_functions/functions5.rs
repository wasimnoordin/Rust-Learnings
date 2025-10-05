// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num // should be expression so removed semi-colon
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
