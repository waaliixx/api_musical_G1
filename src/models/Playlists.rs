use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Playlists {
    pub id_playlist: i32,
    pub nombre_lista: String,
    pub id_usuario: i32,
    pub fecha_creacion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct nueva_playlist {
    pub nombre_lista: String,
    pub id_usuario: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct actualizar_playlist {
    pub id_playlist: i32,
    pub nombre_lista: String,
}