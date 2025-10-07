use sqlx::{Pool, Postgres, Any, AnyPool};
use sqlx::any::AnyPoolOptions;
use anyhow::Result;
use std::time::Duration;

pub type DbPool = AnyPool;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let pool = AnyPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    let database_url = std::env::var("DATABASE_URL")?;

    if database_url.contains("postgres") {
        let pg_pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await?;

        sqlx::query(include_str!("../migrations/001_initial_postgresql.sql"))
            .execute(&pg_pool)
            .await
            .ok();
    } else {
        log::warn!("Only PostgreSQL is currently supported");
    }

    Ok(())
}

// Repository for database operations
pub mod repository {
    use super::*;
    use crate::models::*;
    use crate::error::{AppError, AppResult};
    use sqlx::Row;
    use chrono::Utc;

    pub struct Repository {
        pool: DbPool,
    }

    impl Repository {
        pub fn new(pool: DbPool) -> Self {
            Self { pool }
        }

        // Client operations
        pub async fn get_all_clients(&self) -> AppResult<Vec<Client>> {
            let rows = sqlx::query(
                "SELECT id, uuid, name, description, api_url, api_key,
                        last_sync, status, created_at
                 FROM clients ORDER BY created_at DESC"
            )
            .fetch_all(&self.pool)
            .await?;

            let clients = rows.iter().map(|row| {
                Client {
                    id: row.get("id"),
                    uuid: row.get("uuid"),
                    name: row.get("name"),
                    description: row.try_get("description").ok(),
                    api_url: row.get("api_url"),
                    api_key: row.try_get("api_key").ok(),
                    last_sync: row.try_get::<String, _>("last_sync").ok(),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                }
            }).collect();

            Ok(clients)
        }

