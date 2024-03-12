// tuple_struct.rs

// tuple structs are OK for small, simple structs, say max 4 or 5 fields

struct Color(u8, u8, u8);

fn main() {
    let white = Color(255, 255, 255);

    // Accessing the tuple struct by index
    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("Red Value: {}", red);
    println!("Green Value: {}", green);
    println!("Blue Value: {}\n", blue);

    let orange = Color(255, 165, 0);

    // Accessing the tuple struct by destructuring
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (Orange)", r, g, b);

    // ignoring fields using _
    let Color(r, _, b) = orange;
}
