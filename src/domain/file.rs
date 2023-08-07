use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::mysql::MySqlRow;

// 文件类 !学习用难得写getter setter 所以就不封装了
#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: Option<i64>,
    pub file_name: Option<String>,
    pub origin_name: Option<String>,
    pub file_extension: Option<String>,
    pub file_size: Option<i32>,
    pub file_local_path: Option<String>,
    pub file_remote_path: Option<String>,
    pub deleted: Option<bool>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

// 手动实现结果集处理
impl FromRow<'_, MySqlRow> for File {
    fn from_row(row: &MySqlRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            file_name: row.try_get("file_name")?,
            origin_name: row.try_get("origin_name")?,
            file_extension: row.try_get("file_extension")?,
            file_size: row.try_get("file_size")?,
            file_local_path: row.try_get("file_local_path")?,
            file_remote_path: row.try_get("file_remote_path")?,
            deleted: row.try_get("deleted")?,
            create_time: row.try_get("create_time")?,
            update_time: row.try_get("update_time")?,
        })
    }
}