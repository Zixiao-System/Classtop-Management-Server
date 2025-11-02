use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

// Client (管理的客户端设备)
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Client {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub api_url: String, // 客户端的 API 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>, // 如果客户端需要认证
    #[schema(value_type = Option<String>, example = "2024-01-01T00:00:00")]
    pub last_sync: Option<NaiveDateTime>, // 最后同步时间
    pub status: String,  // online, offline, error
    #[schema(value_type = String, example = "2024-01-01T00:00:00")]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterClient {
    pub uuid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub api_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateClient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// Course model (从客户端同步的课程数据)
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Course {
    pub id: i32,
    pub client_id: i32,           // 所属客户端
    pub course_id_on_client: i32, // 客户端上的课程 ID
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCourse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCourse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

// Schedule Entry model (从客户端同步的课程表数据)
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ScheduleEntry {
    pub id: i32,
    pub client_id: i32,          // 所属客户端
    pub entry_id_on_client: i32, // 客户端上的条目 ID
    pub course_id: i32,          // 管理服务器上的课程 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weeks: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateScheduleEntry {
    pub course_id: i32,
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weeks: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

// Settings model
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSetting {
    pub value: String,
}

// Health check
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

// Week info
#[allow(dead_code)]
#[derive(Debug, Serialize, ToSchema)]
pub struct WeekInfo {
    pub week: i32,
    pub semester_start_date: String,
    pub is_calculated: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct SetSemesterStart {
    pub date: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, ToSchema)]
pub struct SemesterStartResponse {
    pub semester_start_date: String,
    pub calculated_week: i32,
}

// Statistics
#[derive(Debug, Serialize, ToSchema)]
pub struct Statistics {
    pub total_clients: i64,
    pub online_clients: i64,
    pub total_courses: i64,
    pub total_schedule_entries: i64,
}

// Client Statistics
#[derive(Debug, Serialize, ToSchema)]
pub struct ClientStatistics {
    pub client_id: i32,
    pub client_name: String,
    pub total_courses: i64,
    pub total_schedule_entries: i64,
    pub last_sync: Option<String>,
}

// Sync Request (客户端主动同步数据到服务器)
#[derive(Debug, Deserialize, ToSchema)]
pub struct SyncRequest {
    pub client_uuid: String,
    pub courses: Vec<ClientCourse>,
    pub schedule_entries: Vec<ClientScheduleEntry>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ClientCourse {
    pub id: i32, // 客户端上的 ID
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ClientScheduleEntry {
    pub id: i32,        // 客户端上的 ID
    pub course_id: i32, // 客户端上的课程 ID
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weeks: Option<Vec<i32>>,
}

// Sync Response
#[derive(Debug, Serialize, ToSchema)]
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
    pub synced_courses: i32,
    pub synced_entries: i32,
}

// Logs
#[allow(dead_code)]
#[derive(Debug, Serialize, ToSchema)]
pub struct LogsResponse {
    pub lines: Vec<String>,
}

// Root response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RootResponse {
    pub message: String,
    pub version: String,
    pub docs: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

// LMS (Light Management Service) Models
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct LMSInstance {
    pub id: String, // UUID
    pub lms_uuid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    pub port: i32,
    pub status: String, // online, offline, error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat: Option<String>,
    pub client_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterLMSRequest {
    pub lms_uuid: String,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub version: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterLMSResponse {
    pub lms_id: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LMSHeartbeatRequest {
    pub lms_uuid: String,
    pub client_count: i32,
    pub clients: Vec<LMSClientInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct LMSClientInfo {
    pub uuid: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LMSStatistics {
    pub total_lms_instances: i64,
    pub online_lms_instances: i64,
    pub total_clients_managed_by_lms: i64,
}

// CCTV Models
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CCTVConfig {
    pub id: String, // UUID
    pub client_id: i32,
    pub camera_id: String,
    pub camera_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp_url: Option<String>,
    pub recording_enabled: bool,
    pub streaming_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCCTVRequest {
    pub client_id: i32,
    pub camera_id: String,
    pub camera_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp_url: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCCTVRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_enabled: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, ToSchema)]
pub struct CCTVEvent {
    pub id: i32,
    pub camera_config_id: String,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub created_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct LogCCTVEventRequest {
    pub camera_config_id: String,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

// User Authentication Models
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: Option<String>,
    pub role: String, // admin, user
    pub is_active: bool,
    #[schema(value_type = String, example = "2024-01-01T00:00:00")]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, example = "2024-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            uuid: user.uuid,
            username: user.username,
            email: user.email,
            role: user.role,
        }
    }
}

// Pagination Models
#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> i64 {
        self.page_size
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationInfo {
    pub page: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

impl PaginationInfo {
    pub fn new(page: i64, page_size: i64, total_items: i64) -> Self {
        let total_pages = (total_items + page_size - 1) / page_size;
        Self {
            page,
            page_size,
            total_items,
            total_pages,
        }
    }
}
