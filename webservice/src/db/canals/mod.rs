mod connector;
mod models;

use chat_model::canal::canal::Canal;
use crate::db::connection::MySQLPool;
use crate::db::canals::connector::CanalConnector;

pub struct CanalRepository {
    connector: CanalConnector,
}

impl CanalRepository {
    pub fn new(conn: MySQLPool) -> CanalRepository {
        CanalRepository {
            connector: CanalConnector::new(conn)
        }
    }

    pub fn new_canal(&self, canal_name: String) -> Canal {
        self.connector.get_canal(&canal_name)
            .map(|canal| {
                Canal::new(&canal.name)
            }).expect(&format!("cannot find canal {}", canal_name))
    }

    pub fn save_canal(&self, canal: Canal) {
        // todo link between canal & messages via diesel
        // todo add only new messages of canal
    }
}