use sqlx::{PgPool, Row};
use crate::models::Albumes::{Albumes, NuevoAlbum, ActualizarAlbum};

pub struct AlbumesRepository{
    pool: PgPool,
}

impl AlbumesRepository{
    pub fn new (pool: PgPool) -> Self{
        Self { pool }
    }


    pub async fn obtener_albumes(&self) -> sqlx::Result<Vec<Albumes>>{
        let filas = sqlx::query("SELECT id_album, titulo, fecha_lanzamiento, id_artista FROM Albumes")
        .fetch_all(&self.pool)
        .await?;

         let Albumes = filas.into_iter().map(|fila| {
            Albumes {
            id_album: fila.get("id_album"),
            titulo: fila.get("titulo"),
            fecha_lanzamiento: fila.get("fecha_lanzamiento"),
            id_artista: fila.get("id_artista"),
         }
         }).collect();
         Ok(Albumes)
        }   

    pub async fn crear_album(&self, Nuevo_Album: NuevoAlbum) -> sqlx::Result<Albumes>{
         let fila = sqlx::query("INSERT INTO Albumes(titulo, fecha_lanzamiento, id_artista) VALUES ($1, $2, $3) RETURNING id_album, titulo, fecha_lanzamiento,id_artista")
             .bind(Nuevo_Album.titulo)
             .bind(Nuevo_Album.fecha_lanzamiento)
             .bind(Nuevo_Album.id_artista)
             .fetch_one(&self.pool)
             .await?;
         Ok(Albumes{
             id_album: fila.get("id_album"),
             titulo: fila.get("titulo"),
             fecha_lanzamiento: fila.get("fecha_lanzamiento"),
              id_artista: fila.get("id_artista"),
         })
    } 

    pub async fn actualizar_album(&self, id_album: i32, album_actualizado: ActualizarAlbum) -> sqlx::Result<Albumes> {
        let fila = sqlx::query("UPDATE Albumes SET titulo = $1, fecha_lanzamiento = $2, id_artista= $3 WHERE id_album = $4 RETURNING id_album, titulo, fecha_lanzamiento, id_artista")
         .bind(album_actualizado.titulo)
         .bind(album_actualizado.fecha_lanzamiento)
         .bind(album_actualizado.id_artista)
         .bind(id_album)
         .fetch_one(&self.pool)
         .await?;

        Ok(Albumes{
             id_album: fila.get("id_album"),
             titulo: fila.get("titulo"),
             fecha_lanzamiento: fila.get("fecha_lanzamiento"),
             id_artista: fila.get("id_artista"),
         })

    }

    pub async fn eliminar_album(&self, id_album: i32) -> sqlx::Result<()>{
         sqlx::query("DELETE FROM Albumes WHERE id_album = $1")
             .bind(id_album)
             .execute(&self.pool)
             .await?;
         Ok(())
    }

}