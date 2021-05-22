mod connector;
mod models;

use chat_model::canal::canal::Canal;
use crate::db::connection::MySQLPool;
use crate::db::canals::connector::CanalConnector;

pub trait CanalRepository {
    fn new_canal(&self, canal_name: String) -> Canal;
    fn save_canal(&self, canal: Canal);
}

pub struct MysqlCanalRepository {
    connector: CanalConnector,
}

impl MysqlCanalRepository {
    pub fn new(conn: MySQLPool) -> MysqlCanalRepository {
        MysqlCanalRepository {
            connector: CanalConnector::new(conn)
        }
    }
}

impl CanalRepository for MysqlCanalRepository {
    fn new_canal(&self, canal_name: String) -> Canal {
        self.connector.get_canal(&canal_name)
            .map(|canal| {
                Canal::new(&canal.name)
            }).expect(&format!("cannot find canal {}", canal_name))
    }

    fn save_canal(&self, _canal: Canal) {
        // todo link between canal & messages via diesel
        // todo add only new messages of canal
    }
}