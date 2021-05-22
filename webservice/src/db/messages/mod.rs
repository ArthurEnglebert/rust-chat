use chat_model::message::message::Message;
use crate::db::connection::MySQLPool;

#[cfg(not(feature="heap_db"))]
mod mysql;
#[cfg(feature="heap_db")]
mod heap;

pub trait MessageRepository {
    fn select_last_5_messages(&self) -> Vec<Message>;

    fn save_message(&self, client: &str, msg: &str);
}

pub struct MessageRepositoryFactory {}

impl MessageRepositoryFactory {
    #[cfg(not(feature="heap_db"))]
    pub fn new(conn: Option<MySQLPool>) -> mysql::MySQLMessageRepository {
        mysql::MySQLMessageRepository::new(conn.unwrap())
    }
    #[cfg(feature = "heap_db")]
    pub fn new(conn: Option<MySQLPool>) -> heap::HeapMessageRepository {
        heap::HeapMessageRepository::new()
    }
}