use std::collections::{HashMap, HashSet};

use crate::game::chat::Lobby;
use crate::game::user::UserId;

pub type GameId = String;

#[derive(Default)]
pub struct GameManager {
    // The game's name is the key; at most one game should be able to have a given name.
    games: HashMap<GameId, Game>,
}

struct Game {
    name: GameId,
    // TODO: Choose a better key type (or data structure in general) for lobbies.
    lobbies: HashMap<String, Lobby>,
    users: HashSet<UserId>,
}
