use diesel::prelude::*;
use diesel::mysql::{MysqlConnection, Mysql};
use std::env;
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};

pub type MySQLPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySQLPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_pool() -> MySQLPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::new(&database_url);

    MySQLPool::builder()
        .max_size(20)
        .build(manager)
        .expect("Failed to create pool.")
}