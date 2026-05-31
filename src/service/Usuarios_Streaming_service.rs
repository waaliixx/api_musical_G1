use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::usuarios_streaming::{ActualizarUsuarioStreaming, NuevoUsuarioStreaming, UsuariosStreaming};
use crate::repository::usuarios_streaming_repository::UsuariosStreamingRepository;

pub async fn obtener_usuarios_streaming(State(pool): State<PgPool>) -> Json<Vec<UsuariosStreaming>> {
    let repo = UsuariosStreamingRepository::new(pool);
    match repo.obtener_usuarios_streaming().await {
        Ok(usuarios) => Json(usuarios),
        Err(_) => Json(vec![]),
    }
}

pub async fn crear_usuario_streaming(State(pool): State<PgPool>, Json(nuevo_usuario): Json<NuevoUsuarioStreaming>) -> Json<Option<UsuariosStreaming>> {
    let repo = UsuariosStreamingRepository::new(pool);
    match repo.crear_usuario_streaming(nuevo_usuario).await {
        Ok(usuario) => Json(Some(usuario)),
        Err(_) => Json(None),
    }
}

pub async fn eliminar_usuario_streaming(State(pool): State<PgPool>, Json(id_usuario): Json<i32>) -> Json<bool> {
    let repo = UsuariosStreamingRepository::new(pool);
    match repo.eliminar_usuario_streaming(id_usuario).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_usuario_streaming_por_id(State(pool): State<PgPool>, Path(id_usuario): Path<i32>) -> Json<bool> {
    let repo = UsuariosStreamingRepository::new(pool);
    match repo.eliminar_usuario_streaming_por_id(id_usuario).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn actualizar_usuario_streaming(State(pool): State<PgPool>, Json(usuario_actualizado): Json<ActualizarUsuarioStreaming>) -> Json<UsuariosStreaming> {
    let repo = UsuariosStreamingRepository::new(pool);
    let id = usuario_actualizado.id_usuario;

    match repo.actualizar_usuario_streaming(id, usuario_actualizado).await {
        Ok(usuario) => Json(usuario),
        Err(_) => Json(UsuariosStreaming {
            id_usuario: 0,
            nombre_usuario: "Error al actualizar usuario".to_string(),
            tipo_suscripcion: "".to_string(),
        }),
    }
}

pub async fn actualizar_usuario_streaming_por_id(State(pool): State<PgPool>, Path(id_usuario): Path<i32>, Json(usuario_actualizado): Json<ActualizarUsuarioStreaming>) -> Json<UsuariosStreaming> {
    let repo = UsuariosStreamingRepository::new(pool);

    match repo.actualizar_usuario_streaming_por_id(id_usuario, usuario_actualizado).await {
        Ok(usuario) => Json(usuario),
        Err(_) => Json(UsuariosStreaming {
            id_usuario: 0,
            nombre_usuario: "Error al actualizar usuario".to_string(),
            tipo_suscripcion: "".to_string(),
        }),
    }
}
