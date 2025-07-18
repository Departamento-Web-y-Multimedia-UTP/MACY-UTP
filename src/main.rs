pub mod schema;
pub mod start_axum;
pub mod db;
pub mod controllers;
pub mod schedulers;
pub mod utils;

use start_axum::start_axum;
use schedulers::cajas::cerrar_cajas_job;


#[tokio::main]
async fn main() {
    cerrar_cajas_job().await.unwrap();
    start_axum().await.unwrap();
}



