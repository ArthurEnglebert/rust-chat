use actix_web_actors::ws;
use crate::websocket::session::{WsChatSession};
use crate::websocket::server;
use actix::{fut, WrapFuture, ActorFuture, ContextFutureSpawner};
use super::{Command};

pub struct ListCommand {}

impl Command for ListCommand {
    fn supports(&self, input: &String, _session: &mut WsChatSession, _ctx: &mut ws::WebsocketContext<WsChatSession>) -> bool {
        input.starts_with("/list ")
    }

    fn invoke(&self, _input: &String, session: &mut WsChatSession, ctx: &mut ws::WebsocketContext<WsChatSession>) {
        // Send ListRooms message to chat server and wait for
        // response
        println!("List rooms");
        session.addr
            .send(server::ListRooms)
            .into_actor(session)
            .then(|res, _, ctx| {
                match res {
                    Ok(rooms) => {
                        for room in rooms {
                            ctx.text(room);
                        }
                    }
                    _ => println!("Something is wrong"),
                }
                fut::ready(())
            })
            .wait(ctx)
        // .wait(ctx) pauses all events in context,
        // so actor wont receive any new messages until it get list
        // of rooms back
    }
}