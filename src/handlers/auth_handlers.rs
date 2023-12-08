use std::sync::Arc;
use axum::{Json, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode,  EncodingKey, Header};
use serde_json::json;
use sqlx::{PgPool, query_as};
use crate::models::auth_models::{AuthUser, Claims, SignInData};

pub async fn sign_in(Json(signin_data): Json<SignInData>, pool: Arc<PgPool>) -> impl IntoResponse {
    // Extract username and password from signin_data
    let user_email = signin_data.email;
    let user_password = signin_data.password;




    // Perform database query to check if the user exists and validate the password
    // "SELECT id, email, password, first_name, last_name FROM users WHERE email = $1"
    let user: Option<AuthUser> = query_as!(
        AuthUser,
        "SELECT id, email, password, verified, created_at, updated_at FROM users WHERE email = $1",
        user_email
    )
        .fetch_optional(&*pool)
        .await
        .expect("Failed to fetch user");

    // // Check if user exists and password is valid
    if let Some(user) = user {

        // if let (Some(id), Some(email), Some(hashed_password), Some(first_name), Some(last_name)) = (user.id, user.email, user.password, user.first_name, user.last_name) {
        if let (Some(id), Some(email), Some(hashed_password), Some(verified), Some(created_at), Some(updated_at)) = (user.id, user.email, user.password, user.verified, user.created_at, user.updated_at) {
            if bcrypt::verify(&user_password, &hashed_password).expect("Failed to verify password") {

                let jwt_secret = "CfLTk9J0MA3jBF3/zuE4VUyN7djM2KMPy4otUpbkbE8=";
                let expiration = Utc::now() + Duration::hours(1);




                let my_claims = Claims {
                    email: email.to_owned(),
                    verified: verified.to_owned(),
                    // sub: first_name.to_owned(),
                    // first_name: first_name.to_owned(),
                    // last_name: last_name.to_owned(),
                    user_id: id.to_owned(),
                    created_at: created_at.to_owned(),
                    updated_at: updated_at.to_owned(),
                    exp: 10000000000,
                };

                let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(b"CfLTk9J0MA3jBF3/zuE4VUyN7djM2KMPy4otUpbkbE8=")) {
                    Ok(t) => t,
                    Err(_) => panic!(), // in practice you would return the error
                };

                // Return success response with token
                return Json(json!({
                    "status": "success",
                    "message": "Sign-in successful",
                    "token": token
                }));
            } else {
                // Passwords do not match, return an error response
                return Json(json!({
                    "status": "error",
                    "message": "Invalid credentials"
                }));
            }


        }else {
            // Passwords do not match, return an error response
            return Json(json!({
                    "status": "error",
                    "message": "Invalid credentials"
                }));
        }
    }

    // If user doesn't exist or password is invalid, return an error response
    Json(json!({
        "status": "error",
        "message": "Invalid credentials"
    }))
}



pub async fn sign_out(Json(signin_data): Json<SignInData>, pool: Arc<PgPool>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Invalid credentials"
    }))
}