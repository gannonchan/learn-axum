use serde::Serialize;

use crate::domain::payment_status::PaymentStatus;

#[derive(Serialize)]
pub struct Payment {
    pub id: i64,
    pub price: f64,
    pub status: PaymentStatus,
    pub description: String,
}