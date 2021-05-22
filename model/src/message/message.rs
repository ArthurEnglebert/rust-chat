use crate::client::client::Client;
use chrono::{DateTime, Local};
use std::fmt::{Debug, Formatter};
use core::fmt;

pub struct Message {
    pub client: String, // not a Client because we want to be able to delete them while still having message history
    pub msg: String,
    pub date: DateTime<Local>
}

impl Message {
    pub fn client(&self) -> &str {
        &self.client
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn date(self) -> DateTime<Local> {
        self.date
    }

    pub fn new(client: &Client, msg: &str) -> Message {
        Message {
            client: client.name().clone(),
            msg : String::from(msg),
            date: Local::now()
        }
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Message")
            .field("client", &self.client)
            .field("msg", &self.msg)
            .field("date", &self.date)
            .finish()
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        let client = self.client.clone();
        let msg = self.msg.clone();
        let date = self.date.clone();
        Message {
            client,
            msg,
            date
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::client::Client;
    use crate::message::message::Message;
    use std::ops::Add;
    use chrono::{Duration, Local};

    #[test]
    fn message_has_client() {
        let client = Client::new("my_client", "his_pass").unwrap();

        let message = Message::new(&client, "my message");

        assert_eq!(message.client(), "my_client");
    }

    #[test]
    fn message_has_msg() {
        let client = Client::new("my_client", "his_pass").unwrap();

        let message = Message::new(&client, "my message");

        assert_eq!(message.msg(), "my message");
    }

    #[test]
    fn message_has_date() {
        let client = Client::new("my_client", "his_pass").unwrap();

        let message = Message::new(&client, "my message");

        assert!(message.date() + Duration::minutes(2) > Local::now());
    }
}