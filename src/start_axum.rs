use axum::{
    routing::{delete, get, post}, Router
};

use crate::controllers::yappy::{hello_world, abrir_caja, generar_qr, cerrar_caja, estado_transaccion};

// Import your controller handlers

pub async fn start_axum() -> Result<(), Box<dyn std::error::Error>> {
    
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/abrir-sesion", post(abrir_caja))   
        .route("/generar-qr", post(generar_qr))
        .route("/cerrar-sesion/{auth_token}", delete(cerrar_caja))
        .route("/estado-transaccion/{auth_token}/{id}", get(estado_transaccion));
        //.route("/cerrar-sesion", get())
        //.route("/retornar-transacion", get())
        //.route("/status-transacion", get())


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
