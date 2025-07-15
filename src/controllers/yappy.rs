use axum::{
    Json,
    extract::{OriginalUri, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};

// use diesel::{RunQueryDsl, insert_into};
// use crate::db::establish_connection;

use crate::utils::utils::{insert_auth_headers, json_error};

use dotenvy::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct CreateRockRequest {
    name: String,
    kind: String,
}

#[derive(Serialize, Deserialize)]
pub struct RootPayload {
    pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub device: Device,
    pub group_id: String,
}
#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub user: String,
}
#[derive(Serialize, Deserialize)]
pub struct AbrirCaja {
    id_caja: String,
    id_grupo: String,
    nombre_caja: String,
    nombre_cajero: String,
}
impl AbrirCaja {
    pub fn to_payload(&self) -> RootPayload {
        RootPayload {
            body: Body {
                device: Device {
                    id: self.id_caja.clone(),
                    name: self.nombre_caja.clone(),
                    user: self.nombre_cajero.clone(),
                },
                group_id: self.id_grupo.clone(),
            },
        }
    }
}

fn default_f64() -> f64 {
    0.0
}

#[derive(Serialize, Deserialize)]
pub struct GenerarQR {
    auth_token: String,
    tipo_qr: String,
    subtotal: f64,
    total: f64,
    #[serde(default = "default_f64")]
    impuesto: f64,
    #[serde(default = "default_f64")]
    propina: f64,
    #[serde(default = "default_f64")]
    descuento: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_orden: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descripcion: Option<String>,
}

impl GenerarQR {
    pub fn to_payload(&self) -> RootPayloadQR {
        RootPayloadQR {
            body: BodyGenerarQR {
                charge_amount: ChargeAmount {
                    sub_total: self.subtotal,
                    tax: self.impuesto,
                    tip: self.propina,
                    discount: self.descuento,
                    total: self.total,
                },
                order_id: self.id_orden.clone(),
                description: self.descripcion.clone(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RootPayloadQR {
    pub body: BodyGenerarQR,
}

#[derive(Serialize, Deserialize)]
pub struct BodyGenerarQR {
    pub charge_amount: ChargeAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChargeAmount {
    pub sub_total: f64,
    pub tax: f64,
    pub tip: f64,
    pub discount: f64,
    pub total: f64,
}

pub async fn hello_world() -> Json<Value> {
    Json(json!({ "mensaje": "hola che" }))
}

// pub async fn create_rock(
//     Json(payload): Json<CreateRockRequest>,
// ) -> Result<impl IntoResponse, StatusCode> {
//     let conn = &mut establish_connection().unwrap();
//     let name = payload.name;
//     let kind = payload.kind;

//     let new_rock = NewRock {
//         name: &name,
//         kind: &kind,
//     };

//     let insert_result = insert_into(rocks::table).values(&new_rock).execute(conn);

//     match insert_result {
//         Ok(rows_affected) => Ok(Json(json!({ "success": true, "inserted": rows_affected }))),
//         Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }

// pub async fn rocks() -> Result<impl IntoResponse, StatusCode> {
//     let conn = &mut establish_connection().unwrap();

//     use crate::schema::rocks::dsl::*;
//     let results = rocks
//         .load::<Rock>(conn)
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(Json(results))
// }

pub async fn abrir_caja(
    Json(payload): Json<AbrirCaja>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    //let conn = &mut establish_connection().unwrap(); //conexion a base de datos

    let formatted = payload.to_payload();

    let client = reqwest::Client::new();

    let url = format!(
        "{}/session/device",
        env::var("YAPPY_ENDPOINT").map_err(|err| json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid endpoint",
            err
        ))?
    );

    let response = client
        .post(url)
        .headers(insert_auth_headers(
            env::var("API_KEY").unwrap(),
            env::var("SECRET_KEY").unwrap(),
            None,
        ))
        .json(&formatted) // This automatically serializes `formatted` to JSON
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}

pub async fn generar_qr(
    Json(payload): Json<GenerarQR>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    let formatted = payload.to_payload();

    println!("{}", serde_json::to_string_pretty(&formatted).unwrap());

    let client = reqwest::Client::new();

    let tipo_qr = match payload.tipo_qr.as_str() {
        "dinamico" => "DYN",
        "hibrido" => "HYB",
        _ => "DYN",
    };

    let url = format!(
        "{}/qr/generate/{}",
        env::var("YAPPY_ENDPOINT").map_err(|err| json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid endpoint",
            err
        ))?,
        tipo_qr
    );

    let response = client
        .post(url)
        .headers(insert_auth_headers(
            env::var("API_KEY").unwrap(),
            env::var("SECRET_KEY").unwrap(),
            Some(payload.auth_token.to_string()),
        ))
        .json(&formatted) // This automatically serializes `formatted` to JSON
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}

pub async fn cerrar_caja(
    Path(auth_token): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    let client = reqwest::Client::new();

    let url = format!(
        "{}/session/device",
        env::var("YAPPY_ENDPOINT").map_err(|err| json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid endpoint",
            err
        ))?
    );

    println!("{}", url);

    let response = client
        .delete(url)
        .headers(insert_auth_headers(
            env::var("API_KEY").unwrap(),
            env::var("SECRET_KEY").unwrap(),
            Some(auth_token),
        ))
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}

pub async fn handle_transaccion(
    Path((auth_token, id)): Path<(String, String)>,
    OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env
    let path = uri.path();

    let client = reqwest::Client::new();

    println!("{}", path);
    
    let method = if path.contains("estado-transaccion") {
        "GET"
    } else if path.contains("retornar-transaccion") {
        "PUT"
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid route" })),
        ));
    };

    let url = format!(
        "{}/transaction/{}",
        env::var("YAPPY_ENDPOINT").map_err(|err| json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid endpoint",
            err
        ))?,
        id.to_string()
    );

    let request_builder = match method {
        "GET" => client.get(url),
        "PUT" => client.put(url),
        _ => unreachable!("No hay metodo"),
    }
    .headers(insert_auth_headers(
        std::env::var("API_KEY").unwrap(),
        std::env::var("SECRET_KEY").unwrap(),
        Some(auth_token),
    ));

    let response = request_builder
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, "error", err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}
