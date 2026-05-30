use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::Artistas::{Artistas,NuevoArtista,ActualizarArtista};
use crate::repository::ArtistasRepository;

pub async fn obtener_artistas(State(pool): State<PgPool>) ->Json<Vec<Artistas>>{
    let repo= ArtistasRepository::new(pool);
    match repo.obtener_artistas().await {
        Ok(artistas) => Json(artistas),
        Err(_) => Json(vec![]),
    }
}

pub async fn crear_artista(State(pool): State<PgPool>, Json(nuevo_artista): Json<NuevoArtista>) ->
Json<Option<Artistas>> {
    let repo = ArtistasRepository::new(pool);
    match repo.crear_artista(nuevo_artista).await {
        Ok(artistas) => Json(Some(artistas)),
        Err(_) => Json(None), 
    }
}

pub async fn eliminar_artista(
    State(pool): State<PgPool>,
    Path(id_artista): Path<i32>,
) -> Json<bool> {
    let repo = ArtistasRepository::new(pool);
    match repo.eliminar_artista(id_artista).await{
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn actualizar_artista(
    State(pool): State<PgPool>,
    Path(id_artista): Path<i32>,
    Json(artista_actualizado): Json<ActualizarArtista>,
) -> Json<Option<Artistas>> {
    let repo = ArtistasRepository::new(pool);

    match repo.actualizar_artista(id_artista, artista_actualizado).await {
        Ok(artistas) => Json(Some(artistas)),
        Err(_) => Json(None)
    }
}