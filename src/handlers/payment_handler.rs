use std::sync::Arc;

use axum::{Extension, Json};
use sqlx::{MySql, Pool};

use crate::domain::payment::Payment;
use crate::domain::payment_status::PaymentStatus;
use crate::dto::payment_dto::CreatePayment;

pub async fn find_all(Extension(pool): Extension<Arc<Pool<MySql>>>) -> Json<Vec<Payment>> {
    let mut all: Vec<Payment> = Vec::new();
    let payment1 = Payment {
        id: 13,
        status: PaymentStatus::PAYMENTED,
        description: "已支付完成".to_owned(),
        price: 3.5f64,
    };

    let payment2 = Payment {
        id: 18,
        status: PaymentStatus::NO_PAYMENT,
        description: "未进行支付".to_owned(),
        price: 13f64,
    };
    all.push(payment1);
    all.push(payment2);
    Json(all)
}

pub async fn create_payment(Json(payload): Json<CreatePayment>) -> Json<Payment> {
    let payment = Payment {
        id: payload.id,
        price: payload.price,
        status: PaymentStatus::NO_PAYMENT,
        description: payload.description,
    };
    Json(payment)
}