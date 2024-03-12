// for_loops.rs

fn main() {
    // won't include 10 in the range
    println!("Exclusive ranges");
    for i in 0..10 {
        // prints 0 to 9
        println!("i: {}", i);
    }
    println!(); // prints a new line

    // will include 10 in the range
    println!("Inclusive ranges");
    for i in 0..=10 {
        // prints 0 to 10
        println!("i: {}", i);
    }
}
