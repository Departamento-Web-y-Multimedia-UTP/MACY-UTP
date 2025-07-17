use axum::{
    Json,
    extract::{OriginalUri, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Value, json};

// use diesel::{RunQueryDsl, insert_into};
//use crate::db::conection::establish_connection;

use crate::utils::utils::{get_info_by_mac_address, insert_auth_headers, json_error};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct AbrirCaja {
    id_caja: String,
    id_grupo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre_caja: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre_cajero: Option<String>,
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
    Json(json!({ "mensaje": "hola che aqui hay yappy" }))
}

pub async fn abrir_caja(
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    let mac_address = headers
        .get("mac-address")
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Prohibido"))?;

    let info = get_info_by_mac_address(mac_address)
        .map_err(|_err| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Sin acceso"))?;

    let info_abrir = AbrirCaja {
        id_caja: info.nombre_caja.to_string(), // id como nombre de la caja en yappy
        id_grupo: info.id_yappy.clone(), //id del grupo de yappy
        nombre_caja: Some(info.nombre.clone()),
        nombre_cajero: None,
    };

    let formatted = info_abrir.to_payload();

    println!("{}", serde_json::to_string_pretty(&formatted).unwrap());

    let client = reqwest::Client::new();

    let url = format!(
        "{}/session/device",
        env::var("YAPPY_ENDPOINT")
            .map_err(|err| json_error(StatusCode::INTERNAL_SERVER_ERROR, err))?
    );

    let response = client
        .post(url)
        .headers(insert_auth_headers(
            info.api_key, 
            info.secret_key, 
            None))
        .json(&formatted) // This automatically serializes `formatted` to JSON
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}

pub async fn generar_qr(
    headers: HeaderMap,
    Json(mut payload): Json<GenerarQR>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    let mac_address = headers
        .get("mac-address")
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Prohibido"))?;

    let info = get_info_by_mac_address(mac_address)
        .map_err(|_err| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Sin acceso"))?;

    println!("{:#?}", info);

    payload.descripcion = format!("Pago en Kiosko UTP del {}", info.nombre).into();

    let formatted = payload.to_payload();

    println!(
        "informacion formatiada: {}",
        serde_json::to_string_pretty(&formatted).unwrap()
    );

    let client = reqwest::Client::new();

    let tipo_qr = match payload.tipo_qr.as_str() {
        "dinamico" => "DYN",
        "hibrido" => "HYB",
        _ => "DYN",
    };

    let url = format!(
        "{}/qr/generate/{}",
        env::var("YAPPY_ENDPOINT")
            .map_err(|err| json_error(StatusCode::INTERNAL_SERVER_ERROR, err))?,
        tipo_qr
    );

    let response = client
        .post(url)
        .headers(insert_auth_headers(
            info.api_key,
            info.secret_key,
            info.token_autorizacion,
        ))
        .json(&formatted) // This automatically serializes `formatted` to JSON
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}

pub async fn cerrar_caja(
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    let mac_address = headers
        .get("mac-address")
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Prohibido"))?;

    let info = get_info_by_mac_address(mac_address)
        .map_err(|_err| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Sin acceso"))?;

    let client = reqwest::Client::new();

    let url = format!(
        "{}/session/device",
        env::var("YAPPY_ENDPOINT")
            .map_err(|err| json_error(StatusCode::INTERNAL_SERVER_ERROR, err,))?
    );

    //println!("{}", url);

    let response = client
        .delete(url)
        .headers(insert_auth_headers(
            info.api_key,
            info.secret_key,
            info.token_autorizacion,
        ))
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}

pub async fn handle_transaccion(
    headers: HeaderMap,
    Path(id): Path<String>,
    OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

    let mac_address = headers
        .get("mac-address")
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Prohibido"))?; // return early if header missing or invalid

    let info = get_info_by_mac_address(mac_address)
        .map_err(|_err| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Sin acceso"))?; // or map the diesel error more precisely

    let path = uri.path();

    let client = reqwest::Client::new();

    //println!("{}", path);

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
        env::var("YAPPY_ENDPOINT")
            .map_err(|err| json_error(StatusCode::INTERNAL_SERVER_ERROR, err,))?,
        id.to_string()
    );

    let request_builder = match method {
        "GET" => client.get(url),
        "PUT" => client.put(url),
        _ => unreachable!("No hay metodo"),
    }
    .headers(insert_auth_headers(
        info.api_key,
        info.secret_key,
        info.token_autorizacion,
    ));

    let response = request_builder
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}
