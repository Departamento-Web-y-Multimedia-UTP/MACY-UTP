use serde::Deserialize;
use serde::Serialize;

pub fn default_f64() -> f64 {
    0.0
}

#[derive(Serialize, Deserialize)]
pub struct RootPayload {
    pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub device: Device,
    pub group_id: String,
}
#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct AbrirCaja {
    pub id_caja: String,
    pub id_grupo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre_caja: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre_cajero: Option<String>,
}

impl AbrirCaja {
    pub fn to_payload(&self) -> RootPayload {
        RootPayload {
            body: Body {
                device: Device {
                    id: self.id_caja.clone(),
                    name: self.nombre_caja.clone(),
                    user: self.nombre_cajero.clone(),
                },
                group_id: self.id_grupo.clone(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GenerarQR {
    pub tipo_qr: String,
    pub subtotal: f64,
    pub total: f64,
    #[serde(default = "default_f64")]
    pub impuesto: f64,
    #[serde(default = "default_f64")]
    pub propina: f64,
    #[serde(default = "default_f64")]
    pub descuento: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_orden: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descripcion: Option<String>,
}

impl GenerarQR {
    pub fn to_payload(&self) -> RootPayloadQR {
        RootPayloadQR {
            body: BodyGenerarQR {
                charge_amount: ChargeAmount {
                    sub_total: self.subtotal,
                    tax: self.impuesto,
                    tip: self.propina,
                    discount: self.descuento,
                    total: self.total,
                },
                order_id: self.id_orden.clone(),
                description: self.descripcion.clone(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RootPayloadQR {
    pub body: BodyGenerarQR,
}

#[derive(Serialize, Deserialize)]
pub struct BodyGenerarQR {
    pub charge_amount: ChargeAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChargeAmount {
    pub sub_total: f64,
    pub tax: f64,
    pub tip: f64,
    pub discount: f64,
    pub total: f64,
}