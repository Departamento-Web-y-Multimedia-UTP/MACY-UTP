use crate::AppState;
use crate::controllers::structs::yappy::AbrirCaja;
use crate::db::{
    models::{NewCajaCierreError, NewCajaCierreResumen},
    types::enums::CajasEstadoEnum,
};
use crate::schema::{caja_cierre_errores, caja_cierre_resumen, cajas};
use crate::utils::utils::{get_info_by_mac_address, insert_auth_headers, json_error};
use axum::http::HeaderMap;
use axum::{Json, http::StatusCode};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use std::env;
//use serde::Serialize;
use serde_json::Value;

pub async fn guardar_datos_caja(
    state: AppState,
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
    caja_id: i32,
    nombre_caja: String,
) -> Result<Value, (StatusCode, Json<Value>)> {
    let mut conn = state.db_pool.get().unwrap();

    let respuesta =
        cerrar_caja_en_yappy(api_key.clone(), secret_key.clone(), auth_token.clone()).await;

    println!("respuesta de caja {}: {:#?}", nombre_caja, respuesta);

    match &respuesta {
        Ok(json) => {
            // Safely extract status.code
            let code = json.pointer("/status/code").and_then(|v| v.as_str());

            if let Some("YP-0000") = code {
                // It's a successful response, extract summaries
                if let Some(summary) = json.pointer("/body/summary").and_then(|v| v.as_array()) {
                    for entry in summary {
                        let tipo = entry
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("UNKNOWN")
                            .to_string();
                        let monto = entry.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let transacciones = entry
                            .get("transactions")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0);

                        // Insert into caja_cierre_resumen
                        let resumen = NewCajaCierreResumen {
                            id_caja: caja_id, // assuming integer
                            tipo,
                            monto: BigDecimal::from_f64(monto).unwrap(),
                            transacciones: transacciones as i32,
                        };

                        let _ = diesel::insert_into(caja_cierre_resumen::table)
                            .values(&resumen)
                            .execute(&mut conn);
                    }
                }

                let _ = diesel::update(cajas::table)
                    .filter(cajas::id.eq(caja_id))
                    .set((
                        cajas::token_autorizacion.eq(None::<String>),
                        cajas::estado.eq(CajasEstadoEnum::Cerrado),
                    ))
                    .execute(&mut conn);
            } else {
                // Save full response to caja_cierre_errores
                let error = NewCajaCierreError {
                    id_caja: caja_id,
                    respuesta_json: json.clone(),
                };

                let _ = diesel::insert_into(caja_cierre_errores::table)
                    .values(&error)
                    .execute(&mut conn);
            }
        }

        Err((_status, err_json)) => {
            // Handle outright request failure
            let error = NewCajaCierreError {
                id_caja: caja_id,
                respuesta_json: err_json.0.clone(),
            };

            let _ = diesel::insert_into(caja_cierre_errores::table)
                .values(&error)
                .execute(&mut conn);
        }
    };

    return respuesta;
}

pub async fn cerrar_caja_en_yappy(
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
) -> Result<Value, (StatusCode, Json<Value>)> {

    let client = reqwest::Client::new();

    let url = format!(
        "{}/session/device",
        env::var("YAPPY_ENDPOINT")
            .map_err(|err| json_error(StatusCode::INTERNAL_SERVER_ERROR, err,))?
    );

    let response = client
        .delete(url)
        .headers(insert_auth_headers(api_key, secret_key, auth_token))
        .send()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?
        .text()
        .await
        .map_err(|err| json_error(StatusCode::BAD_REQUEST, err))?;

    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

    Ok(response_json)
}

pub async fn abrir_caja_and_return_value(
    headers: HeaderMap,
    state: AppState,
) -> Result<Value, (StatusCode, Json<Value>)> {

    let mac_address = headers
        .get("mac-address")
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| json_error(StatusCode::INTERNAL_SERVER_ERROR, "Prohibido"))?;

    let info = get_info_by_mac_address(&state, mac_address)
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

    let mut conn = state.db_pool.get().unwrap();

    let _ = diesel::update(cajas::table.filter(cajas::id.eq(info.id_caja)))
        .set((
            cajas::token_autorizacion.eq(response_json
                .pointer("/body/token")
                .and_then(|v| v.as_str())),
            cajas::estado.eq(CajasEstadoEnum::Abierto),
        ))
        .execute(&mut conn);

    Ok(response_json)
}

pub async fn manage_transaction_response(
    path: &str,
    response_json: &Value,
    id_caja: i32,
    state: &AppState,
) -> Result<Option<String>, (StatusCode, Json<Value>)> {
    // Handle "estado-transaccion"
    if path.contains("estado-transaccion") {
        if let Some(status) = response_json
            .pointer("/body/status")
            .and_then(|s| s.as_str())
        {
            if status == "COMPLETED" {
                let referencia = update_caja_transaccion_actual_null(&state, id_caja);
                return Ok(referencia?);
            }
        }
    }
    // Handle "retornar-transaccion"
    else if path.contains("retornar-transaccion") {
        if let Some(code) = response_json
            .pointer("/status/code")
            .and_then(|c| c.as_str())
        {
            if code == "YP-0000" {
                let referencia = update_caja_transaccion_actual_null(&state, id_caja);
                return Ok(referencia?);
            }
        }
    }

    Ok(None)
}

fn update_caja_transaccion_actual_null(
    state: &AppState,
    id_caja: i32,
) -> Result<Option<String>, (StatusCode, Json<Value>)> {
    let mut conn = state.db_pool.get().unwrap();

    let referencia: Option<String> = cajas::table
        .filter(cajas::id.eq(id_caja))
        .select(cajas::transaccion_actual)
        .first(&mut conn).unwrap();

    diesel::update(cajas::table.filter(cajas::id.eq(id_caja)))
        .set(cajas::transaccion_actual.eq(None::<String>))
        .execute(&mut conn)
        .map_err(|err| {
            json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al actualizar la caja: {}", err),
            )
        })?;

    Ok(referencia)
}
