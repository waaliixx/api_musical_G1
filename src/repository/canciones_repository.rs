use sqlx::{PgPool, Row};
use crate::models::Canciones::{Canciones, NuevaCancion, ActualizarCancion};

pub struct CancionesRepository{
    pool: PgPool,
}

impl CancionesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_canciones(&self) -> sqlx::Result<Vec<Canciones>> {
        let filas = sqlx::query("SELECT id_cancion, nombre, duracion, id_album FROM Canciones")
            .fetch_all(&self.pool)
            .await?;
        let canciones = filas.into_iter().map(|fila| Canciones {
            id_cancion: fila.get("id_cancion"),
            nombre: fila.get("nombre"),
            duracion: fila.get("duracion"),
            id_album: fila.get("id_album"),
        }).collect();
        Ok(canciones)
    }

    pub async fn crear_cancion(&self, nueva_cancion: NuevaCancion) -> sqlx::Result<Canciones> {
        let fila = sqlx::query("INSERT INTO Canciones (nombre, duracion, id_album) VALUES ($1, $2, $3) RETURNING id_cancion, nombre, duracion, id_album")
            .bind(&nueva_cancion.nombre)
            .bind(nueva_cancion.duracion)
            .bind(nueva_cancion.id_album)
            .fetch_one(&self.pool)
            .await?;

        Ok(Canciones {
            id_cancion: fila.get("id_cancion"),
            nombre: fila.get("nombre"),
            duracion: fila.get("duracion"),
            id_album: fila.get("id_album"),
        })
    }

    pub async fn actualizar_cancion(&self, id_cancion: i32, actualizar_cancion: ActualizarCancion) -> sqlx::Result<Canciones> {
        let fila = sqlx::query("UPDATE Canciones SET nombre = $1, duracion = $2, id_album = $3 WHERE id_cancion = $4 RETURNING id_cancion, nombre, duracion, id_album")
            .bind(&actualizar_cancion.nombre)
            .bind(actualizar_cancion.duracion)
            .bind(actualizar_cancion.id_album)
            .bind(id_cancion)
            .fetch_one(&self.pool)
            .await?;

        Ok(Canciones {
            id_cancion: fila.get("id_cancion"),
            nombre: fila.get("nombre"),
            duracion: fila.get("duracion"),
            id_album: fila.get("id_album"),
        })
    }

    pub async fn eliminar_cancion(&self, id_cancion: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Canciones WHERE id_cancion = $1")
            .bind(id_cancion)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

}