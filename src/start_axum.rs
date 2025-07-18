use axum::{
    Router,
    routing::{delete, get, post},
};

use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use tower_http::catch_panic::CatchPanicLayer;

use crate::{controllers::yappy::{
    abrir_caja, cerrar_caja, generar_qr, handle_transaccion, hello_world,
}, AppState};
use crate::controllers::grupos::{
   get_grupos
};

// Import your controller handlers

pub async fn start_axum(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/grupos", get(get_grupos))
        .route("/abrir-sesion", get(abrir_caja))
        .route("/generar-qr", post(generar_qr))
        .route("/cerrar-sesion", delete(cerrar_caja))
        .route("/estado-transaccion/{id}", get(handle_transaccion))
        .route("/retornar-transaccion/{id}", get(handle_transaccion))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new())
        .with_state(state.clone());

    // solo para desarrollo
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
