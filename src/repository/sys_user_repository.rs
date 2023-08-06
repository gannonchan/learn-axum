use sqlx::Row;

use crate::AppDbStat;
use crate::domain::sys_user::SysUser;

// 让sqlx来帮我们给对象赋值属性
pub async fn find_all(app_stat: AppDbStat) -> sqlx::Result<Vec<SysUser>> {
    let sys_users: Vec<SysUser> = sqlx::query_as("SELECT id, username, password FROM t_sys_user")
        .fetch_all(&app_stat.pool)
        .await?;
    Ok(sys_users)
}

// 自己处理结果集给对象赋值
pub async fn find_all1(app_stat: AppDbStat) -> sqlx::Result<Vec<SysUser>> {
    let rows = sqlx::query("SELECT id, username, password FROM t_sys_user")
        .fetch_all(&app_stat.pool)
        .await?;
    let mut sys_users = Vec::new();

    for row in rows {
        let id = row.get(0);
        let username = row.get(1);
        let password = row.get(2);
        println!("id:{:?}\tusername:{:?}\tpassword:{:?}", id, username, password);
        let mut sys_user = SysUser::default();
        sys_user.set_id(id);
        sys_user.set_username(username);
        sys_user.set_password(password);
        sys_users.push(sys_user);
    }
    Ok(sys_users)
}

pub async fn find(app_stat: AppDbStat, id: i64) -> sqlx::Result<SysUser> {
    let sys_user: SysUser = sqlx::query_as("SELECT id, username, password FROM t_sys_user WHERE id = ?")
        .bind(id)
        .fetch_one(&app_stat.pool)
        .await?;
    Ok(sys_user)
}