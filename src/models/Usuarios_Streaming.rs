use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct UsuariosStreaming {
    pub id_usuario: i32,
    pub nombre_usuario: String,
    pub tipo_suscripcion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoUsuarioStreaming {
    pub id_usuario: i32,
    pub nombre_usuario: String,
    pub tipo_suscripcion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarUsuarioStreaming {
    pub nombre_usuario: String,
    pub tipo_suscripcion: String,
}