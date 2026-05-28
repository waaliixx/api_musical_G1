use sqlx:: {PgPool, Row};
use crate::models::Usuarios_Streaming::{ActualizarUsuarioStreaming, NuevoUsuarioStreaming, UsuariosStreaming};

pub struct UsuariosStreamingRepository {
    pool: PgPool,
}
impl UsuariosStreamingRepository {
    pub fn nuevo(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_usuarios_streaming(&self) -> sqlx::Result<Vec<UsuariosStreaming>> {
        let filas = sqlx::query("SELECT id_usuario, nombre_usuario, tipo_suscripcion FROM usuarios_streaming")
            .fetch_all(&self.pool)
            .await?;

        let usuarios_streaming = filas.into_iter().map(|fila| UsuariosStreaming {
            id_usuario: fila.get("id_usuario"),
            nombre_usuario: fila.get("nombre_usuario"),
            tipo_suscripcion: fila.get("tipo_suscripcion"),
        }).collect();
        Ok(usuarios_streaming)
    }

    pub async fn crear_usuario_streaming(&self, nuevo_usuario: NuevoUsuarioStreaming) -> sqlx::Result<UsuariosStreaming> {
        let fila = sqlx::query("INSERT INTO usuarios_streaming (id_usuario, nombre_usuario, tipo_suscripcion) VALUES ($1, $2, $3) RETURNING id_usuario, nombre_usuario, tipo_suscripcion")
            .bind(nuevo_usuario.id_usuario)
            .bind(&nuevo_usuario.nombre_usuario)
            .bind(&nuevo_usuario.tipo_suscripcion)
            .fetch_one(&self.pool)
            .await?;

        Ok(UsuariosStreaming {
            id_usuario: fila.get("id_usuario"),
            nombre_usuario: fila.get("nombre_usuario"),
            tipo_suscripcion: fila.get("tipo_suscripcion"),
        })
    }

    pub async fn actualizar_usuario_streaming(&self, id_usuario: i32, actualizar_usuario: ActualizarUsuarioStreaming) -> sqlx::Result<UsuariosStreaming> {
        let fila = sqlx::query("UPDATE usuarios_streaming SET nombre_usuario = $1, tipo_suscripcion = $2 WHERE id_usuario = $3 RETURNING id_usuario, nombre_usuario, tipo_suscripcion")
            .bind(&actualizar_usuario.nombre_usuario)
            .bind(&actualizar_usuario.tipo_suscripcion)
            .bind(id_usuario)
            .fetch_one(&self.pool)
            .await?;

        Ok(UsuariosStreaming {
            id_usuario: fila.get("id_usuario"),
            nombre_usuario: fila.get("nombre_usuario"),
            tipo_suscripcion: fila.get("tipo_suscripcion"),
        })
    }
}