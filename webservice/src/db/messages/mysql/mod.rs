mod models;
mod connector;

use chrono::{Local, TimeZone};

use chat_model::message::message::Message;
use connector::MessageConnector;

use crate::db::messages::MessageRepository;
use crate::db::connection::MySQLPool;

pub struct MySQLMessageRepository {
    connector: MessageConnector,
}

impl MySQLMessageRepository {
    pub fn new(conn: MySQLPool) -> MySQLMessageRepository {
        MySQLMessageRepository {
            connector: MessageConnector::new(conn)
        }
    }
}

impl MessageRepository for MySQLMessageRepository {
    fn select_last_5_messages(&self) -> Vec<Message> {
        self.connector.select_last_5_messages().iter()
            .map(|message| {
                Message {
                    client: String::from(&message.client),
                    msg: String::from(&message.body),
                    date: Local.from_local_datetime(&message.date).unwrap()
                }
            }).collect()
    }

    fn save_message(&self, client: &str, msg: &str) {
        self.connector.create_message(client, msg);
    }
}
