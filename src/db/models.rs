use crate::schema::{cajas, grupos, kioskos, caja_cierre_errores, caja_cierre_resumen};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::db::types::enums::CajasEstadoEnum;
use bigdecimal::BigDecimal;
use serde_json::{Value};

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
}

#[derive(Debug,Serialize)]
pub struct ErrorResponse {
    pub error: String,
}


#[derive(Debug, Queryable, Associations, Serialize, Selectable, Clone)]
#[diesel(table_name = kioskos)]
#[diesel(belongs_to(Caja, foreign_key = id_caja))]
pub struct Kiosko {
    pub id: i32,
    pub id_caja: i32,
    pub nombre: String,
    pub mac_address: String,
}

#[derive(Debug,Queryable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(table_name = cajas)]
#[diesel(belongs_to(Grupo, foreign_key = id_grupo))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Caja {
    pub id: i32,
    pub id_grupo: i32,
    pub nombre_caja: String,
    pub tipo: String,
    pub token_autorizacion: Option<String>,
    pub estado: CajasEstadoEnum,
}

#[derive(Debug,Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = grupos)]
pub struct Grupo {
    pub id: i32,
    pub id_yappy: String,
    pub nombre: String,
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = caja_cierre_resumen)]
pub struct NewCajaCierreResumen {
    pub id_caja: i32,
    pub tipo: String,
    pub monto: BigDecimal,
    pub transacciones: i32,
}

#[derive(Insertable)]
#[diesel(table_name = caja_cierre_errores)]
pub struct NewCajaCierreError {
    pub id_caja: i32,
    pub respuesta_json: Value,
}