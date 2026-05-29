use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::playlists::{Playlists, actualizar_playlist, nueva_playlist};
use crate::repository::playlists_repository;

pub async fn obtener(State(pool): State<PgPool>) -> Json<Vec<Playlists>> {
    let playlists = playlists_repository:: new(&pool);
    match playlists.obtener_playlists().await {
        ok(Playlists) => Json(playlists),
        Err(_) => Json(vec![]),
    }
}

/*
pub async fn obtener_por_id(State(pool): State<PgPool>, Path(id_playlist): Path<i32>) -> Json<Option<Playlists>> {
    let playlists = playlists_repository::new(&pool);
    match playlists.obtener_playlist_por_id(id_playlist).await {
        Ok(playlist) => Json(Some(playlist)),
        Err(_) => Json(None),
    }
}

pub async fn obtener_por_usuario(State(pool): State<PgPool>, Path(id_usuario): Path<i32>) -> Json<Vec<Playlists>> {
    let playlists = playlists_repository::new(&pool);
    match playlists.obtener_playlists_por_usuario(id_usuario).await {
        Ok(playlists) => Json(playlists),
        Err(_) => Json(vec![]),
    }
}
*/


pub async fn crear(State(pool): State<PgPool>, Json(nueva_playlist): Json<nueva_playlist>) -> 
Json<Option<nueva_playlist>>{
    let playlists = playlists_repository::new(&pool);
    match playlists.crear_playlist(nueva_playlist).await {
        Ok(playlist) => Json(Some(playlist)),
        Err(_) => Json(Playlists {
            id_playlist: 0,
            nombre_lista: "Error al crear playlist".to_string(),
            id_usuario: 0,
            fecha_creacion: chrono::Utc::now(),
        }),
    }
}

pub async fn actualizar(State(pool): State<PgPool>, Json(actualizar_playlist): 
Json<actualizar_playlist>) -> Json<Option<actualizar_playlist>> {
    let playlists = playlists_repository::new(&pool);
    match playlists.actualizar_playlist(actualizar_playlist).await {
        Ok(playlist) => Json(Some(playlist)),
        Err(_) => Json(Playlists {
            id_playlist: 0,
            nombre_lista: "Error al actualizar playlist".to_string(),
            id_usuario: 0,
            fecha_creacion: chrono::Utc::now(),
        }),
    }
}

pub async fn eliminar(State(pool): State<PgPool>, Path(id_playlist): Path<i32>) -> 
Json<bool> {
    let playlists = playlists_repository::new(&pool);
    match playlists.eliminar_playlist(id_playlist).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_por_usuario(State(pool): State<PgPool>, Path(id_usuario): Path<i32>) -> 
Json<bool> {
    let playlists = playlists_repository::new(&pool);
    match playlists.eliminar_playlist(id_usuario).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}



