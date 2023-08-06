use serde::Deserialize;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}
