use std::collections::{HashMap, HashSet};

use actix::prelude::*;

use crate::game::messages::*;

// TODO: replace SessionInfo with a custom struct later
pub type SessionInfo = HashMap<String, String>;
pub type UserId = String; // TODO: Replace this with Uuid when custom struct is implemented

// TODO: Move this to a dedicated file, probably
struct Game {
    players: HashSet<UserId>,
    // TODO: Game state goes here
}

/// The game server which manages the active games.
pub struct WsServer {
    users: HashMap<UserId, Recipient<ServerMessage>>,
    games: HashMap<String, Game>,
}

impl WsServer {
    /// Creates a new [`WsServer`].
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            games: HashMap::new(),
        }
    }
}

impl Actor for WsServer {
    /// [`WsServer`] is as generic as possible, accepting any type of context.
    type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Received a new connection request");

        match self.users.insert(msg.user_id, msg.address) {
            None => (),
            Some(_) => {
                // Some() is returned if the user already exists, meaning a connection must already
                // exist.

                // TODO: Determine what to do if a user is reconnecting before a "full drop" (the
                // user is flushed from the game server) has occurred.
                todo!()
            }
        }

        // TODO: Probably want to reply with a game list
        todo!()
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(name) = msg.game_name {
            if let Some(game) = self.games.get_mut(&name) {
                game.players.remove(&msg.user_id);
                // Do we want to remove the players immediately on disconnect? Probably makes sense
                // to give some leniency and allow for reconnects mid-game (no need to notify other
                // players either until a full drop occurs). In this case, we should add two
                // Disconnect message types: "quit" and "drop" (intentional vs unintentional). If
                // this change is made, it should also be made where the user is removed from the
                // user set below.
            }
        }

        self.users.remove(&msg.user_id);
    }
}

impl Handler<ClientMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, _msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
