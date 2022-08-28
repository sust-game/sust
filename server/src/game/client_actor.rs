use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use super::messages::{Connect, Disconnect, ServerMessage};
use super::server_actor::{SessionInfo, WsServer};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsClient {
    game_name: Option<String>,
    game_server: Addr<WsServer>,
    last_heartbeat: Instant,
    session_info: SessionInfo,
}

impl WsClient {
    /// Creates a new [`WsClient`], an actor which sits between the user and the game server.
    pub fn _new(session: SessionInfo, game_server: Addr<WsServer>) -> Self {
        Self {
            game_name: None,
            game_server,
            last_heartbeat: Instant::now(),
            session_info: session,
        }
    }

    /// Begins sending heartbeats to the user.
    fn start_heartbeats(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |client, ctx| {
            if Instant::now().duration_since(client.last_heartbeat) > CLIENT_TIMEOUT {
                println!("Heartbeat timed out, disconnecting.");
                client.game_server.do_send(Disconnect {
                    user_id: client.session_info["user_id"].clone(),
                    game_name: client.game_name.clone(),
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"Ping!");
        });
    }
}

impl Actor for WsClient {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeats(ctx);

        let address = ctx.address();

        self.game_server
            .send(Connect {
                address: address.recipient(),
                user_id: self.session_info["user_id"].clone(),
            })
            .into_actor(self)
            .then(|result, _client, ctx| {
                match result {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.game_server.do_send(Disconnect {
            user_id: self.session_info["user_id"].clone(),
            game_name: self.game_name.clone(),
        });
        Running::Stop
    }
}

/// Handle messages from the server
impl Handler<ServerMessage> for WsClient {
    type Result = ();

    fn handle(&mut self, _msg: ServerMessage, _ctx: &mut Self::Context) {
        todo!()
    }
}

/// Handle messages from the user
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(_text) => {
                // "Text" type messages are successful game messages
                // TODO: all of the user-client message handling :)
                todo!()
            }
            ws::Message::Ping(msg) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.last_heartbeat = Instant::now();
            }
            ws::Message::Binary(_) => panic!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
