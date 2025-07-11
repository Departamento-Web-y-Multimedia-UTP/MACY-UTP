use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::db::establish_connection;
use crate::models::ErrorResponse;
use crate::rusty_rocks::{get_rocks, insert_rock};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct CreateRockRequest {
    name: String,
    kind: String,
}

pub async fn hello_world() -> Json<Value> {
    Json(json!({ "mensaje": "hola che" }))
}

pub async fn create_rock(
    Json(payload): Json<CreateRockRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection().unwrap();

    let new_rock = insert_rock(conn, &payload.name, &payload.kind).unwrap();

    Ok(Json(new_rock))
}

pub async fn rocks() -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection().unwrap();
    match get_rocks(conn) {
        Ok(rocks) => Ok(Json(rocks).into_response()),
        Err(e) => {
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}
