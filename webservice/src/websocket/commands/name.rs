use super::{Command};
use actix_web_actors::ws::WebsocketContext;
use crate::websocket::session::{WsChatSession};

pub struct NameCommand {}

impl Command for NameCommand {

    fn supports(&self, input: &String, _session: &mut WsChatSession, _ctx: &mut WebsocketContext<WsChatSession>) -> bool {
        input.starts_with("/name ")
    }

    fn invoke(&self, input: &String, session: &mut WsChatSession, ctx: &mut WebsocketContext<WsChatSession>) {
        let v: Vec<&str> = input.splitn(2, ' ').collect();
        if v.len() == 2 {
            session.name = Some(v[1].to_owned());
        } else {
            ctx.text("!!! name is required");
        }
    }
}