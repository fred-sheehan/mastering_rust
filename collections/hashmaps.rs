// hashmaps.rs

/*
hashmaps are used to store key-value pairs. They are similar to vectors,
but instead of using an index to access the data, you use a key.
keys and values can be any type, ALL keys or ALL values must be the same types.
*/

// import the HashMap type from the collections module
use std::collections::HashMap;

fn main() {
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("banana", 2);
    fruits.insert("orange", 5);
    fruits.insert("mango", 4);
    fruits.insert("grapes", 1);
    fruits.insert("pineapple", 6);
    fruits.insert("avocado", 7);
    for (key, value) in &fruits {
        // use & to borrow the reference to the key and value
        println!("I bought {} {}", value, key);
    }
    // could have also used key() and value() methods to iterate over map
    // for either keys or values alone

    fruits.remove("banana");
    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 6);
    println!("\nI now have {} avocados", fruits["avocado"]);
}
