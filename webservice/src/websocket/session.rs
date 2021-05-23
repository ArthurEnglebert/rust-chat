use std::time::{Duration, Instant};

use actix::*;
use actix_web_actors::ws;

use super::server;
use crate::websocket::commands::CommandRegistry;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of clients response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsChatSession {
    /// unique session id
    pub id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// joined room
    pub room: String,
    /// peer name
    pub name: Option<String>,
    /// Chat server
    pub addr: Addr<server::ChatServer>,
    /// Command Registry,
    command_reg: CommandRegistry,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                self.analyze_input_and_dispatch(text, ctx);
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
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

impl WsChatSession {
    pub fn new(addr: Addr<server::ChatServer>) -> WsChatSession {
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr,
            command_reg: CommandRegistry::new(),
        }
    }

    /// helper method that sends ping to clients every second.
    ///
    /// also this method checks heartbeats from clients
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check clients heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn analyze_input_and_dispatch(&mut self, input: String, ctx: &mut ws::WebsocketContext<WsChatSession>) {
        let m = input.trim();
        // we check for /sss type of messages
        if m.starts_with('/') {
            if ! CommandRegistry::new().invoke(&input, self, ctx) {
                ctx.text(format!("!!! unknown command: {:?}", m))
            }
        } else {
            let client_name = if let Some(ref name) = self.name {
                String::from(name)
            } else {
                String::from("unknown")
            };

            // send message to chat server
            self.addr.do_send(server::ClientMessage {
                id: self.id,
                client_name,
                msg: m.to_owned(),
                room: self.room.clone(),
            })
        }
    }
}