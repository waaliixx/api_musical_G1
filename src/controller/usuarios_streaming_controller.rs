use axum:: {
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::Usuarios_Streaming_service::{
    actualizar_usuario_streaming, 
    crear_usuario_streaming, 
    eliminar_usuario_streaming, 
    eliminar_usuario_streaming_por_id, 
    obtener_usuarios_streaming,
};

#[allow(dead_code)]
pub fn usuarios_streaming_router(pool: PgPool) -> Router {
    Router::new()
        .route("/usuarios_streaming", get(obtener_usuarios_streaming))
        .route("/usuarios_streaming", post(crear_usuario_streaming))
        .route("/usuarios_streaming", delete(eliminar_usuario_streaming))
        .route("/usuarios_streaming/:id_usuario", delete(eliminar_usuario_streaming_por_id))
        .route("/usuarios_streaming/:id_usuario", put(actualizar_usuario_streaming))
        .with_state(pool)
}