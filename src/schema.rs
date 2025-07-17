// @generated automatically by Diesel CLI.
diesel::table! {

    caja_cierre_errores (id) {
        id -> Integer,
        id_caja -> Integer,
        respuesta_json -> Json,
        fecha -> Nullable<Timestamp>,
    }
}

diesel::table! {
    caja_cierre_resumen (id) {
        id -> Integer,
        id_caja -> Integer,
        #[max_length = 50]
        tipo -> Varchar,
        monto -> Decimal,
        transacciones -> Integer,
        fecha -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::db::types::enums::CajasEstadoEnumMapping; 

    cajas (id) {
        id -> Integer,
        id_grupo -> Integer,
        #[max_length = 100]
        nombre_caja -> Varchar,
        #[max_length = 50]
        tipo -> Varchar,
        token_autorizacion -> Nullable<Text>,
        #[max_length = 7]
        estado -> CajasEstadoEnumMapping,
    }
}

diesel::table! {
    grupos (id) {
        id -> Integer,
        #[max_length = 100]
        id_yappy -> Varchar,
        #[max_length = 100]
        nombre -> Varchar,
        #[max_length = 255]
        api_key -> Varchar,
        #[max_length = 255]
        secret_key -> Varchar,
    }
}

diesel::table! {
    kioskos (id) {
        id -> Integer,
        id_caja -> Integer,
        #[max_length = 100]
        nombre -> Varchar,
        #[max_length = 50]
        mac_address -> Varchar,
    }
}

diesel::joinable!(caja_cierre_errores -> cajas (id_caja));
diesel::joinable!(caja_cierre_resumen -> cajas (id_caja));
diesel::joinable!(cajas -> grupos (id_grupo));
diesel::joinable!(kioskos -> cajas (id_caja));

diesel::allow_tables_to_appear_in_same_query!(
    caja_cierre_errores,
    caja_cierre_resumen,
    cajas,
    grupos,
    kioskos,
);
