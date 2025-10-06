use std::env;

use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool}
};

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn create_pool() -> MySqlPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create MySQL connection pool")
}