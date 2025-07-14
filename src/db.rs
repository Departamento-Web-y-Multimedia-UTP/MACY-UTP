
use dotenvy::dotenv;
use std::env;

use diesel::{
    mysql::MysqlConnection, Connection
};

pub fn establish_connection() -> Result<MysqlConnection, diesel::result::ConnectionError> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").map_err(|_| {
        diesel::result::ConnectionError::BadConnection("DATABASE_URL not set".into())
    })?;
    println!("{}", db_url);
    MysqlConnection::establish(&db_url)
}
