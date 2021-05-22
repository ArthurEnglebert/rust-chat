use crate::db::connection::{MySQLPooledConnection, MySQLPool};
use super::models::Client;
use crate::db::clients::models::{NewClient, UpdateClient};
use crate::db::schema::clients::dsl::*;
use diesel::{RunQueryDsl, QueryDsl, TextExpressionMethods};

pub struct ClientConnector {
    conn: MySQLPool
}

impl ClientConnector {
    pub fn new(conn: MySQLPool) -> ClientConnector {
        ClientConnector {
            conn
        }
    }

    fn _conn_handler(&self) -> MySQLPooledConnection {
        self.conn.get().expect("Cannot get connection")
    }

    pub fn create_client(&self, client: Client) -> Client {
        let new_client = NewClient {
            uuid: &client.uuid,
            name: &client.name,
            pass: &client.pass,
            salt: &client.salt,
        };

        diesel::insert_into(clients)
            .values(&new_client)
            .execute(&self._conn_handler())
            .expect("Error saving new message");

        clients.find(&client.uuid)
            .first(&self._conn_handler())
            .expect(&format!("unable to find clients {}", client.uuid))
    }

    pub fn update_client(&self, the_uuid: &str, the_name: &str) -> Client {
        diesel::update(clients.find(the_uuid))
            .set(&UpdateClient {
                name: Some(the_name),
                pass: None,
                salt: None,
            }).execute(&self._conn_handler())
            .expect(&format!("Unable to update clients {}", the_uuid));

        self.find_client(the_uuid).expect(&format!("unable to find clients {}", the_uuid))
    }

    pub fn find_client(&self, the_uuid: &str) -> Option<Client> {
        match clients.find(the_uuid).first(&self._conn_handler()) {
            Ok(client) => Some(client),
            Err(e) => {
                eprintln!("Cannot find clients {} because : {:?}", the_uuid, e);
                None
            }
        }
    }

    pub fn delete_client_by_name(&self, pattern: &str) {
        diesel::delete(clients.filter(name.like(pattern)))
            .execute(&self._conn_handler())
            .expect("Error deleting messages");
    }
}