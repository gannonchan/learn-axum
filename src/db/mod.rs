use std::time::Duration;

use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;

pub async fn mysql_pool() -> anyhow::Result<Pool<MySql>> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(6u64))
        .connect("mysql://root:123456@localhost:33061/userdb").await?;
    Ok(pool)
}