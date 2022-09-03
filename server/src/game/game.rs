use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::game::chat::Lobby;
use crate::game::user::UserId;

pub type GameId = Uuid;

#[derive(Default)]
pub struct GameManager {
    games: HashMap<GameId, Game>,
}

struct Game {
    name: String,
    // TODO: Choose a better key type (or data structure in general)
    lobbies: HashMap<String, Lobby>,
    users: HashSet<UserId>,
}
