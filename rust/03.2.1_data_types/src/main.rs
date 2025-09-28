fn main() {

    // ***  SCALAR TYPES *** //

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

    // *** COMPOUND TYPES *** // 

    // tuple types 
    // grouping values with variety of types, fixed size
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching to get vals
    let (x, y, z) = tup; 
    println!("The value of y is: {y}");

    // period (direct access) to get vals
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array types
    // useful when no. elements will not need to change
    let ar = [1, 2, 3, 4, 5];
    let ar: [i32; 5] = [1, 2, 3, 4, 5]; // explicit with type and no. ele
    let arr = [3; 5]; // same as arr = [3, 3, 3, 3, 3]
    let first = ar[0];
    let second = ar[1];
}
