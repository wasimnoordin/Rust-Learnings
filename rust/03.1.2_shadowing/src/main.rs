fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}


fn main() {

    // integer types
    let a: i8 = 10; // with explicit type annotation
    

    // floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // numeric ops
    let sum = 5 + 10;
    let difference = 99.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1
    let remainder = 43 % 5;

    // boolean types
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character types
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

}
