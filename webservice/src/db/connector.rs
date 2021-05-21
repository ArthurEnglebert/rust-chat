use diesel::prelude::*;
use super::models::*;
use super::schema::messages::dsl::*;

pub struct MessageConnector<'a> {
    conn: &'a MysqlConnection
}

impl MessageConnector<'_> {
    pub fn new(conn: &MysqlConnection) -> MessageConnector {
        MessageConnector {
            conn
        }
    }

    pub fn select_last_5_messages(&self) -> Vec<Message> {
        messages.limit(5)
            // .filter(published.eq(true))
            .load(self.conn)
            .expect("Error loading messages")
    }

    pub fn create_message(&self, the_body: & str) -> Message {
        let new_message = NewMessage {
            body: the_body
        };

        diesel::insert_into(messages)
            .values(&new_message)
            .execute(self.conn)
            .expect("Error saving new message");

        messages.order(id.desc()).first(self.conn).expect("Cannot execute select")
    }

    pub fn update_message(&self, the_id: i32, new_body: &str) -> Message {
        // diesel::update(messages.find(id))
        //     .set(body.eq(newBody))
        //     .execute(conn)
        //     .expect(&format!("Unable to find message {}", id));

        diesel::update(messages.find(the_id))
            .set(&MessageForm {
                body: Some(new_body)
            }).execute(self.conn)
            .expect(&format!("Unable to update message {}", the_id));

        messages.find(the_id).first(self.conn).expect(&format!("unable to find message {}", the_id))
    }

    pub fn delete_message(&self, pattern: &str) {
        diesel::delete(messages.filter(body.like(pattern)))
            .execute(self.conn)
            .expect("Error deleting messages");
    }
}