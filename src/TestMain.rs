#[cfg(test)]
mod tests {
    use crate::{HockeyPlayer, PlayerPosition};
    use super::*;

    #[test]
    fn test_hockey_player() {
        let player = HockeyPlayer{
            name: String::from("Bryan Rust"),
            age: 19,
            position: PlayerPosition::CENTER
        };

        assert_eq!(player.name, "Bryan Rust");
        assert_eq!(player.age, 19);
        assert_eq!(player.position, PlayerPosition::CENTER);
    }
}