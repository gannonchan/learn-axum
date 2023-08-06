use crate::AppDbStat;
use crate::domain::sys_role::SysRole;

pub async fn find_all(app_stat: AppDbStat) -> sqlx::Result<Vec<SysRole>> {
    let sys_roles: Vec<SysRole> = sqlx::query_as("SELECT * FROM t_sys_role")
        .fetch_all(&app_stat.pool)
        .await?;
    Ok(sys_roles)
}

pub async fn find(app_stat: AppDbStat, id: i64) -> sqlx::Result<SysRole> {
    let sys_role = sqlx::query_as("SELECT * FROM t_sys_role WHERE id = ?")
        .bind(id)
        .fetch_one(&app_stat.pool)
        .await?;
    Ok(sys_role)
}