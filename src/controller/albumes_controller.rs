use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::albumes_service::{
    obtener_albumes,
    crear_album,
    eliminar_album,
    actualizar_album,
};

pub fn album_router(pool: PgPool) -> Router{
    Router::new()
        .route("/api/albumes", get(obtener_albumes)) 
        .route("/api/albumes", post(crear_album)) 
        .route("/api/albumes/{id_album}", delete(eliminar_album)) 
        .route("/api/albumes/{id_album}", put(actualizar_album))
        .with_state(pool)
}
