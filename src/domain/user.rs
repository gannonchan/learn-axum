use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
    gender: Option<String>,
    province: Option<String>,
    city: Option<String>,
    address: Option<String>,
    phone: Option<String>,
    create_time: Option<NaiveDateTime>,
    update_time: Option<NaiveDateTime>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            name: None,
            age: None,
            gender: None,
            province: None,
            city: None,
            address: None,
            phone: None,
            create_time: None,
            update_time: None,
        }
    }
}

impl User {
    // all arguments constructor
    pub fn new(id: i64, name: String, age: i32, gender: String, province: String, city: String,
               address: String, phone: String, create_time: NaiveDateTime, update_time: NaiveDateTime) -> Self {
        Self {
            id: Some(id),
            name: Some(name),
            age: Some(age),
            gender: Some(gender),
            province: Some(province),
            city: Some(city),
            address: Some(address),
            phone: Some(phone),
            create_time: Some(create_time),
            update_time: Some(update_time),
        }
    }

    // setter
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned())
    }
    pub fn set_age(&mut self, age: i32) {
        self.age = Some(age)
    }
    pub fn set_gender(&mut self, gender: &str) {
        self.gender = Some(gender.to_owned())
    }
    pub fn set_province(&mut self, province: &str) {
        self.province = Some(province.to_owned())
    }
    pub fn set_city(&mut self, city: &str) {
        self.city = Some(city.to_owned())
    }
    pub fn set_address(&mut self, address: &str) {
        self.address = Some(address.to_owned())
    }
    pub fn set_phone(&mut self, phone: &str) {
        self.phone = Some(phone.to_owned())
    }
    pub fn set_create_time(&mut self, create_time: NaiveDateTime) {
        self.create_time = Some(create_time)
    }
    pub fn set_update_time(&mut self, update_time: NaiveDateTime) {
        self.update_time = Some(update_time)
    }
    // getter
    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn age(&self) -> Option<i32> {
        self.age
    }
    pub fn gender(&self) -> Option<String> {
        self.gender.clone()
    }
    pub fn province(&self) -> Option<String> {
        self.province.clone()
    }
    pub fn city(&self) -> Option<String> {
        self.city.clone()
    }
    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }
    pub fn phone(&self) -> Option<String> {
        self.phone.clone()
    }
    pub fn create_time(&self) -> Option<NaiveDateTime> {
        self.create_time
    }
    pub fn update_time(&self) -> Option<NaiveDateTime> {
        self.update_time
    }
}