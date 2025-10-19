mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod websocket;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use log::info;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;

    info!(
        "Starting ClassTop Management Server v{}",
        config.app_version
    );
    info!("Database: PostgreSQL");

    // Initialize database connection pool
    let db_pool = db::create_pool(&config.database_url).await?;

    // Run migrations
    info!("Running database migrations...");
    db::run_migrations(&db_pool).await?;
    info!("Migrations completed successfully");

    let bind_address = format!("{}:{}", config.host, config.port);
    info!("Server starting on http://{}", bind_address);

    // Initialize WebSocket connection manager
    let ws_manager = actix_web::web::Data::new(websocket::WSConnectionManager::new());

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(ws_manager.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Root endpoint
            .route("/", web::get().to(routes::root))
            // WebSocket endpoint
            .route("/ws", web::get().to(websocket::ws_endpoint))
            // WebSocket control endpoints
            .route(
                "/api/control/command",
                web::post().to(websocket::send_command),
            )
            .route(
                "/api/control/status",
                web::get().to(websocket::get_connections_status),
            )
            // API routes
            .service(web::scope("/api").configure(routes::configure_routes))
            // Swagger UI
            .service(
                SwaggerUi::new("/api/docs/{_:.*}")
                    .url("/api/openapi.json", routes::ApiDoc::openapi()),
            )
            // ReDoc
            .service(Redoc::with_url("/api/redoc", routes::ApiDoc::openapi()))
            // Static files
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await?;

    Ok(())
}
