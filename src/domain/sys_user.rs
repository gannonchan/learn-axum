use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::sys_role::SysRole;

#[derive(Serialize, Deserialize, FromRow)]
pub struct SysUser {
    id: Option<i64>,
    username: Option<String>,
    password: Option<String>,
    #[sqlx(skip)] // 跳过该字段的自动映射赋值
    roles: Option<Vec<SysRole>>,
}

impl SysUser {
    // getter
    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }
    pub fn password(&self) -> Option<String> {
        self.password.clone()
    }
    pub fn roles(&self) -> Option<&Vec<SysRole>> {
        self.roles.as_ref()
    }
    // setter
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
    pub fn set_username(&mut self, username: &str) {
        self.username = Some(username.to_string())
    }
    pub fn set_password(&mut self, password: &str) {
        self.password = Some(password.to_string())
    }
    pub fn set_roles(&mut self, roles: Vec<SysRole>) {
        self.roles = Some(roles)
    }
}

impl Default for SysUser {
    fn default() -> Self {
        Self {
            id: None,
            username: None,
            password: None,
            roles: None,
        }
    }
}