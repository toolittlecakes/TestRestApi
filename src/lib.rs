#[macro_use]
extern crate log;

use futures::executor::block_on;

pub mod image;
pub mod server;

/// Run Rest-API.
/// This function initialises database, server with routes
/// and create SIGINT handle, which stops server gracefully
///
/// # Errors
/// Will return error if host or port is not valid for binding
///
/// # Panics
/// Will panic if database initialization fails
///
pub async fn run(host: String, port: String) -> std::io::Result<()> {
    let server = actix_web::HttpServer::new(||
        actix_web::App::new()
            .configure(server::init_routes)
    )
        .bind(format!("{}:{}", host, port))?
        .shutdown_timeout(60)
        .disable_signals()
        .run();

    info!("Server started");
    let srv = server.clone();

    // In case we need gracefull shutdown only with SIGTERM signal and common shutdown with SIGINT,
    // we don't need the code below. Just delete line ".disable_signals()" in creating server,
    // because it's default behavior of actix server (graceful shutdown only with SIGTERM).
    ctrlc::set_handler(move || {
        block_on(srv.stop(true));
        info!("Server stopped")
    }).expect("SIGINT handler failed");

    server.await
}