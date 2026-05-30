use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::Canciones::{Canciones, NuevaCancion, ActualizarCancion};
use crate::repository::CancionesRepository;

pub async fn obtener_canciones(State(pool): State<PgPool>) ->Json<Vec<Canciones>>{
    let repo= CancionesRepository::new(pool);
    match repo.obtener_canciones().await {
        Ok(canciones) => Json(canciones),
        Err(_) => Json(vec![]),
    }   
}

pub async fn crear_canciones(State(pool): State<PgPool>, Json(nueva_cancion): Json<NuevaCancion>) ->
Json<Option<Canciones>> {
    let repo = CancionesRepository::new(pool);
    match repo.crear_cancion(nueva_cancion).await {
        Ok(cancion) => Json(Some(cancion)),
        Err(_) => Json(None), 
    }
}

pub async fn eliminar_cancion(
    State(pool): State<PgPool>,
    Path(id_cancion): Path<i32>,
) -> Json<bool> {
    let repo = CancionesRepository::new(pool);
    match repo.eliminar_cancion(id_cancion).await{
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn actualizar_cancion(
    State(pool): State<PgPool>,
    Path(id_cancion): Path<i32>,
    Json(cancion_actualizada): Json<ActualizarCancion>,
) -> Json<Option<Canciones>> {
    let repo = CancionesRepository::new(pool);

    match repo.actualizar_cancion(id_cancion, cancion_actualizada).await {
        Ok(cancion) => Json(Some(cancion)),
        Err(_) => Json(None)
    }
}



    




