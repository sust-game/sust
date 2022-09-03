use std::collections::HashMap;

use actix::prelude::*;

use crate::game::chat::Lobby;
use crate::game::game::GameManager;
use crate::game::messages::*;
use crate::game::user::UserManager;

// TODO: replace SessionInfo with a custom struct later
pub type SessionInfo = HashMap<String, String>;

/// The game server which manages the active games.
#[derive(Default)]
pub struct WsServer {
    user_manager: UserManager,
    game_manager: GameManager,
    main_lobby: Lobby,
}

impl WsServer {
    /// Creates a new [`WsServer`].
    pub fn new() -> Self {
        WsServer::default()
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

        self.user_manager.connect_user(msg.user_id, msg.address);

        // TODO: Probably want to reply with a game list
        todo!()
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        // if let Some(name) = msg.game_name {
        //     if let Some(game) = self.games.get_mut(&name) {
        //         game.players.remove(&msg.user_id);
        //         // Do we want to remove the players immediately on disconnect? Probably makes sense
        //         // to give some leniency and allow for reconnects mid-game (no need to notify other
        //         // players either until a full drop occurs). In this case, we should add two
        //         // Disconnect message types: "quit" and "drop" (intentional vs unintentional). If
        //         // this change is made, it should also be made where the user is removed from the
        //         // user set below.
        //     }
        // }

        self.user_manager.disconnect_user(&msg.user_id);
    }
}

impl Handler<ClientMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, _msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
