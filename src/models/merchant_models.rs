use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::OptionalExtension;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Merchant {
    pub id: i32,
    pub description: Option<String>,
    pub latitude: Option<sqlx::types::BigDecimal>,
    pub longitude: Option<sqlx::types::BigDecimal>,
    pub created_at: Option<NaiveDateTime>,
    pub business_name: Option<String>,
    pub business_type: Option<String>,
    pub address: Option<String>,
    pub business_phone_number: Option<String>,
    pub website: Option<String>,
    pub edited_at: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
    // pub name: Option<String>,

    // pub email: Option<String>,
    // pub password_hash: Option<String>,
    // pub verification_code: Option<String>,
    // pub verified: Option<bool>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMerchant {
    pub description: Option<String>,
    // pub latitude: f64,
    // pub longitude: f64,
    // pub created_at: Option<NaiveDateTime>,
    pub business_name: Option<String>,
    pub business_type: Option<String>,
    pub address: Option<String>,
    pub business_phone_number: Option<String>,
    pub website: Option<String>,
    // pub edited_at: Option<NaiveDateTime>,
    // pub user_id: i32,
}

//  the once below are not yet implemented

#[derive(Deserialize)]
pub struct EditMerchant {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub email: Option<String>,
    // pub  password: Option<String>,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct EditMerchantPassoword {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct MerchantEmail {
    pub email: Option<String>,
}

pub struct MerchantPhoneNumber {
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpMerchantEmail {
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantId {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyMerchant {
    pub verification_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantToVerify {
    pub id: i32,
    pub email: Option<String>,
    pub verification_code: Option<String>,
    pub verified: Option<bool>,
    pub verification_code_created_at: Option<NaiveDateTime>,
}

pub struct AuthorizedMerchant {
    pub id: i32,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceAmount {
    pub total_paid: i32,
}
