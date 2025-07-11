

use diesel::{
    mysql::MysqlConnection, RunQueryDsl
};

use crate::models::{NewRock, Rock};
use crate::schema::rocks;

pub fn insert_rock(
    conn: &mut MysqlConnection,
    name: &str,
    kind: &str,
) -> Result<usize, diesel::result::Error> {
    let new_rock = NewRock { name, kind };

    diesel::insert_into(rocks::table)
        .values(&new_rock)
        .execute(conn)
}

pub fn get_rocks(conn: &mut MysqlConnection) -> Result<Vec<Rock>, diesel::result::Error> {
    use crate::schema::rocks::dsl::*;
    rocks.load::<Rock>(conn)
}