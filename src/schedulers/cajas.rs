use crate::db::types::enums::CajasEstadoEnum;
use crate::schema::{cajas, grupos};
use crate::utils::cajas_utils::guardar_datos_caja;
use diesel::prelude::*;
use crate::AppState;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

#[derive(Queryable, Debug, serde::Serialize)]
pub struct CajaWithCreds {
    pub id: i32,
    pub nombre_caja: String,
    pub estado: CajasEstadoEnum,
    pub api_key: String,
    pub secret_key: String,
    pub token_autorizacion: Option<String>,
}

pub async fn cerrar_cajas_job(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await.unwrap();

    let state = state.clone(); // 👈 clone it outside

    let cerrar_caja_job = JobBuilder::new()
        .with_timezone(chrono_tz::America::Panama)
        .with_cron_job_type()
        .with_schedule("* 59 23 * * *")
        .unwrap()
        .with_run_async(Box::new(move |_uuid, mut _lock| {
            let state = state.clone(); // 👈 move it into the closure
            Box::pin(async move {
                let mut conn = state.db_pool.get().unwrap();

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
                        state.clone(),
                        caja.api_key,
                        caja.secret_key,
                        caja.token_autorizacion,
                        caja.id,
                        caja.nombre_caja,
                    )
                    .await;
                }
            })
        }))
        .build()
        .unwrap();

    scheduler.add(cerrar_caja_job).await.unwrap();
    scheduler.start().await.unwrap();

    Ok(())
}
