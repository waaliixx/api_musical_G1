use axum::{
    routing::{delete, get, post, put},
    Router, 
};

use sqlx::PgPool;
use crate::service::artistas_services::{
    obtener_artistas,
    crear_artista,
    eliminar_artista,
    actualizar_artista,
};

pub fn artistas_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/artistas", get(obtener_artistas))
        .route("/api/artistas", post(crear_artista))
        .route("/api/artistas/{id_artista}", delete(eliminar_artista))
        .route("api/artistas/{id_artista}", put(actualizar_artista))
        .with_state(pool)
}
