use std::collections::HashSet;

use uuid::Uuid;

use crate::game::user::UserId;

/// An object representing a text chat lobby.
pub struct Lobby {
    name: String,
    id: Uuid,
    users: HashSet<UserId>,
}

impl Default for Lobby {
    fn default() -> Self {
        Self {
            name: "main".to_owned(),
            id: Uuid::nil(),
            users: HashSet::new(),
        }
    }
}
