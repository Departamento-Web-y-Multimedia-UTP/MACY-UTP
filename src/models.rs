use crate::schema::rocks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
   pub  error: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = rocks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Rock {
    pub id: i32,
    pub name: String,
    pub kind: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = rocks)]
pub struct NewRock<'a> {
    pub name: &'a str,
    pub kind: &'a str,
}