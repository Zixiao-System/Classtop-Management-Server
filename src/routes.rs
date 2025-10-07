use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use crate::handlers;
use crate::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health_check,
        handlers::get_clients,
        handlers::get_client,
        handlers::register_client,
        handlers::update_client,
        handlers::delete_client,
        handlers::get_client_courses,
        handlers::get_client_schedule,
        handlers::sync_data,
        handlers::get_statistics,
        handlers::get_client_statistics,
        handlers::get_settings,
        handlers::get_setting,
        handlers::update_setting,
    ),
    components(
        schemas(
            ApiResponse<HealthResponse>,
            ApiResponse<Vec<Client>>,
            ApiResponse<Client>,
            ApiResponse<Vec<Course>>,
            ApiResponse<Vec<ScheduleEntry>>,
            ApiResponse<MessageResponse>,
            ApiResponse<Statistics>,
            ApiResponse<Vec<ClientStatistics>>,
            ApiResponse<Setting>,
            HealthResponse,
            Client,
            RegisterClient,
            UpdateClient,
            Course,
            ScheduleEntry,
            SyncRequest,
            SyncResponse,
            ClientCourse,
            ClientScheduleEntry,
            Statistics,
            ClientStatistics,
            Setting,
            UpdateSetting,
            MessageResponse,
            RootResponse,
        )
    ),
    tags(
        (name = "System", description = "System endpoints"),
        (name = "Clients", description = "Client management"),
        (name = "Sync", description = "Data synchronization"),
        (name = "Statistics", description = "Statistics"),
        (name = "Settings", description = "Settings management"),
    ),
    info(
        title = "ClassTop Management Server API",
        version = "1.0.0",
        description = "Centralized management server for ClassTop clients"
    )
)]
pub struct ApiDoc;

pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(RootResponse {
        message: "ClassTop Management Server".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        docs: "/api/docs".to_string(),
    })
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health
        .route("/health", web::get().to(handlers::health_check))
        // Clients
        .service(
            web::scope("/clients")
                .route("", web::get().to(handlers::get_clients))
                .route("/register", web::post().to(handlers::register_client))
                .route("/{id}", web::get().to(handlers::get_client))
                .route("/{id}", web::put().to(handlers::update_client))
                .route("/{id}", web::delete().to(handlers::delete_client))
                .route("/{id}/courses", web::get().to(handlers::get_client_courses))
                .route("/{id}/schedule", web::get().to(handlers::get_client_schedule))
        )
        // Sync
        .route("/sync", web::post().to(handlers::sync_data))
        // Statistics
        .service(
            web::scope("/statistics")
                .route("", web::get().to(handlers::get_statistics))
                .route("/clients", web::get().to(handlers::get_client_statistics))
        )
        // Settings
        .service(
            web::scope("/settings")
                .route("", web::get().to(handlers::get_settings))
                .route("/{key}", web::get().to(handlers::get_setting))
                .route("/{key}", web::put().to(handlers::update_setting))
        );
}
