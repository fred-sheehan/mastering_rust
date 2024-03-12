// tuples.rs

/*
A tuple is a collection of values of different types. Tuples are constructed using parentheses, and each tuple itself is a value with a type signature. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
*/

fn main() {
    let num_and_str: (u8, &str) = (40, "Have a good day!");
    println!("{:?}", num_and_str); // Printing the tuple 'as is'

    let (num, string) = num_and_str; // Destructuring the tuple
                                     // and then printing the seperate values
    println!("From tuple: Number: {}, String: {}", num, string);
}
