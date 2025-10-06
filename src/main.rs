pub mod schema;
pub mod start_axum;
pub mod db;
pub mod controllers;
pub mod schedulers;
pub mod utils;

use dotenvy::dotenv;
use start_axum::start_axum;
use schedulers::cajas::cerrar_cajas_job;

use crate::db::conection::{create_pool, MySqlPool};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_pool = create_pool();
    let state = AppState { db_pool };
    
    cerrar_cajas_job(&state).await.unwrap();
    start_axum(&state).await.unwrap();
}



