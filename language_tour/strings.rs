// strings.rs

fn main() {
    let question = "how are you?"; // &str type - allocated on the stack or heap
    let person = "Fred".to_string(); // String type - allocated on the heap
                                     // unicode proper namaste spelling!
    let namaste = String::from("नमस्ते"); // String type

    println!("{} {}, {}", namaste, person, question);
}
