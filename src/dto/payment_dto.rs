use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePayment {
    pub id: i64,
    pub price: f64,
    pub description: String,
}