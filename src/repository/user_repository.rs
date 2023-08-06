use crate::AppDbStat;
use crate::domain::user::User;

pub async fn find_all(app_stat: AppDbStat) -> sqlx::Result<Vec<User>> {
    let users: Vec<User> = sqlx::query_as("select * from t_user")
        .fetch_all(&app_stat.pool)
        .await?;
    Ok(users)
}

pub async fn find(app_stat: AppDbStat, id: i64) -> sqlx::Result<User> {
    let user: User = sqlx::query_as("select * from t_user where id = ?")
        .bind(id)
        .fetch_one(&app_stat.pool)
        .await?;
    Ok(user)
}

pub async fn create(user: &User, app_stat: AppDbStat) -> sqlx::Result<bool> {
    let result = sqlx::query(r#"insert into t_user(name, age, gender, province, city, address, phone)
     values(?, ?, ?, ?, ?, ?, ?)"#)
        .bind(user.name())
        .bind(user.age())
        .bind(user.gender())
        .bind(user.province())
        .bind(user.city())
        .bind(user.address())
        .bind(user.phone())
        .execute(&app_stat.pool)
        .await?
        .rows_affected();
    Ok(result > 0)
}


pub async fn update_by_id(user: &User, app_stat: AppDbStat) -> sqlx::Result<bool> {
    let result = sqlx::query(r#"update t_user set name=?, age = ?, gender = ?, province = ?,
     city = ?, address = ?, phone = ? where id = ? "#)
        .bind(user.name())
        .bind(user.age())
        .bind(user.gender())
        .bind(user.province())
        .bind(user.city())
        .bind(user.address())
        .bind(user.phone())
        .bind(user.id())
        .execute(&app_stat.pool)
        .await?
        .rows_affected();
    Ok(result > 0)
}

pub async fn delete(app_stat: AppDbStat) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM t_user")
        .execute(&app_stat.pool)
        .await?
        .rows_affected();
    Ok(result > 0)
}

pub async fn delete_by_id(app_stat: AppDbStat, id: i64) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM t_user WHERE id = ?")
        .bind(id)
        .execute(&app_stat.pool)
        .await?
        .rows_affected();
    Ok(result > 0)
}