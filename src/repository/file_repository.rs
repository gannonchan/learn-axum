use crate::AppDbStat;
use crate::domain::file::File;

pub async fn find_all(app_stat: AppDbStat) -> sqlx::Result<Vec<File>> {
    let files: Vec<File> = sqlx::query_as("SELECT * FROM t_file")
        .fetch_all(&app_stat.pool)
        .await?;
    Ok(files)
}