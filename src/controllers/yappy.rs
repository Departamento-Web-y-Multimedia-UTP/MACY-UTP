use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Value, json};

// use diesel::{RunQueryDsl, insert_into};
// use crate::db::establish_connection;
// use crate::schema::rocks;
// use crate::models::{NewRock, Rock};

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

#[derive(Serialize, Deserialize)]
pub struct GenerarQR {
    tipo_qr: String,
    subtotal: f64,
    impuesto: f64,
    propina: f64,
    descuento: f64,
    total: f64,
    id_orden: String,
    descripcion: String,
    auth_token: String,
}

impl GenerarQR {
    pub fn to_payload(&self) -> RootPayloadQR {
        RootPayloadQR {
            body: BodyGenerarQR {
                charge_amount: ChargeAmount {
                    sub_total: self.subtotal.clone(),
                    tax: self.impuesto.clone(),
                    tip: self.propina.clone(),
                    discount: self.descuento.clone(),
                    total: self.total.clone(),
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
    pub order_id: String,
    pub description: String,
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

    // let json_value: serde_json::Value = serde_json::from_str(&respuesta).unwrap();
    // println!("{}", serde_json::to_string_pretty(&json_value).unwrap());

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

    println!("{}", url);

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
            Some(auth_token.to_string()),
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

pub async fn estado_transaccion(
    Path((auth_token, id)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    let client = reqwest::Client::new();

    let url = format!(
        "{}/transaction/{}",
        env::var("YAPPY_ENDPOINT").map_err(|err| json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid endpoint",
            err
        ))?,
        id.to_string()
    );

    println!("{}", url);

    let response = client
        .get(url)
        .headers(insert_auth_headers(
            env::var("API_KEY").unwrap(),
            env::var("SECRET_KEY").unwrap(),
            Some(auth_token.to_string()),
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