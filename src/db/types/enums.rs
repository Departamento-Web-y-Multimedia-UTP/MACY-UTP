use serde::{Deserialize, Serialize};
#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CajasEstadoEnum {
    Cerrado,
    Abierto,
}