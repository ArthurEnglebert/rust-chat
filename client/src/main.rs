mod console;

use clap::{Arg, App};

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use actix::{Actor, Arbiter};
use crate::console::console_work;


#[derive(Debug)]
struct Client {
}

impl Future for Client {
    type Output = io::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("main loop tick");
        Poll::Pending
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app = App::new("Rust Chat Client")
        .version("1.0")
        .author("Arthur Englebert <arthur.englebert@skynet.be>")
        .about("just testing things around with rust");

    let client = Client{};

    Arbiter::current().spawn(console_work());

    client.await
}
