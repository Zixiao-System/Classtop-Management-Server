mod config;
mod db;
mod models;
mod routes;
mod handlers;
mod error;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use actix_files::Files;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
use log::info;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;

    info!("Starting ClassTop Management Server v{}", config.app_version);
    info!("Database: {}", if config.database_url.contains("postgres") { "PostgreSQL" } else { "SQL Server" });

    // Initialize database connection pool
    let db_pool = db::create_pool(&config.database_url).await?;

    // Run migrations
    info!("Running database migrations...");
    db::run_migrations(&db_pool).await?;
    info!("Migrations completed successfully");

    let bind_address = format!("{}:{}", config.host, config.port);
    info!("Server starting on http://{}", bind_address);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Root endpoint
            .route("/", web::get().to(routes::root))
            // API routes
            .service(
                web::scope("/api")
                    .configure(routes::configure_routes)
            )
            // Swagger UI
            .service(
                SwaggerUi::new("/api/docs/{_:.*}")
                    .url("/api/openapi.json", routes::ApiDoc::openapi())
            )
            // ReDoc
            .service(
                Redoc::with_url("/api/redoc", routes::ApiDoc::openapi())
            )
            // Static files
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await?;

    Ok(())
}
