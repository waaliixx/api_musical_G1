use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::Albumes::{Albumes, NuevoAlbum, ActualizarAlbum};
use crate::repository::AlbumesRepository;

pub async fn obtener_albumes(State(pool): State<PgPool>) ->Json<Vec<Albumes>>{
    let repo= AlbumesRepository::new(pool);
    match repo.obtener_albumes().await {
        Ok(albumes) => Json(albumes),
        Err(_) => Json(vec![]),
    }
}

pub async fn crear_album(State(pool): State<PgPool>, Json(nuevo_album): Json<NuevoAlbum>) ->
Json<Option<Albumes>> {
    let repo = AlbumesRepository::new(pool);
    match repo.crear_album(nuevo_album).await {
        Ok(album) => Json(Some(album)),
        Err(_) => Json(None), 
    }
}

pub async fn eliminar_album(
    State(pool): State<PgPool>,
    Path(id_album): Path<i32>,
) -> Json<bool> {
    let repo = AlbumesRepository::new(pool);
    match repo.eliminar_album(id_album).await{
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn actualizar_album(
    State(pool): State<PgPool>,
    Path(id_album): Path<i32>,
    Json(album_actualizado): Json<ActualizarAlbum>,
) -> Json<Option<Albumes>> {
    let repo = AlbumesRepository::new(pool);

    match repo.actualizar_album(id_album, album_actualizado).await {
        Ok(album) => Json(Some(album)),
        Err(_) => Json(None)
    }
}