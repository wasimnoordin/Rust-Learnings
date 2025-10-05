fn main() {
    println!("Hello, world!");
    another_function();
    another_function_2(5);
    print_labeled_measurement(5, 'm');

    // statements and expressions
    let y = {
        let x = 3;
        // remove semi-colon to turn statement (which performs action) into expression (which returns value)
        x + 1 
    };
    println!("The value of y is: {y}");

    // return value to initialize a variable
    let f = five();
    println!("The value of f is: {f}");

    let xp1 = x_plus_one(5);
    println!("The value of xp1 is: {xp1}");
}

fn another_function() {
    println!("Another function");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

// functions with return value
fn five() -> i32 {
    5
}

fn x_plus_one(x: i32) -> i32 {
    x + 1
}
