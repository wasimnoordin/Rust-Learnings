fn main() {
    
    // loop labels to disambiguate multiple loops
    // - use break and apply with loop label to apply to that loop, not the innermost (default)

    let mut count = 0;
    'counting_up: loop { // outer loop counts up from 0 -> 2
        println!("count = {count}");
        let mut remaining = 10;

        loop { // inner loop counts down from 10 -> 9
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
