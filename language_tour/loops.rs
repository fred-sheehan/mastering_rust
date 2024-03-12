// loops.rs

fn main() {
    let mut x = 1024;
    loop {
        if x < 0 {
            // if x is less than 0, break the loop
            break;
        }
        println!("{} more runs to go", x);
        x -= 1;
    }
}
