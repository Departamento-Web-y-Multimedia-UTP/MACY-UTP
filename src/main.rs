pub mod models;
pub mod rusty_rocks;
pub mod schema;
pub mod start_axum;
pub mod db;
pub mod controllers;
pub mod schedulers;
use start_axum::start_axum;
use schedulers::primero::start_primero_schedulers;


#[tokio::main]
async fn main() {
    start_primero_schedulers().await.unwrap();
    start_axum().await.unwrap();
}



