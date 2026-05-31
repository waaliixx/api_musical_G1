use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::playlists_service::{
    obtener_playlist, 
    crear_playlist, 
    actualizar_playlist, 
    eliminar_playlist, 
    obtener_playlist_por_id,
};

pub fn playlists_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/playlists", get(obtener_playlist))
        .route("/api/playlists", post(crear_playlist))
        .route("/api/playlists", put(actualizar_playlist))
        .route("/api/playlists/{id_playlist}", delete(eliminar_playlist))
        .route("/api/playlists/{id_playlist}", get(obtener_playlist_por_id))
        .with_state(pool)
}