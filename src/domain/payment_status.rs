use serde::Serialize;

#[allow(non_camel_case_types)]
#[derive(Serialize)]
pub enum PaymentStatus {
    NO_PAYMENT,
    PAYMENTED,
}