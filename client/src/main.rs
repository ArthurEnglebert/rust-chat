use clap::{Arg, App};
use actix_web::client::{Client, SendRequestError};
use std::time::Duration;
use std::collections::LinkedList;
use serde::Serialize;
use serde::Deserialize;
use actix_web::rt::Runtime;
use std::fmt::{Debug, Formatter, Error};
use async_std::sync::Arc;
use std::rc::Rc;
use actix_web::web::block;
use std::io::Read;


fn read_std_in() -> Result<String, std::io::Error> {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.lock().read_to_string(&mut line);
    Ok(String::from(&line[..line.len() - 1]))
}

#[derive(Deserialize, Serialize)]
struct MessageInfo {
    sender: String,
    text: String,
    date: String,
}

#[derive(Deserialize, Serialize)]
struct CanalInfo {
    name: String,
    messages: LinkedList<MessageInfo>
}

impl Debug for MessageInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MessageInfo")
            .field("sender", &self.sender)
            .field("text", &self.text)
            .field("date", &self.date)
            .finish()
    }
}

impl Debug for CanalInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CanalInfo")
            .field("name",&self.name)
            .field("messages", &self.messages)
            .finish()
    }
}

fn work(client: &Client) {
    while let Ok(line) = read_std_in() {
        client.post("http://localhost:8081/canal/my_canal/messages")
            .query(&[("message", &line)])
            .unwrap()
            .send();

        println!("Just sent : {}", &line);
    }
}

async fn poll_msgs(client: &Client, canal_name: &str) -> Result<CanalInfo, Error> {
    return Ok(
        client.get(format!("http://localhost:8081/canal/{}", canal_name))
            .send()
            .await
            .unwrap()
            .json::<CanalInfo>()
            .await
            .unwrap()
    );
}

async fn infinite_poll_msgs(client: &Client, canal_name: &str) {
    loop {
        match poll_msgs(client, canal_name).await {
            Ok(result) => {
                for message in result.messages.iter() {
                    println!("[{}] {} : {}", message.date, message.sender, message.text);
                }
            },
            Err(err) => println!("{}", err)
        }

        async_std::task::sleep(Duration::from_millis(10000)).await;
    }
}

#[actix_web::main]
async fn main() {
    let app = App::new("Rust Chat Client")
        .version("1.0")
        .author("Arthur Englebert <arthur.englebert@skynet.be>")
        .about("just testing things around with rust")
        .arg(
            Arg::with_name("username")
                .index(1)
                .short("u")
                .long("user")
                .value_name("USERNAME")
                .help("Your username credential")
                .required(true)
        ).arg(
        Arg::with_name("password")
            .index(2)
            .short("p")
            .long("pass")
            .value_name("PASSWORD")
            .help("Your password credential")
            .required(true)
    ).get_matches();

    let username = String::from(app.value_of("username").unwrap());
    let password = String::from(app.value_of("password").unwrap());
    let client = Client::builder().basic_auth(&username, Some(&password)).finish();

    // connect stdin
    let stdin_handle = std::thread::spawn(move || {
        let stdin_client = Client::builder().basic_auth(&username, Some(&password)).finish();
        println!("User : {} | Pass : {}", username, password);
        work(&stdin_client)
    });

    // connect poller
    infinite_poll_msgs(&client, "my canal").await

    // https://github.com/actix/examples/blob/master/websockets/chat/client.py
}
