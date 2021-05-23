use super::{Command};
use actix_web_actors::ws::WebsocketContext;
use crate::websocket::session::{WsChatSession};
use crate::websocket::server;

pub struct JoinCommand {}

impl Command for JoinCommand {

    fn supports(&self, input: &String, _session: &mut WsChatSession, _ctx: &mut WebsocketContext<WsChatSession>) -> bool {
        input.starts_with("/join ")
    }

    fn invoke(&self, input: &String, session: &mut WsChatSession, ctx: &mut WebsocketContext<WsChatSession>) {
        let v: Vec<&str> = input.splitn(2, ' ').collect();
        if v.len() == 2 {
            session.room = v[1].to_owned();
            session.addr.do_send(server::Join {
                id: session.id,
                name: session.room.clone(),
            });

            ctx.text("joined");
        } else {
            ctx.text("!!! room name is required");
        }
    }
}