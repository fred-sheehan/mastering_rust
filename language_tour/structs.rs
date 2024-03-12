// structs.rs

/* The size of a struct is the sum of the sizes of its fields, along with any padding required to ensure that the fields are properly aligned.
The alignment of a struct is determined by the alignment of the field with the largest alignment requirement.
*/

struct Player {
    // a C-like type struct and could include many more fields
    name: String,
    age: u8,
    iq: u8,
    friends: u8,
    score: u16,
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += 120;
    println!("Updated player stats:");
    println!("Name: {}", player.name);
    println!("Age: {}", player.age);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friends);
    println!("Score: {}", player.score);
}

fn main() {
    let name = "Fred".to_string();
    let player = Player {
        name,
        iq: 143,
        age: 55,
        friends: 5,
        score: 1236,
    };
    bump_player_score(player, 120);
}
