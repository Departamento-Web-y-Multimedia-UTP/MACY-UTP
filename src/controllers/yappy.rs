use crate::controllers::structs::yappy::{AbrirCaja, GenerarQR};
use crate::db::conection::establish_connection;
use crate::db::models::Caja;
use crate::db::types::enums::CajasEstadoEnum;
use crate::schema::cajas;
use crate::utils::cajas_utils::guardar_datos_caja;
use crate::utils::utils::{get_info_by_mac_address, insert_auth_headers, json_error};
use axum::{
    Json,
    extract::{OriginalUri, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use serde_json::{Value, json};
use std::env;

pub async fn hello_world() -> Json<Value> {
    Json(json!({ "mensaje": "hola che aqui hay yappy" }))
}

async fn abrir_caja_and_return_value(
    headers: HeaderMap,
) -> Result<Value, (StatusCode, Json<Value>)> {
    dotenv().ok();

    let mac_address = headers
        .get("mac-address")
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Prohibido"))?;

    let info = get_info_by_mac_address(mac_address)
        .map_err(|_| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Sin acceso"))?;

    let info_abrir = AbrirCaja {
        id_caja: info.nombre_caja.to_string(),
        id_grupo: info.id_yappy.clone(),
        nombre_caja: Some(info.nombre.clone()),
        nombre_cajero: None,
    };

    let formatted = info_abrir.to_payload();

    let client = reqwest::Client::new();
    let url = format!(
        "{}/session/device",
        env::var("YAPPY_ENDPOINT")
            .map_err(|err| json_error(StatusCode::INTERNAL_SERVER_ERROR, err))?
    );

    let response = client
        .post(url)
        .headers(insert_auth_headers(info.api_key, info.secret_key, None))
        .json(&formatted)
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?;

    let response_json: Value = serde_json::from_str(&response).unwrap();

    // Update database
    use crate::schema::cajas::dsl::*;

    let mut conn = establish_connection().unwrap();

    let _ = diesel::update(cajas.filter(id.eq(info.id_caja)))
        .set((
            token_autorizacion.eq(response_json
                .pointer("/body/token")
                .and_then(|v| v.as_str())),
            estado.eq(CajasEstadoEnum::Abierto),
        ))
        .execute(&mut conn);

    Ok(response_json)
}

pub async fn abrir_caja(
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let json = abrir_caja_and_return_value(headers).await?;
    Ok(Json(json!({
        "success": true,
        "data": json
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

    let mut info = get_info_by_mac_address(mac_address)
        .map_err(|_err| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Sin acceso"))?;

    let mut conn = establish_connection().unwrap();

    let caja = cajas::table
        .filter(cajas::id.eq(info.id_caja))
        .select(Caja::as_select())
        .first::<Caja>(&mut conn)
        .map_err(|_| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Caja no encontrada"))?;

    if caja.estado.eq(&CajasEstadoEnum::Cerrado) {
        println!(
            "Caja {} está cerrada. Abriéndola automáticamente...",
            info.nombre
        );
        //Llama a tu función abrir_caja con headers
        let info_caja_json = abrir_caja_and_return_value(headers.clone())
            .await
            .map_err(|_| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Error al abrir la caja"))?;

        // access the token from JSON
        info.token_autorizacion = info_caja_json
            .pointer("/body/token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }

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

    let response_json = guardar_datos_caja(
        info.api_key,
        info.secret_key,
        info.token_autorizacion,
        info.id_caja,
        info.nombre_caja,
    )
    .await?;

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
