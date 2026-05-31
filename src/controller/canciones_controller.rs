use axum::{
    routing::{delete, get, post, put},
    Router, 
};

use sqlx::PgPool;
use crate::service::canciones_services::{
    obtener_canciones, 
    crear_canciones,
    eliminar_cancion,
    actualizar_cancion
};

pub fn canciones_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/canciones", get(obtener_canciones))
        .route("/api/canciones", post(crear_canciones))
        .route("/api/canciones/{id_cancion}", delete(eliminar_cancion))
        .route("/api/canciones/{id_cancion}", put(actualizar_cancion))
        .with_state(pool)
}


