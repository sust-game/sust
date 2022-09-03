//! Defines the structs associated with users and user management.
use std::collections::HashMap;

use actix::Recipient;

use crate::game::game::GameId;
use crate::game::messages::ServerMessage;

pub type UserId = String; // TODO: Replace this with Uuid when custom session struct is implemented.

#[derive(Default)]
pub struct UserManager {
    active_users: HashMap<UserId, User>,
}

impl UserManager {
    pub fn connect_user(self: &mut Self, user_id: UserId, address: Recipient<ServerMessage>) {
        match self.active_users.get(&user_id) {
            Some(user) => {
                // Some() is returned if the user already exists, meaning a connection must already
                // exist.

                // TODO: Determine what to do if a user is reconnecting before a "full drop" (the
                // user is flushed from the game server) has occurred.
                todo!()
            }
            None => {
                // Lookup the user from the database.
                let username = "".to_owned();

                //
                let user = User::new(username, user_id.clone(), address);
                self.active_users.insert(user_id, user);
                todo!()
            }
        };
    }

    pub fn disconnect_user(self: &mut Self, user_id: &UserId) {
        self.active_users.remove(user_id);
    }
}

struct User {
    name: String,
    id: UserId,
    address: Recipient<ServerMessage>,
    current_game: Option<GameId>,
}

impl User {
    pub fn new(name: String, id: UserId, address: Recipient<ServerMessage>) -> Self {
        Self {
            name,
            id,
            address,
            current_game: None,
        }
    }
}
