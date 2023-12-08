use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::OptionalExtension;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub password: Option<String>,
    pub verification_code: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub email: Option<String>,
    pub password: String,
    // pub verification_code: Option<String>,
    // pub verified: Option<bool>,
    // pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    // pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}


#[derive(Deserialize)]
pub  struct EditUser {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub  email: Option<String>,
    // pub  password: Option<String>,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub  struct EditUserPassoword {
    // pub  first_name: Option<String>,
    // pub  last_name: Option<String>,
    pub  email: Option<String>,
    pub  password: String,
    // pub registration_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub  struct UserEmail {
    pub  email: Option<String>,
}
