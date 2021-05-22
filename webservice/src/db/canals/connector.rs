use crate::db::connection::{MySQLPooledConnection, MySQLPool};
use super::models::*;
use crate::db::schema::canals::dsl::*;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, TextExpressionMethods};

pub struct CanalConnector {
    conn: MySQLPool
}

impl CanalConnector {
    pub fn new(conn: MySQLPool) -> CanalConnector {
        CanalConnector {
            conn
        }
    }

    fn _conn_handler(&self) -> MySQLPooledConnection {
        self.conn.get().expect("Cannot get connection")
    }

    pub fn create_canal(&self, canal_name: &str) -> Canal {
        let new_canal = NewCanal {
            name: canal_name,
        };

        diesel::insert_into(canals)
            .values(&new_canal)
            .execute(&self._conn_handler())
            .expect("Error saving new canal");

        canals.order(id.desc()).first(&self._conn_handler()).expect("Cannot execute select")
    }

    pub fn get_canal(&self, canal_name: &str) -> Option<Canal> {
        match canals.filter(name.like(canal_name)).first(&self._conn_handler()) {
            Ok(found) => Some(found),
            Err(e) => {
                eprintln!("Cannot find canal {} because : {:?}", canal_name, e);
                None
            }
        }
    }
}