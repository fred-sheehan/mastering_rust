// arrays.rs

// Arrays are a collection of elements of the same type.
// They are fixed in size and cannot grow or shrink.

fn main() {
    let numbers: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let floats = [0.73_f64, 0.2, 0.6];

    // Accessing elements of the array by index
    // Indexing starts from 0
    println!("Number: {}", numbers[5]);
    println!("Float: {}", floats[1]);
}
