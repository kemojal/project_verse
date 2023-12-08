use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct SignInData {
    pub email:  String,
    pub password:  String,
}

#[derive(Debug, Deserialize)]
pub  struct AuthUser {
    pub  id: Option<i32>,
    pub  email: Option<String>,
    pub  password: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    // pub first_name: Option<String>,
    // pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // pub aud: String,
    // pub sub: String,
    pub email: String,
    pub verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // pub first_name: String,
    // pub last_name: String,
    pub user_id: i32,
    // pub verified: bool,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
    pub exp: usize,

}