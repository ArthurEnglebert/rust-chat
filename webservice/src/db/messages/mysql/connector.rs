use diesel::prelude::*;
use crate::db::messages::mysql::models::*;
use crate::db::schema::messages::dsl::*;
use chrono::{Local};
use crate::db::connection::{MySQLPool, MySQLPooledConnection};

pub struct MessageConnector {
    conn: MySQLPool
}

impl MessageConnector {
    pub fn new(conn: MySQLPool) -> MessageConnector {
        MessageConnector {
            conn
        }
    }

    fn _conn_handler(&self) -> MySQLPooledConnection {
        self.conn.get().expect("Cannot get connection")
    }

    pub fn select_last_5_messages(&self) -> Vec<Message> {
        messages.limit(5)
            .load(&self._conn_handler())
            .expect("Error loading messages")
    }

    pub fn create_message(&self, the_client: &str, the_body: & str) -> Message {
        let new_message = NewMessage {
            client: the_client,
            body: the_body,
            date: Local::now().naive_local()
        };

        diesel::insert_into(messages)
            .values(&new_message)
            .execute(&self._conn_handler())
            .expect("Error saving new message");

        messages.order(id.desc()).first(&self._conn_handler()).expect("Cannot execute select")
    }

    pub fn update_message(&self, the_id: i32, new_body: &str) -> Message {

        diesel::update(messages.find(the_id))
            .set(&UpdateMessage {
                body: Some(new_body)
            }).execute(&self._conn_handler())
            .expect(&format!("Unable to update message {}", the_id));

        messages.find(the_id).first(&self._conn_handler()).expect(&format!("unable to find message {}", the_id))
    }

    pub fn delete_message(&self, pattern: &str) {
        diesel::delete(messages.filter(body.like(pattern)))
            .execute(&self._conn_handler())
            .expect("Error deleting messages");
    }
}