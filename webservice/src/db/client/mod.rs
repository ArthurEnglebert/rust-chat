use chat_model::client::client::{Client, Pass};
use crate::db::connection::MySQLPool;
use crate::db::client::connector::ClientConnector;

mod connector;
mod models;

pub struct ClientRepository {
    connector: ClientConnector,
}

impl ClientRepository {
    pub fn new(conn: MySQLPool) -> ClientRepository {
        ClientRepository {
            connector: ClientConnector::new(conn)
        }
    }

    pub fn find_client(&self, name: &str) -> Option<Client> {
        self.connector.find_client(name)
            .map(|cli| {
                Client {
                    uuid: cli.uuid,
                    name: cli.name,
                    pass: Pass {
                        encrypted_pass: cli.pass,
                        salt: cli.salt
                    }
                }
            })
    }
}