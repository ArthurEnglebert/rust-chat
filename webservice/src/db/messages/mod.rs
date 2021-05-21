mod models;
mod connector;

use connector::MessageConnector;
use chat_model::message::message::Message;
use chrono::{Local, TimeZone, DateTime};
use crate::db::connection::MySQLPool;

pub struct MessageRepository {
    connector: MessageConnector,
}

impl MessageRepository {
    pub fn new(conn: MySQLPool) -> MessageRepository {
        MessageRepository {
            connector: MessageConnector::new(conn)
        }
    }

    pub fn select_last_5_messages(&self) -> Vec<Message> {
        self.connector.select_last_5_messages().iter()
            .map(|message| {
                Message {
                    client: String::from(&message.client),
                    msg: String::from(&message.body),
                    date: Local.from_local_datetime(&message.date).unwrap()
                }
            }).collect()
    }

    pub fn save_message(&self, client: &str, msg: &str) {
        self.connector.create_message(client, msg);
    }
}