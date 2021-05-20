use actix::{Actor, Context, ActorContext, ActorState, Handler};
use actix::dev::MessageResponse;
use actix::prelude::*;

struct ConsoleReader {
}

impl Actor for ConsoleReader {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "String")]
struct Read();

impl Handler<Read> for ConsoleReader {
    type Result = String;

    fn handle(&mut self, msg: Read, ctx: &mut Context<Self>) -> Self::Result {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
        input
    }
}

struct ConsoleSender {
}

impl Actor for ConsoleSender {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct Send(String);

impl Handler<Send> for ConsoleSender {
    type Result = ();

    fn handle(&mut self, msg: Send, ctx: &mut Context<Self>) -> Self::Result {
        println!("Sending {} ...", msg.0)
    }
}

pub async fn console_work() {
    let reader = ConsoleReader{}.start();
    let sender = ConsoleSender{}.start();

    match reader.send(Read()).await {
        Ok(line) => {
            sender.send(Send(line)).await;
        }
        Err(e) => {
            eprintln!("Encountered error : {:?}", e);
        }
    };
}