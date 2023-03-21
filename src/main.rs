mod TestMain;

use crate::PlayerPosition::CENTER;

#[derive(Debug)]
enum PlayerPosition {
    CENTER
}

struct HockeyPlayer {
    name: String,
    age: u8,
    position: PlayerPosition
}


fn main() {
    let player = HockeyPlayer{
        name: String::from("Bryan Rust"),
        age: 19,
        position: CENTER
    };

    println!("{} is aged: {} and plays: {:?}", player.name, player.age, player.position);
}
