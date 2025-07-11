use axum::{
    Router,
    routing::{get, post},
};

use crate::controllers::roquitas::{create_rock, rocks, hello_world};

// Import your controller handlers

pub async fn start_axum() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/create-rock", post(create_rock))
        .route("/rocks", get(rocks));

    println!("booting up server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
