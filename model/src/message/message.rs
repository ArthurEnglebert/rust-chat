use crate::client::client::Client;
use chrono::{DateTime, Local};

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