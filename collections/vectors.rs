// vectors.rs

// Vectors can grow or shrink dynamically, so they will be allocated on the Heap

fn main() {
    // Create a new empty vector of u8 type, using the Vec::new() method
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1); // Add a new element to the vector
    numbers_vec.push(2); // and another one

    // Create a new vector with the vec! macro
    let mut vec_with_macro = vec![1]; // with an initial value of 1
    vec_with_macro.push(2); // Add another element to the vector
                            // Remove the last element from the vector with
    let _ = vec_with_macro.pop(); // The pop method - returns an Option enum,
                                  // so we can use the _ to ignore the result

    let message = if numbers_vec == vec_with_macro {
        "They are equal"
    } else {
        "They look different to me!"
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);
}
