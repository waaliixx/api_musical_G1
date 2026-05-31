mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

//cambiar por los controllers que tengas

use controller::playlists_controller::playlists_router;
use controller::canciones_controller::canciones_router;
use controller::artistas_controller::artistas_router;

use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}


fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    let mut router1 = playlists_router(pool.clone());
    let router2 = canciones_router(pool.clone());
    router1.merge(router2)
}
