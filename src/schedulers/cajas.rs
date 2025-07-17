use crate::db::{
    conection::establish_connection,
    models::{NewCajaCierreError, NewCajaCierreResumen},
    types::enums::CajasEstadoEnum,
};
use crate::schema::{caja_cierre_errores, caja_cierre_resumen, cajas, grupos};
use crate::utils::utils::{insert_auth_headers, json_error};
use axum::{Json, http::StatusCode};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use dotenvy::dotenv;
//use serde::Serialize;
use serde_json::{Value, json};
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

pub async fn cerrar_cajas() -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await.unwrap();

    let cerrar_caja_job = Job::new_async("1/50 * * * * *", |_uuid, _lock| {
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

                let respuesta = ir_cerrar_caja(
                    caja.api_key.clone(),
                    caja.secret_key.clone(),
                    caja.token_autorizacion.clone(),
                )
                .await;

                println!("respuesta de caja {}: {:#?}", caja.nombre_caja, respuesta);

                match respuesta {
                    Ok(Json(json)) => {
                        // Safely extract status.code
                        let code = json.pointer("/data/status/code").and_then(|v| v.as_str());

                        if let Some("YP-0000") = code {
                            // It's a successful response, extract summaries
                            if let Some(summary) = json
                                .pointer("/data/body/summary")
                                .and_then(|v| v.as_array())
                            {
                                for entry in summary {
                                    let tipo = entry
                                        .get("type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("UNKNOWN")
                                        .to_string();
                                    let monto =
                                        entry.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                    let transacciones = entry
                                        .get("transactions")
                                        .and_then(|v| v.as_i64())
                                        .unwrap_or(0);

                                    // Insert into caja_cierre_resumen
                                    let resumen = NewCajaCierreResumen {
                                        id_caja: caja.id, // assuming integer
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
                                .filter(cajas::id.eq(caja.id))
                                .set((
                                    cajas::token_autorizacion.eq(None::<String>),
                                    cajas::estado.eq(CajasEstadoEnum::Cerrado),
                                ))
                                .execute(&mut conn);
                        } else {
                            // Save full response to caja_cierre_errores
                            let error = NewCajaCierreError {
                                id_caja: caja.id,
                                respuesta_json: json,
                            };

                            let _ = diesel::insert_into(caja_cierre_errores::table)
                                .values(&error)
                                .execute(&mut conn);
                        }
                    }

                    Err((_status, err_json)) => {
                        // Handle outright request failure
                        let error = NewCajaCierreError {
                            id_caja: caja.id,
                            respuesta_json: err_json.0,
                        };

                        let _ = diesel::insert_into(caja_cierre_errores::table)
                            .values(&error)
                            .execute(&mut conn);
                    }
                };
            }
        })
    })
    .unwrap();

    scheduler.add(cerrar_caja_job).await.unwrap();
    scheduler.start().await.unwrap();

    Ok(())
}

pub async fn ir_cerrar_caja(
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
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

    Ok(Json(json!({
        "success": true,
        "data": response_json
    })))
}
