use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
    pub last_sync: Option<String>, // 最后同步时间
    pub status: String,  // online, offline, error
    pub created_at: String,
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
#[derive(Debug, Serialize, ToSchema)]
pub struct WeekInfo {
    pub week: i32,
    pub semester_start_date: String,
    pub is_calculated: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetSemesterStart {
    pub date: String,
}

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
#[derive(Debug, Serialize, ToSchema)]
pub struct LogsResponse {
    pub lines: Vec<String>,
}

// Root response
#[derive(Debug, Serialize, ToSchema)]
pub struct RootResponse {
    pub message: String,
    pub version: String,
    pub docs: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}
