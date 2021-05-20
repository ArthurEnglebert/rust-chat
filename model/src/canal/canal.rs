use std::collections::LinkedList;
use crate::message::message::Message;

pub struct Canal {
    pub name: String,
    pub messages: LinkedList<Message>,
}

impl Canal {
    pub fn new(name: &str) -> Canal {
        Canal {
            name: String::from(name),
            messages: LinkedList::new()
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push_back(message);
    }
}

#[cfg(test)]
mod tests {
    use crate::canal::canal::Canal;
    use crate::message::message::Message;
    use crate::client::client::Client;

    #[test]
    fn canal_has_name() {
        let canal_name = "test-canal-name";

        let canal = Canal::new(canal_name);

        assert_eq!(canal.name, canal_name);
    }

    #[test]
    fn canal_has_no_message() {
        let canal_name = "test-canal-name";

        let canal = Canal::new(canal_name);

        assert_eq!(canal.messages.len(), 0);
    }

    #[test]
    fn canal_can_have_new_messages() {
        let canal_name = "test-canal-name";
        let mut canal = Canal::new(canal_name);

        let client = Client::new("my_client", "his_pass").unwrap();

        canal.add_message(Message::new(&client, "my first message"));
        canal.add_message(Message::new(&client, "my second message"));
        canal.add_message(Message::new(&client, "my third message"));

        assert_eq!(canal.messages.len(), 3);
        let message_data = canal.messages.iter()
            .map(|msg| msg.msg())
            .collect::<Vec<&str>>();
        assert_eq!(message_data, vec!["my first message", "my second message", "my third message"]);
    }
}