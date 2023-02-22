#[derive(Debug)]
pub struct Game<'a> {
    id: &'a str,
    name: &'a str,
    players: Vec<&'a str>,
    current_player_idx: i32,
}

impl<'a> Game<'a> {
    pub fn begin<T: Clone>(name: &'a str, players: &[&'a str]) -> Game<'a> {
        Game {
            id: name,
            name,
            players: players.to_vec(),
            current_player_idx: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let game = Game::begin::<Vec<&str>>("name", &vec!["fred", "dan"]);
        assert_eq!(game.name, "name");
        assert_eq!(game.id, "id");
    }
}
