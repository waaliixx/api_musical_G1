use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::playlists::{Playlists, actualizar_playlist, nueva_playlist};
use crate::repository::playlists_repository::playlists_repository as PlaylistsRepository;

pub async fn obtener_playlist(State(pool): State<PgPool>) -> Json<Vec<Playlists>> {
    let playlists = PlaylistsRepository::new(pool);
    match playlists.obtener_playlists().await {
        Ok(playlists) => Json(playlists),
        Err(_) => Json(vec![]),
    }
}


pub async fn crear_playlist(State(pool): State<PgPool>, Json(nueva_playlist): Json<nueva_playlist>) -> 
Json<Option<Playlists>> {
    let playlists = PlaylistsRepository::new(pool);
    match playlists.crear_playlist(nueva_playlist).await {
        Ok(playlist) => Json(Some(playlist)),
        Err(_) => Json(None),
    }
}

pub async fn actualizar_playlist(State(pool): State<PgPool>, Json(actualizar_playlist): 
Json<actualizar_playlist>) -> Json<Option<Playlists>> {
    let playlists = PlaylistsRepository::new(pool);
    match playlists.actualizar_playlist(actualizar_playlist).await {
        Ok(playlist) => Json(Some(playlist)),
        Err(_) => Json(None),
    }
}

pub async fn eliminar_playlist(State(pool): State<PgPool>, Path(id_playlist): Path<i32>) -> 
Json<bool> {
    let playlists = PlaylistsRepository::new(pool);
    match playlists.eliminar_playlist(id_playlist).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn obtener_playlist_por_id(State(pool): State<PgPool>, Path(id_playlist): Path<i32>) -> 
Json<Option<Playlists>> {
    let playlists = PlaylistsRepository::new(pool);
    match playlists.obtener_playlist_por_id(id_playlist).await {
        Ok(playlist) => Json(Some(playlist)),
        Err(_) => Json(None),
    }
}
