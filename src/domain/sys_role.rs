use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct SysRole {
    id: Option<i64>,
    role_code: Option<String>,
    role_name: Option<String>,
}

impl SysRole {
    // getter
    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn role_code(&self) -> Option<String> {
        self.role_code.clone()
    }
    pub fn role_name(&self) -> Option<String> {
        self.role_name.clone()
    }
    // setter
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
    pub fn set_role_code(&mut self, role_code: &str) {
        self.role_code = Some(role_code.to_string())
    }
    pub fn set_role_name(&mut self, role_name: &str) {
        self.role_name = Some(role_name.to_string())
    }
}

impl Default for SysRole {
    fn default() -> Self {
        Self {
            id: None,
            role_code: None,
            role_name: None,
        }
    }
}