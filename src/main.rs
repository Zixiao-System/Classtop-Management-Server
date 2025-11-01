use classtop_management_server::{config, db, routes, websocket};

use actix_cors::Cors;
use actix_files::Files;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{http::header, middleware, web, App, HttpServer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,actix_web=debug,sqlx=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;
    let config_data = web::Data::new(config.clone());

    info!(
        version = config.app_version,
        "Starting ClassTop Management Server"
    );
    info!("Database: PostgreSQL");
    info!(
        auth_enabled = config.enable_auth,
        "Authentication configuration loaded"
    );

    // Initialize database connection pool
    let db_pool = db::create_pool(&config.database_url).await?;

    // Run migrations
    info!("Running database migrations...");
    db::run_migrations(&db_pool).await?;
    info!("Migrations completed successfully");

    let bind_address = format!("{}:{}", config.host, config.port);
    info!(address = %bind_address, "Server starting");

    // Initialize WebSocket connection manager
    let ws_manager = actix_web::web::Data::new(websocket::WSConnectionManager::new());

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = if config.enable_auth {
            // Production-ready CORS configuration
            let mut cors_builder = Cors::default()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ])
                .max_age(3600);

            // Add allowed origins
            for origin in &config.cors_allowed_origins {
                cors_builder = cors_builder.allowed_origin(origin);
            }

            cors_builder
        } else {
            // Development mode - permissive CORS
            Cors::permissive()
        };

        // Configure rate limiting (100 burst requests, then limited)
        #[allow(deprecated)]
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(2) // Allow 2 requests per second
            .burst_size(100)
            .finish()
            .unwrap();

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(config_data.clone())
            .app_data(ws_manager.clone())
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            .wrap(tracing_actix_web::TracingLogger::default())
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
