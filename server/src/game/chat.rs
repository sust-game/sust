use std::collections::HashSet;

use uuid::Uuid;

use crate::game::user::UserId;

pub type LobbyId = Uuid;

/// An object representing a text chat lobby.
pub struct Lobby {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    id: Uuid,
    #[allow(dead_code)]
    /// The set of Users which receive the messages sent to this lobby.
    users: HashSet<UserId>,
    #[allow(dead_code)]
    /// The subset of `users` which cannot send messages to this lobby.
    muted: HashSet<UserId>,
}

impl Default for Lobby {
    fn default() -> Self {
        Self {
            name: "main".to_owned(),
            id: Uuid::nil(),
            users: HashSet::new(),
            muted: HashSet::new(),
        }
    }
}
