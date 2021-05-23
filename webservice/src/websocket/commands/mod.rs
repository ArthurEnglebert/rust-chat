mod list;
mod join;
mod name;

use actix_web_actors::ws;
use crate::websocket::session::{WsChatSession};

pub trait Command {
    fn supports(&self, input: &String, session: &mut WsChatSession, ctx: &mut ws::WebsocketContext<WsChatSession>) -> bool;
    fn invoke(&self, input: &String, session: &mut WsChatSession,  ctx: &mut ws::WebsocketContext<WsChatSession>);
}

pub struct CommandRegistry {
    commands: Vec<Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> CommandRegistry {
        let mut commands : Vec<Box<dyn Command>> = Vec::new();

        commands.push(Box::new(list::ListCommand{}));
        commands.push(Box::new(join::JoinCommand{}));
        commands.push(Box::new(name::NameCommand{}));

        CommandRegistry {
            commands
        }
    }

    pub fn invoke(&self, input: &String, session: &mut WsChatSession, ctx: &mut ws::WebsocketContext<WsChatSession>) -> bool {
        let mut handled = false;
        for cmd in self.commands.iter() {
            if cmd.supports(input, session, ctx) {
                cmd.invoke(input, session, ctx);
                handled = true;
            }
        }
        handled
    }
}