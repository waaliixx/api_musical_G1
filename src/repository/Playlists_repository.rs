use sqlx:: {PgPool, Row};
use crate::models::playlists::{actualizar_playlist, nueva_playlist, Playlists};

pub struct PlaylistsRepository {
    pool: PgPool,
}

impl PlaylistsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_playlists(&self) -> sqlx::Result<Vec<Playlists>> {
        let filas = sqlx::query("SELECT id_playlist, nombre_lista, id_usuario, fecha_creacion FROM playlists")
            .fetch_all(&self.pool)
            .await?;

        let playlists = filas.into_iter().map(|fila| Playlists {
            id_playlist: fila.get("id_playlist"),
            nombre_lista: fila.get("nombre_lista"),
            id_usuario: fila.get("id_usuario"),
            fecha_creacion: fila.get("fecha_creacion"),
        }).collect();
        Ok(playlists)
    }

    pub async fn crear_playlist(&self, nueva_playlist: nueva_playlist) -> sqlx::Result<Playlists> {
        let fila = sqlx::query("INSERT INTO playlists (nombre_lista, id_usuario) VALUES ($1, $2) RETURNING id_playlist, nombre_lista, id_usuario, fecha_creacion")
            .bind(&nueva_playlist.nombre_lista)
            .bind(nueva_playlist.id_usuario)
            .fetch_one(&self.pool)
            .await?;

        Ok(Playlists {
            id_playlist: fila.get("id_playlist"),
            nombre_lista: fila.get("nombre_lista"),
            id_usuario: fila.get("id_usuario"),
            fecha_creacion: fila.get("fecha_creacion"),
        })
    }

    pub async fn actualizar_playlist(&self, actualizar_playlist: actualizar_playlist) -> sqlx::Result<Playlists> {
        let fila = sqlx::query("UPDATE playlists SET nombre_lista = $1 WHERE id_playlist = $2 RETURNING id_playlist, nombre_lista, id_usuario, fecha_creacion")
            .bind(&actualizar_playlist.nombre_lista)
            .bind(actualizar_playlist.id_playlist)
            .fetch_one(&self.pool)
            .await?;

        Ok(Playlists {
            id_playlist: fila.get("id_playlist"),
            nombre_lista: fila.get("nombre_lista"),
            id_usuario: fila.get("id_usuario"),
            fecha_creacion: fila.get("fecha_creacion"),
        })
    }

    pub async fn eliminar_playlist(&self, id_playlist: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM playlists WHERE id_playlist = $1")
            .bind(id_playlist)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

}