use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Albumes{
    pub id_album: i32,
    pub titulo: String,
    pub fecha_lanzamiento: NaiveDate,
    pub id_artista: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoAlbum {
    pub titulo: String,
    pub fecha_lanzamiento: NaiveDate,
    pub id_artista: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarAlbum {
    pub titulo: String,
    pub fecha_lanzamiento: NaiveDate,
    pub id_artista: i32,
}