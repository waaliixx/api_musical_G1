use sqlx:: {PgPool, Row};
use crate::models::Artistas::{ActualizarArtista, NuevoArtista, Artistas};

pub struct ArtistasRepository{
    pool: PgPool,
}

impl ArtistasRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_artistas(&self) -> sqlx::Result<Vec<Artistas>> {
        let filas = sqlx::query("SELECT id_artista, nombre_artistico, genero_principal FROM artistas")
            .fetch_all(&self.pool)
            .await?;

        let artistas = filas.into_iter().map(|fila| Artistas {
            id_artista: fila.get("id_artista"),
            nombre_artistico: fila.get("nombre_artistico"),
            genero_principal: fila.get("genero_principal"),
        }).collect();
        Ok(artistas)
    }

    pub async fn crear_artista(&self, nuevo_artista: NuevoArtista) -> sqlx::Result<Artistas> {
        let fila = sqlx::query("INSERT INTO artistas (id_artista, nombre_artistico, genero_principal) VALUES ($1, $2, $3) RETURNING id_artista, nombre_artistico, genero_principal")
            .bind(nuevo_artista.id_artista)
            .bind(&nuevo_artista.nombre_artistico)
            .bind(&nuevo_artista.genero_principal)
            .fetch_one(&self.pool)
            .await?;

        Ok(Artistas {
            id_artista: fila.get("id_artista"),
            nombre_artistico: fila.get("nombre_artistico"),
            genero_principal: fila.get("genero_principal"),
        })
    }

    pub async fn actualizar_artista(&self, id_artista: i32, actualizar_Artista: ActualizarArtista) -> sqlx::Result<Artistas> {
        let fila = sqlx::query("UPDATE artistas SET nombre_artistico = $1, genero_principal = $2 WHERE id_artista = $3 RETURNING id_artista, nombre_artistico, genero_principal")
            .bind(&actualizar_Artista.nombre_artistico)
            .bind(&actualizar_Artista.genero_principal)
            .bind(id_artista)
            .fetch_one(&self.pool)
            .await?;

        Ok(Artistas {
            id_artista: fila.get("id_artista"),
            nombre_artistico: fila.get("nombre_artistico"),
            genero_principal: fila.get("genero_principal"),
        })
    }

    pub async fn eliminar_artista(&self, id_artista: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM artistas WHERE id_artista = $1")
            .bind(id_artista)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}