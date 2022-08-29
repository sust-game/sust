//! Define message types between server and clients
use actix::prelude::*;

use crate::game::server_actor::UserId;

/// Message from user to client actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct UserMessage(pub String);

/// New connection notification
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub address: Recipient<ServerMessage>,
    pub user_id: UserId,
}

/// Connection (intentional) disconnect notification
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: UserId,
    pub game_name: Option<String>,
}

/// Message from client actor to server
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub user_id: UserId,
}

/// Message from server to client actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage {
    // TODO: Flesh this out
}
