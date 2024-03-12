// greet.rs

use std::env;

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(name) => println!("Hi there {}", name),
        None => println!("You didn't input name!"),
    };
}
