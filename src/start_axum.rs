use axum::{
    Router,
    routing::{delete, get, post},
};

use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use tower_http::catch_panic::CatchPanicLayer;

use crate::controllers::yappy::{
    abrir_caja, cerrar_caja, generar_qr, handle_transaccion, hello_world,
};

// Import your controller handlers

pub async fn start_axum() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/abrir-sesion", post(abrir_caja))
        .route("/generar-qr", post(generar_qr))
        .route("/cerrar-sesion/{auth_token}", delete(cerrar_caja))
        .route(
            "/estado-transaccion/{auth_token}/{id}",
            get(handle_transaccion),
        )
        .route(
            "/retornar-transaccion/{auth_token}/{id}",
            get(handle_transaccion),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new());

    // solo para desarrollo
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
