use axum::{Json, http::StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

use crate::db::conection::establish_connection;
use crate::db::models::{Caja, Grupo, Kiosko};
use crate::schema::{
    cajas::dsl as cajas_dsl, grupos::dsl as grupos_dsl, kioskos::dsl as kioskos_dsl,
};
use diesel::prelude::*;
use serde::Serialize;
use crate::db::types::enums::CajasEstadoEnum;


/// Converts any error with a label into an Axum-compatible error response.
pub fn json_error<E: std::fmt::Display>(
    status: StatusCode,
    err: E,
) -> (StatusCode, Json<serde_json::Value>) {
    let error_msg = format!("{}", err);
    (
        status,
        Json(json!({
            "success": false,
            "error": error_msg
        })),
    )
}

pub fn insert_auth_headers(
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
) -> HeaderMap {
    let mut auth_headers = HeaderMap::new();

    auth_headers.insert("api-key", HeaderValue::from_str(&api_key).unwrap());
    auth_headers.insert("secret-key", HeaderValue::from_str(&secret_key).unwrap());

    if let Some(t) = auth_token {
        auth_headers.insert("authorization", HeaderValue::from_str(&t).unwrap());
    }

    //println!("{:#?}", auth_headers);
    return auth_headers;
}

#[derive(Debug, Serialize)]
pub struct KioskoInfo {
    // From kiosko
    pub nombre: String,

    // From caja
    pub id_caja: i32,
    pub nombre_caja: String,
    pub token_autorizacion: Option<String>,
    pub estado: CajasEstadoEnum, // or String if you serialize it as string

    // From grupo
    pub id_yappy: String,
    pub nombre_grupo: String,
    pub api_key: String,
    pub secret_key: String,
}

pub fn get_info_by_mac_address(mac_address: &str) -> Result<KioskoInfo, diesel::result::Error> {
    let mut conn = establish_connection().unwrap();

    let kiosko = kioskos_dsl::kioskos
        .filter(kioskos_dsl::mac_address.eq(mac_address))
        .select(Kiosko::as_select())
        .first::<Kiosko>(&mut conn)?; // Unwrap or return error

    let caja = cajas_dsl::cajas
        .filter(cajas_dsl::id.eq(kiosko.id_caja))
        .select(Caja::as_select())
        .first::<Caja>(&mut conn)?;

    let grupo = grupos_dsl::grupos
        .filter(grupos_dsl::id.eq(caja.id_grupo))
        .select(Grupo::as_select())
        .first::<Grupo>(&mut conn)?;

    Ok(KioskoInfo {
        nombre: kiosko.nombre,
        id_caja: caja.id,
        nombre_caja: caja.nombre_caja,
        token_autorizacion: caja.token_autorizacion,
        estado: caja.estado,
        id_yappy: grupo.id_yappy,
        nombre_grupo: grupo.nombre,
        api_key: grupo.api_key,
        secret_key: grupo.secret_key,
    })
}
