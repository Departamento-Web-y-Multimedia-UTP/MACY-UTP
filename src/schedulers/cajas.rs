use crate::{db::{
    conection::establish_connection,
    types::enums::CajasEstadoEnum,
}};
use crate::utils::cajas_utils::guardar_datos_caja;
use crate::schema::{cajas, grupos};
use crate::utils::utils::{insert_auth_headers, json_error};
use axum::{Json, http::StatusCode};
use diesel::prelude::*;
use dotenvy::dotenv;
//use serde::Serialize;
use serde_json::{Value};
use std::env;
use tokio_cron_scheduler::{Job, JobScheduler};

#[derive(Queryable, Debug, serde::Serialize)]
pub struct CajaWithCreds {
    pub id: i32,
    pub nombre_caja: String,
    pub estado: CajasEstadoEnum,
    pub api_key: String,
    pub secret_key: String,
    pub token_autorizacion: Option<String>,
}

pub async fn cerrar_cajas_job() -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await.unwrap();

    let cerrar_caja_job = Job::new_async("* * 5 * * *", |_uuid, _lock| {
        //let cerrar_caja_job = Job::new_async("* 59 23 * * *", |_uuid, _lock| {
        Box::pin(async move {
            let mut conn = establish_connection().unwrap();

            let cajas_with_keys: Vec<CajaWithCreds> = cajas::table
                .inner_join(grupos::table.on(grupos::id.eq(cajas::id_grupo)))
                .filter(cajas::estado.eq(CajasEstadoEnum::Abierto))
                .select((
                    cajas::id,
                    cajas::nombre_caja,
                    cajas::estado,
                    grupos::api_key,
                    grupos::secret_key,
                    cajas::token_autorizacion,
                ))
                .load(&mut conn)
                .unwrap();

            println!(
                "revisando si las cajas estan abiertas: {:#?}",
                cajas_with_keys
            );

            for caja in cajas_with_keys {
                println!("cerrando la caja: {}", caja.nombre_caja);

                let _ = guardar_datos_caja(
                    caja.api_key,
                    caja.secret_key,
                    caja.token_autorizacion,
                    caja.id,
                    caja.nombre_caja,
                )
                .await;
            }
        })
    })
    .unwrap();

    scheduler.add(cerrar_caja_job).await.unwrap();
    scheduler.start().await.unwrap();

    Ok(())
}

pub async fn cerrar_caja_en_yappy(
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
) -> Result<Value, (StatusCode, Json<Value>)> {
    dotenv().ok(); // para inicializar el env

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
