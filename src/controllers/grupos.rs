use axum::{Json, http::StatusCode, response::IntoResponse, extract::{State}};

use crate::db::models::{Caja, Grupo};
use crate::db::types::enums::CajasEstadoEnum;
use crate::AppState;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GrupoConCajas {
    pub id: i32,
    pub id_yappy: String,
    pub nombre: String,
    // pub api_key: String,
    // pub secret_key: String,
    pub cajas: Vec<CajaConKiosko>,
}

#[derive(Debug, Serialize)]
pub struct CajaConKiosko {
    pub id: i32,
    pub nombre_caja: String,
    pub tipo: String,
    //pub token_autorizacion: Option<String>,
    pub estado: CajasEstadoEnum,
    //pub kiosko: Option<Kiosko>,
}

pub async fn get_grupos(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    use crate::schema::{cajas::dsl as cajas_dsl, grupos::dsl::*};

    let mut conn = state.db_pool.get().unwrap();

    // 1. Obtener todos los grupos
    let all_grupos: Vec<Grupo> = grupos
        .select(Grupo::as_select())
        .load(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Obtener todas las cajas
    let all_cajas: Vec<Caja> = cajas_dsl::cajas
        .select(Caja::as_select())
        .load(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Obtener todos los kioskos
    // let all_kioskos: Vec<Kiosko> = kioskos_dsl::kioskos
    //     .select(Kiosko::as_select())
    //     .load(&mut conn)
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Composición
    let grupos_con_cajas: Vec<GrupoConCajas> = all_grupos
        .into_iter()
        .map(|g| {
            let cajas_del_grupo = all_cajas
                .iter()
                .filter(|c| c.id_grupo == g.id)
                .map(|c| {
                    // let kiosko = if c.tipo == "kiosko" {
                    //     all_kioskos.iter().find(|k| k.id_caja == c.id).cloned()
                    // } else {
                    //     None
                    // };

                    CajaConKiosko {
                        id: c.id,
                        nombre_caja: c.nombre_caja.clone(),
                        tipo: c.tipo.clone(),
                        //token_autorizacion: c.token_autorizacion.clone(),
                        estado: c.estado.clone(),
                        //kiosko,
                    }
                })
                .collect();

            GrupoConCajas {
                id: g.id,
                id_yappy: g.id_yappy,
                nombre: g.nombre,
                // api_key: g.api_key,
                // secret_key: g.secret_key,
                cajas: cajas_del_grupo,
            }
        })
        .collect();

    Ok(Json(grupos_con_cajas))
}
