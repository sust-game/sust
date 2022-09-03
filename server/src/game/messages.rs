//! Messages that are sent between the server and clients.

use actix::prelude::*;

use crate::game::chat::LobbyId;
use crate::game::user::UserId;

/// A message sent from a user to a client actor.
#[derive(Message)]
#[rtype(result = "()")]
pub struct UserMessage(pub String);

/// A message sent from a client actor to the server when a user has established a connection.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub address: Recipient<ServerMessage>,
    pub user_id: UserId,
}

/// A message sent from a client actor to the server when a user has intentionally disconnected.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: UserId,
    pub game_name: Option<String>,
}

/// A message sent from a client actor to the server.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub user_id: UserId,
    pub action: UserAction,
}

/// A message sent from the server to a client actor.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage {
    // TODO: Flesh this out
}

pub enum UserAction {
    #[allow(dead_code)]
    ChatMessage { lobby_id: LobbyId, text: String },
}
