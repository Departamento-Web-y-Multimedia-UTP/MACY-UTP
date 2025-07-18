use crate::schema::{caja_cierre_errores, caja_cierre_resumen, cajas};
use crate::{
    db::{
        conection::establish_connection,
        models::{NewCajaCierreError, NewCajaCierreResumen},
        types::enums::CajasEstadoEnum,
    },
    schedulers::cajas::cerrar_caja_en_yappy,
};
use axum::{Json, http::StatusCode};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
//use serde::Serialize;
use serde_json::Value;

pub async fn guardar_datos_caja(
    api_key: String,
    secret_key: String,
    auth_token: Option<String>,
    caja_id: i32,
    nombre_caja: String,
) -> Result<Value, (StatusCode, Json<Value>)> {
    let mut conn = establish_connection().unwrap();

    let respuesta =
        cerrar_caja_en_yappy(api_key.clone(), secret_key.clone(), auth_token.clone()).await;

    println!("respuesta de caja {}: {:#?}", nombre_caja, respuesta);

    match &respuesta {
        Ok(json) => {
            // Safely extract status.code
            let code = json.pointer("/status/code").and_then(|v| v.as_str());

            if let Some("YP-0000") = code {
                // It's a successful response, extract summaries
                if let Some(summary) = json
                    .pointer("/body/summary")
                    .and_then(|v| v.as_array())
                {
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