        pub async fn get_client_by_id(&self, id: i32) -> AppResult<Client> {
            let row = sqlx::query(
                "SELECT id, uuid, name, description, api_url, api_key,
                        last_sync, status, created_at
                 FROM clients WHERE id = $1"
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

            match row {
                Some(row) => Ok(Client {
                    id: row.get("id"),
                    uuid: row.get("uuid"),
                    name: row.get("name"),
                    description: row.try_get("description").ok(),
                    api_url: row.get("api_url"),
                    api_key: row.try_get("api_key").ok(),
                    last_sync: row.try_get::<String, _>("last_sync").ok(),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                }),
                None => Err(AppError::NotFound("Client not found".to_string())),
            }
        }

        pub async fn get_client_by_uuid(&self, uuid: &str) -> AppResult<Client> {
            let row = sqlx::query(
                "SELECT id, uuid, name, description, api_url, api_key,
                        last_sync, status, created_at
                 FROM clients WHERE uuid = $1"
            )
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await?;

            match row {
                Some(row) => Ok(Client {
                    id: row.get("id"),
                    uuid: row.get("uuid"),
                    name: row.get("name"),
                    description: row.try_get("description").ok(),
                    api_url: row.get("api_url"),
                    api_key: row.try_get("api_key").ok(),
                    last_sync: row.try_get::<String, _>("last_sync").ok(),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                }),
                None => Err(AppError::NotFound("Client not found".to_string())),
            }
        }

        pub async fn register_client(&self, client: RegisterClient) -> AppResult<Client> {
            let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

            let row = sqlx::query(
                "INSERT INTO clients (uuid, name, description, api_url, api_key, status, created_at)
                 VALUES ($1, $2, $3, $4, $5, 'offline', $6)
                 RETURNING id, uuid, name, description, api_url, api_key, last_sync, status, created_at"
            )
            .bind(&client.uuid)
            .bind(&client.name)
            .bind(&client.description)
            .bind(&client.api_url)
            .bind(&client.api_key)
            .bind(&now)
            .fetch_one(&self.pool)
            .await?;

            Ok(Client {
                id: row.get("id"),
                uuid: row.get("uuid"),
                name: row.get("name"),
                description: row.try_get("description").ok(),
                api_url: row.get("api_url"),
                api_key: row.try_get("api_key").ok(),
                last_sync: None,
                status: row.get("status"),
                created_at: row.get("created_at"),
            })
        }

        pub async fn update_client(&self, id: i32, client: UpdateClient) -> AppResult<()> {
            let mut query = String::from("UPDATE clients SET ");
            let mut updates = Vec::new();
            let mut bind_index = 1;

            if client.name.is_some() {
                updates.push(format!("name = ${}", bind_index));
                bind_index += 1;
            }
            if client.description.is_some() {
                updates.push(format!("description = ${}", bind_index));
                bind_index += 1;
            }
            if client.api_url.is_some() {
                updates.push(format!("api_url = ${}", bind_index));
                bind_index += 1;
            }
            if client.api_key.is_some() {
                updates.push(format!("api_key = ${}", bind_index));
                bind_index += 1;
            }

            if updates.is_empty() {
                return Ok(());
            }

            query.push_str(&updates.join(", "));
            query.push_str(&format!(" WHERE id = ${}", bind_index));

            let mut q = sqlx::query(&query);

            if let Some(name) = client.name {
                q = q.bind(name);
            }
            if let Some(description) = client.description {
                q = q.bind(description);
            }
            if let Some(api_url) = client.api_url {
                q = q.bind(api_url);
            }
            if let Some(api_key) = client.api_key {
                q = q.bind(api_key);
            }
            q = q.bind(id);

            let result = q.execute(&self.pool).await?;

            if result.rows_affected() == 0 {
                return Err(AppError::NotFound("Client not found".to_string()));
            }

            Ok(())
        }

        pub async fn delete_client(&self, id: i32) -> AppResult<()> {
            let result = sqlx::query("DELETE FROM clients WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await?;

            if result.rows_affected() == 0 {
                return Err(AppError::NotFound("Client not found".to_string()));
            }

            Ok(())
        }

        pub async fn update_client_status(&self, id: i32, status: &str) -> AppResult<()> {
            sqlx::query("UPDATE clients SET status = $1 WHERE id = $2")
                .bind(status)
                .bind(id)
                .execute(&self.pool)
                .await?;

            Ok(())
        }

        pub async fn update_client_last_sync(&self, id: i32) -> AppResult<()> {
            let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

            sqlx::query("UPDATE clients SET last_sync = $1 WHERE id = $2")
                .bind(&now)
                .bind(id)
                .execute(&self.pool)
                .await?;

            Ok(())
        }

        // Sync operations
        pub async fn sync_client_data(&self, client_uuid: &str, courses: Vec<ClientCourse>, entries: Vec<ClientScheduleEntry>) -> AppResult<SyncResponse> {
            let client = self.get_client_by_uuid(client_uuid).await?;
            let client_id = client.id;

            let mut synced_courses = 0;
            let mut synced_entries = 0;

            // Sync courses
            for course in courses {
                let exists = sqlx::query(
                    "SELECT id FROM courses WHERE client_id = $1 AND course_id_on_client = $2"
                )
                .bind(client_id)
                .bind(course.id)
                .fetch_optional(&self.pool)
                .await?;

                if exists.is_some() {
                    // Update existing course
                    sqlx::query(
                        "UPDATE courses SET name = $1, teacher = $2, color = $3, note = $4, synced_at = $5
                         WHERE client_id = $6 AND course_id_on_client = $7"
                    )
                    .bind(&course.name)
                    .bind(&course.teacher)
                    .bind(&course.color)
                    .bind(&course.note)
                    .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                    .bind(client_id)
                    .bind(course.id)
                    .execute(&self.pool)
                    .await?;
                } else {
                    // Insert new course
                    sqlx::query(
                        "INSERT INTO courses (client_id, course_id_on_client, name, teacher, color, note)
                         VALUES ($1, $2, $3, $4, $5, $6)"
                    )
                    .bind(client_id)
                    .bind(course.id)
                    .bind(&course.name)
                    .bind(&course.teacher)
                    .bind(&course.color)
                    .bind(&course.note)
                    .execute(&self.pool)
                    .await?;
                }
                synced_courses += 1;
            }

            // Sync schedule entries
            for entry in entries {
                // Find the course_id in our database
                let course_row = sqlx::query(
                    "SELECT id FROM courses WHERE client_id = $1 AND course_id_on_client = $2"
                )
                .bind(client_id)
                .bind(entry.course_id)
                .fetch_optional(&self.pool)
                .await?;

                if let Some(course_row) = course_row {
                    let course_id: i32 = course_row.get("id");
                    let weeks_json = entry.weeks.map(|w| serde_json::to_string(&w).unwrap());

                    let exists = sqlx::query(
                        "SELECT id FROM schedule_entries WHERE client_id = $1 AND entry_id_on_client = $2"
                    )
                    .bind(client_id)
                    .bind(entry.id)
                    .fetch_optional(&self.pool)
                    .await?;

                    if exists.is_some() {
                        // Update existing entry
                        sqlx::query(
                            "UPDATE schedule_entries
                             SET course_id = $1, day_of_week = $2, start_time = $3, end_time = $4, weeks = $5, synced_at = $6
                             WHERE client_id = $7 AND entry_id_on_client = $8"
                        )
                        .bind(course_id)
                        .bind(entry.day_of_week)
                        .bind(&entry.start_time)
                        .bind(&entry.end_time)
                        .bind(&weeks_json)
                        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                        .bind(client_id)
                        .bind(entry.id)
                        .execute(&self.pool)
                        .await?;
                    } else {
                        // Insert new entry
                        sqlx::query(
                            "INSERT INTO schedule_entries (client_id, entry_id_on_client, course_id, day_of_week, start_time, end_time, weeks)
                             VALUES ($1, $2, $3, $4, $5, $6, $7)"
                        )
                        .bind(client_id)
                        .bind(entry.id)
                        .bind(course_id)
                        .bind(entry.day_of_week)
                        .bind(&entry.start_time)
                        .bind(&entry.end_time)
                        .bind(&weeks_json)
                        .execute(&self.pool)
                        .await?;
                    }
                    synced_entries += 1;
                }
            }

            // Update client last sync time and status
            self.update_client_last_sync(client_id).await?;
            self.update_client_status(client_id, "online").await?;

            // Log sync
            sqlx::query(
                "INSERT INTO sync_logs (client_id, sync_type, status, courses_count, entries_count)
                 VALUES ($1, 'full', 'success', $2, $3)"
            )
            .bind(client_id)
            .bind(synced_courses)
            .bind(synced_entries)
            .execute(&self.pool)
            .await?;

            Ok(SyncResponse {
                success: true,
                message: "Data synced successfully".to_string(),
                synced_courses,
                synced_entries,
            })
        }

        // Get client data
        pub async fn get_client_courses(&self, client_id: i32) -> AppResult<Vec<Course>> {
            let rows = sqlx::query(
                "SELECT id, client_id, course_id_on_client, name, teacher, color, note
                 FROM courses WHERE client_id = $1 ORDER BY name"
            )
            .bind(client_id)
            .fetch_all(&self.pool)
            .await?;

            let courses = rows.iter().map(|row| {
                Course {
                    id: row.get("id"),
                    client_id: row.get("client_id"),
                    course_id_on_client: row.get("course_id_on_client"),
                    name: row.get("name"),
                    teacher: row.try_get("teacher").ok(),
                    location: None,
                    color: row.try_get("color").ok(),
                    note: row.try_get("note").ok(),
                }
            }).collect();

            Ok(courses)
        }

        pub async fn get_client_schedule(&self, client_id: i32) -> AppResult<Vec<ScheduleEntry>> {
            let rows = sqlx::query(
                "SELECT se.id, se.client_id, se.entry_id_on_client, se.course_id,
                        c.name as course_name, c.teacher, c.color,
                        se.day_of_week, se.start_time, se.end_time, se.weeks
                 FROM schedule_entries se
                 JOIN courses c ON se.course_id = c.id
                 WHERE se.client_id = $1
                 ORDER BY se.day_of_week, se.start_time"
            )
            .bind(client_id)
            .fetch_all(&self.pool)
            .await?;

            let entries = rows.iter().map(|row| {
                let weeks_str: Option<String> = row.try_get("weeks").ok();
                let weeks: Option<Vec<i32>> = weeks_str.and_then(|s| {
                    serde_json::from_str(&s).ok()
                });

                ScheduleEntry {
                    id: row.get("id"),
                    client_id: row.get("client_id"),
                    entry_id_on_client: row.get("entry_id_on_client"),
                    course_id: row.get("course_id"),
                    course_name: row.try_get("course_name").ok(),
                    teacher: row.try_get("teacher").ok(),
                    location: None,
                    color: row.try_get("color").ok(),
                    day_of_week: row.get("day_of_week"),
                    start_time: row.get("start_time"),
                    end_time: row.get("end_time"),
                    weeks,
                    note: None,
                }
            }).collect();

            Ok(entries)
        }

        // Statistics
        pub async fn get_statistics(&self) -> AppResult<Statistics> {
            let total_clients: i64 = sqlx::query("SELECT COUNT(*) as count FROM clients")
                .fetch_one(&self.pool)
                .await?
                .get("count");

            let online_clients: i64 = sqlx::query("SELECT COUNT(*) as count FROM clients WHERE status = 'online'")
                .fetch_one(&self.pool)
                .await?
                .get("count");

            let total_courses: i64 = sqlx::query("SELECT COUNT(*) as count FROM courses")
                .fetch_one(&self.pool)
                .await?
                .get("count");

            let total_entries: i64 = sqlx::query("SELECT COUNT(*) as count FROM schedule_entries")
                .fetch_one(&self.pool)
                .await?
                .get("count");

            Ok(Statistics {
                total_clients,
                online_clients,
                total_courses,
                total_schedule_entries: total_entries,
            })
        }

        pub async fn get_client_statistics(&self) -> AppResult<Vec<ClientStatistics>> {
            let rows = sqlx::query(
                "SELECT c.id, c.name, c.last_sync,
                        (SELECT COUNT(*) FROM courses WHERE client_id = c.id) as course_count,
                        (SELECT COUNT(*) FROM schedule_entries WHERE client_id = c.id) as entry_count
                 FROM clients c
                 ORDER BY c.name"
            )
            .fetch_all(&self.pool)
            .await?;

            let stats = rows.iter().map(|row| {
                ClientStatistics {
                    client_id: row.get("id"),
                    client_name: row.get("name"),
                    total_courses: row.get("course_count"),
                    total_schedule_entries: row.get("entry_count"),
                    last_sync: row.try_get::<String, _>("last_sync").ok(),
                }
            }).collect();

            Ok(stats)
        }

        // Settings operations
        pub async fn get_all_settings(&self) -> AppResult<std::collections::HashMap<String, String>> {
            let rows = sqlx::query("SELECT key, value FROM settings")
                .fetch_all(&self.pool)
                .await?;

            let mut settings = std::collections::HashMap::new();
            for row in rows {
                let key: String = row.get("key");
                let value: String = row.get("value");
                settings.insert(key, value);
            }

            Ok(settings)
        }

        pub async fn get_setting(&self, key: &str) -> AppResult<Setting> {
            let row = sqlx::query("SELECT key, value FROM settings WHERE key = $1")
                .bind(key)
                .fetch_optional(&self.pool)
                .await?;

            match row {
                Some(row) => Ok(Setting {
                    key: row.get("key"),
                    value: row.get("value"),
                }),
                None => Err(AppError::NotFound("Setting not found".to_string())),
            }
        }

        pub async fn update_setting(&self, key: &str, value: &str) -> AppResult<()> {
            sqlx::query("INSERT INTO settings (key, value) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET value = $2")
                .bind(key)
                .bind(value)
                .execute(&self.pool)
                .await?;

            Ok(())
        }
    }
}
