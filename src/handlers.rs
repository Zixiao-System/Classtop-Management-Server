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

// LMS Management handlers
#[utoipa::path(
    post,
    path = "/api/lms/register",
    request_body = RegisterLMSRequest,
    responses(
        (status = 200, description = "LMS registered successfully", body = ApiResponse<RegisterLMSResponse>),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "LMS Management"
)]
pub async fn register_lms(
    pool: web::Data<DbPool>,
    req: web::Json<RegisterLMSRequest>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());

    // Generate API Key
    let api_key = generate_api_key();

    // Register LMS
    let lms_id = repo
        .register_lms(
            &req.lms_uuid,
            &req.name,
            &req.host,
            req.port,
            &api_key,
            &req.version,
        )
        .await?;

    let response = RegisterLMSResponse { lms_id, api_key };

    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

#[utoipa::path(
    post,
    path = "/api/lms/heartbeat",
    request_body = LMSHeartbeatRequest,
    responses(
        (status = 200, description = "Heartbeat received"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "LMS not found")
    ),
    tag = "LMS Management"
)]
pub async fn lms_heartbeat(
    pool: web::Data<DbPool>,
    req: web::Json<LMSHeartbeatRequest>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());

    // Update LMS heartbeat
    repo.update_lms_heartbeat(&req.lms_uuid, req.client_count, &req.clients)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(MessageResponse {
        message: "Heartbeat received".to_string(),
    })))
}

#[utoipa::path(
    get,
    path = "/api/lms",
    responses(
        (status = 200, description = "List of LMS instances", body = ApiResponse<Vec<LMSInstance>>)
    ),
    tag = "LMS Management"
)]
pub async fn list_lms(pool: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let instances = repo.get_all_lms_instances().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(instances)))
}

#[utoipa::path(
    get,
    path = "/api/lms/{lms_id}",
    params(
        ("lms_id" = String, Path, description = "LMS instance ID")
    ),
    responses(
        (status = 200, description = "LMS instance details", body = ApiResponse<LMSInstance>),
        (status = 404, description = "LMS not found")
    ),
    tag = "LMS Management"
)]
pub async fn get_lms(
    pool: web::Data<DbPool>,
    lms_id: web::Path<String>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let instance = repo.get_lms_by_id(&lms_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(instance)))
}

#[utoipa::path(
    get,
    path = "/api/lms/{lms_id}/clients",
    params(
        ("lms_id" = String, Path, description = "LMS instance ID")
    ),
    responses(
        (status = 200, description = "List of clients managed by this LMS", body = ApiResponse<Vec<Client>>)
    ),
    tag = "LMS Management"
)]
pub async fn get_lms_clients(
    pool: web::Data<DbPool>,
    lms_id: web::Path<String>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let clients = repo.get_clients_by_lms(&lms_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(clients)))
}

#[utoipa::path(
    delete,
    path = "/api/lms/{lms_id}",
    params(
        ("lms_id" = String, Path, description = "LMS instance ID")
    ),
    responses(
        (status = 200, description = "LMS deleted successfully"),
        (status = 404, description = "LMS not found")
    ),
    tag = "LMS Management"
)]
pub async fn delete_lms(
    pool: web::Data<DbPool>,
    lms_id: web::Path<String>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    repo.delete_lms(&lms_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(MessageResponse {
        message: "LMS deleted".to_string(),
    })))
}

#[utoipa::path(
    get,
    path = "/api/lms/statistics",
    responses(
        (status = 200, description = "LMS statistics", body = ApiResponse<LMSStatistics>)
    ),
    tag = "LMS Management"
)]
pub async fn get_lms_statistics(pool: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let stats = repo.get_lms_statistics().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(stats)))
}

// Helper function to generate API key
fn generate_api_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// Authentication handlers
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterUser,
    responses(
        (status = 200, description = "User registered successfully", body = ApiResponse<UserInfo>),
        (status = 400, description = "Bad request - username already exists")
    ),
    tag = "Authentication"
)]
pub async fn register(
    pool: web::Data<DbPool>,
    config: web::Data<crate::config::Config>,
    user_data: web::Json<RegisterUser>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());

    // Hash password
    let password_hash = crate::auth::hash_password(&user_data.password)
        .map_err(|e| crate::error::AppError::Internal(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = repo
        .create_user(
            &uuid::Uuid::new_v4().to_string(),
            &user_data.username,
            &password_hash,
            user_data.email.as_deref(),
            "user", // Default role
        )
        .await?;

    // Generate token
    let token = crate::auth::generate_token(
        uuid::Uuid::parse_str(&user.uuid).unwrap(),
        user.username.clone(),
        &config.jwt_secret,
    )
    .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;

    let response = LoginResponse {
        token,
        user: user.into(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<LoginResponse>),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Authentication"
)]
pub async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<crate::config::Config>,
    credentials: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());

    // Get user by username
    let user = repo
        .get_user_by_username(&credentials.username)
        .await
        .map_err(|_| {
            crate::error::AppError::BadRequest("Invalid username or password".to_string())
        })?;

    // Verify password
    let is_valid = crate::auth::verify_password(&credentials.password, &user.password_hash)
        .map_err(|e| {
            crate::error::AppError::Internal(format!("Password verification failed: {}", e))
        })?;

    if !is_valid {
        return Err(crate::error::AppError::BadRequest(
            "Invalid username or password".to_string(),
        ));
    }

    if !user.is_active {
        return Err(crate::error::AppError::BadRequest(
            "User account is disabled".to_string(),
        ));
    }

    // Generate token
    let token = crate::auth::generate_token(
        uuid::Uuid::parse_str(&user.uuid).unwrap(),
        user.username.clone(),
        &config.jwt_secret,
    )
    .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;

    let response = LoginResponse {
        token,
        user: user.into(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

// Pagination handlers
#[utoipa::path(
    get,
    path = "/api/clients/paginated",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("page_size" = Option<i64>, Query, description = "Page size (default: 20)")
    ),
    responses(
        (status = 200, description = "Paginated list of clients", body = ApiResponse<PaginatedResponse<Client>>)
    ),
    tag = "Clients"
)]
pub async fn get_clients_paginated(
    pool: web::Data<DbPool>,
    params: web::Query<PaginationParams>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let (clients, total) = repo
        .get_clients_paginated(params.offset(), params.limit())
        .await?;

    let response = PaginatedResponse {
        data: clients,
        pagination: PaginationInfo::new(params.page, params.page_size, total),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

#[utoipa::path(
    get,
    path = "/api/courses/paginated",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("page_size" = Option<i64>, Query, description = "Page size (default: 20)")
    ),
    responses(
        (status = 200, description = "Paginated list of courses", body = ApiResponse<PaginatedResponse<Course>>)
    ),
    tag = "Courses"
)]
pub async fn get_courses_paginated(
    pool: web::Data<DbPool>,
    params: web::Query<PaginationParams>,
) -> AppResult<HttpResponse> {
    let repo = Repository::new(pool.get_ref().clone());
    let (courses, total) = repo
        .get_courses_paginated(params.offset(), params.limit())
        .await?;

    let response = PaginatedResponse {
        data: courses,
        pagination: PaginationInfo::new(params.page, params.page_size, total),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}
