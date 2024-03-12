// struct_methods.rs

struct Player {
    // creating our own struct type
    name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    // implementation block for all the methods of Player struct
    fn with_name(name: &str) -> Player {
        // an associated method
        Player {
            name: name.to_string(),
            iq: 143,
            friends: 8,
        }
    }

    fn get_friends(&self) -> u8 {
        // instance method, can only be called on an instance of the struct
        // this instance could also be referred to as a getter method
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        // another instance method
        // could also be referred to as a setter method
        self.friends = count;
    }
}

fn main() {
    let mut player = Player::with_name("John"); // calling the associated method
    player.set_friends(12);
    println!(
        "{} has an IQ of: {} and his friends count is: {}",
        player.name,
        player.iq,
        player.get_friends()
    );
    // another way to call the instance methods
    let _ = Player::get_friends(&player);
}
