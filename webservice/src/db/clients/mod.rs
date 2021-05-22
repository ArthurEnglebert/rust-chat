mod connector;
mod models;

use chat_model::client::client::{Client, Pass};
use crate::db::connection::MySQLPool;
use crate::db::clients::connector::ClientConnector;

pub trait ClientRepository {
    fn find_client(&self, name: &str) -> Option<Client>;
}

pub struct MysqlClientRepository {
    connector: ClientConnector,
}

impl MysqlClientRepository {
    pub fn new(conn: MySQLPool) -> MysqlClientRepository {
        MysqlClientRepository {
            connector: ClientConnector::new(conn)
        }
    }
}

impl ClientRepository for MysqlClientRepository {
    fn find_client(&self, name: &str) -> Option<Client> {
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