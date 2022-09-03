use std::collections::{HashMap, HashSet};

use crate::game::chat::Lobby;
use crate::game::user::UserId;

pub type GameId = String;

#[derive(Default)]
pub struct GameManager {
    #[allow(dead_code)]
    // The game's name is the key; at most one game should be able to have a given name.
    games: HashMap<GameId, Game>,
}

struct Game {
    #[allow(dead_code)]
    name: GameId,
    #[allow(dead_code)]
    // TODO: Choose a better key type (or data structure in general) for lobbies.
    lobbies: HashMap<String, Lobby>,
    #[allow(dead_code)]
    users: HashSet<UserId>,
}
