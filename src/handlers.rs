use crate::db::{repository::Repository, DbPool};
use crate::error::AppResult;
use crate::models::*;
use actix_web::{web, HttpResponse};
use chrono::Utc;

// Health check handler
#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Health check successful", body = ApiResponse<HealthResponse>)
    ),
    tag = "System"
)]
pub async fn health_check() -> AppResult<HttpResponse> {
    let response = ApiResponse::new(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    });
    Ok(HttpResponse::Ok().json(response))
}

// Client management handlers
#[utoipa::path(
    get,
    path = "/api/clients",
    responses(
        (status = 200, description = "List of all clients", body = ApiResponse<Vec<Client>>)
    ),
    tag = "Clients"
)]
pub async fn get_clients(pool: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let clients = repo.get_all_clients().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(clients)))
}

#[utoipa::path(
    get,
    path = "/api/clients/{id}",
    params(
        ("id" = i32, Path, description = "Client ID")
    ),
    responses(
        (status = 200, description = "Client found", body = ApiResponse<Client>),
        (status = 404, description = "Client not found")
    ),
    tag = "Clients"
)]
pub async fn get_client(pool: web::Data<DbPool>, id: web::Path<i32>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let client = repo.get_client_by_id(*id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(client)))
}

#[utoipa::path(
    post,
    path = "/api/clients/register",
    request_body = RegisterClient,
    responses(
        (status = 200, description = "Client registered", body = ApiResponse<Client>),
        (status = 400, description = "Bad request")
    ),
    tag = "Clients"
)]
pub async fn register_client(
    pool: web::Data<DbPool>,
    client: web::Json<RegisterClient>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let registered = repo.register_client(client.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(registered)))
}

#[utoipa::path(
    put,
    path = "/api/clients/{id}",
    params(
        ("id" = i32, Path, description = "Client ID")
    ),
    request_body = UpdateClient,
    responses(
        (status = 200, description = "Client updated", body = ApiResponse<MessageResponse>),
        (status = 404, description = "Client not found")
    ),
    tag = "Clients"
)]
pub async fn update_client(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    client: web::Json<UpdateClient>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    repo.update_client(*id, client.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(MessageResponse {
        message: "Client updated".to_string(),
    })))
}

#[utoipa::path(
    delete,
    path = "/api/clients/{id}",
    params(
        ("id" = i32, Path, description = "Client ID")
    ),
    responses(
        (status = 200, description = "Client deleted", body = ApiResponse<MessageResponse>),
        (status = 404, description = "Client not found")
    ),
    tag = "Clients"
)]
pub async fn delete_client(pool: web::Data<DbPool>, id: web::Path<i32>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    repo.delete_client(*id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(MessageResponse {
        message: "Client deleted".to_string(),
    })))
}

// Client data handlers
#[utoipa::path(
    get,
    path = "/api/clients/{id}/courses",
    params(
        ("id" = i32, Path, description = "Client ID")
    ),
    responses(
        (status = 200, description = "List of client courses", body = ApiResponse<Vec<Course>>)
    ),
    tag = "Clients"
)]
pub async fn get_client_courses(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let courses = repo.get_client_courses(*id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(courses)))
}

#[utoipa::path(
    get,
    path = "/api/clients/{id}/schedule",
    params(
        ("id" = i32, Path, description = "Client ID")
    ),
    responses(
        (status = 200, description = "List of client schedule entries", body = ApiResponse<Vec<ScheduleEntry>>)
    ),
    tag = "Clients"
)]
pub async fn get_client_schedule(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let schedule = repo.get_client_schedule(*id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(schedule)))
}

// Sync handler
#[utoipa::path(
    post,
    path = "/api/sync",
    request_body = SyncRequest,
    responses(
        (status = 200, description = "Data synced successfully", body = SyncResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Client not found")
    ),
    tag = "Sync"
)]
pub async fn sync_data(
    pool: web::Data<DbPool>,
    request: web::Json<SyncRequest>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let req = request.into_inner();

    let response = repo
        .sync_client_data(&req.client_uuid, req.courses, req.schedule_entries)
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

// Statistics handlers
#[utoipa::path(
    get,
    path = "/api/statistics",
    responses(
        (status = 200, description = "Statistics", body = ApiResponse<Statistics>)
    ),
    tag = "Statistics"
)]
pub async fn get_statistics(pool: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let stats = repo.get_statistics().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(stats)))
}

#[utoipa::path(
    get,
    path = "/api/statistics/clients",
    responses(
        (status = 200, description = "Client statistics", body = ApiResponse<Vec<ClientStatistics>>)
    ),
    tag = "Statistics"
)]
pub async fn get_client_statistics(pool: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let stats = repo.get_client_statistics().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(stats)))
}

// Settings handlers
#[utoipa::path(
    get,
    path = "/api/settings",
    responses(
        (status = 200, description = "All settings", body = ApiResponse<std::collections::HashMap<String, String>>)
    ),
    tag = "Settings"
)]
pub async fn get_settings(pool: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let settings = repo.get_all_settings().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(settings)))
}

#[utoipa::path(
    get,
    path = "/api/settings/{key}",
    params(
        ("key" = String, Path, description = "Setting key")
    ),
    responses(
        (status = 200, description = "Setting found", body = ApiResponse<Setting>),
        (status = 404, description = "Setting not found")
    ),
    tag = "Settings"
)]
pub async fn get_setting(
    pool: web::Data<DbPool>,
    key: web::Path<String>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let setting = repo.get_setting(&key).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(setting)))
}

#[utoipa::path(
    put,
    path = "/api/settings/{key}",
    params(
        ("key" = String, Path, description = "Setting key")
    ),
    request_body = UpdateSetting,
    responses(
        (status = 200, description = "Setting updated", body = ApiResponse<MessageResponse>)
    ),
    tag = "Settings"
)]
pub async fn update_setting(
    pool: web::Data<DbPool>,
    key: web::Path<String>,
    value: web::Json<UpdateSetting>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    repo.update_setting(&key, &value.value).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(MessageResponse {
        message: "Setting updated".to_string(),
    })))
}
