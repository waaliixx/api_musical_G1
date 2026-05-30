use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Canciones {
    pub id_cancion: i32,
    pub nombre: String,
    pub duracion: NaiveTime,
    pub id_album: i32,
}   

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaCancion {
    pub nombre: String,
    pub duracion: NaiveTime,
    pub id_album: i32,
}   

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarCancion {
    pub nombre: String,
    pub duracion: NaiveTime,
    pub id_album: i32,
}   

