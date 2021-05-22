use std::collections::LinkedList;
use crate::db::messages::MessageRepository;
use chat_model::message::message::Message;
use itertools::Itertools;
use chrono::Local;

static mut MESSAGES : LinkedList<Message> = LinkedList::new();

pub struct HeapMessageRepository {
}

impl HeapMessageRepository {
    pub fn new() -> HeapMessageRepository {
        HeapMessageRepository {
        }
    }
}

impl MessageRepository for HeapMessageRepository {
    fn select_last_5_messages(&self) -> Vec<Message> {
        unsafe {
            let msgs = MESSAGES.clone();
            msgs.into_iter()
                .sorted_by(|a, b| Ord::cmp(&b.date, &a.date))
                .take(5)
                .collect()
        }
    }

    fn save_message(&self, client: &str, msg: &str) {
        unsafe {
            MESSAGES.push_back(Message {
                client: client.to_string(),
                msg: msg.to_string(),
                date: Local::now()
            });
        }
    }
}