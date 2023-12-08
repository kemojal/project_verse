use std::sync::Arc;
use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use bcrypt::{DEFAULT_COST, hash};
use serde_json::json;
use sqlx::{PgPool, query, query_as};
use crate::models::user_models::{EditUser, EditUserPassoword, NewUser, User, UserEmail};

pub async fn get_users(pool: Arc<PgPool>) -> impl IntoResponse {
    let users: Vec<User> = query_as!(
        User,
        r#"
        SELECT id, email, password, verification_code,  verified,  created_at, updated_at FROM users
        "#
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch appointments");

    Json(users)
}


pub async fn create_user(Json(new_user): Json<NewUser>, pool: Arc<PgPool>) -> impl IntoResponse {
    // let first_name = new_user.first_name;
    // let last_name = new_user.last_name;
    let email = new_user.email;
    let password = new_user.password;




    let users_emails: Option<UserEmail> = query_as!(
        UserEmail,
        "SELECT email FROM users WHERE email = $1",
        email
    )
        .fetch_optional(&*pool)
        .await
        .expect("Failed to fetch user");





    if users_emails.is_some() {
        // Email already exists
        Json(json!({
            "status": "error",
            "message": "User with this email already exists"
        }))
    }else{

        let hashed_password = hash_password(&password);
        let verification_code = String::from("123456");

        // INSERT INTO users (first_name, last_name, email, password, registration_date)
        let query_result = query!(

            "
            INSERT INTO users (email, password, verification_code )
            VALUES ($1, $2, $3)
            RETURNING *",
            // first_name,
            // last_name,
            email,
            hashed_password,
            verification_code
        )
            .fetch_one(&*pool)
            .await;

        match query_result {
            Ok(row) => {
                let new_id = row.id;
                Json(json!({
                    "status": "success",
                    "message": "User created successfully",
                    "new_id": new_id
                }))
            }
            Err(_) => {
                // Handle error case
                // You can return an error response or customize it as needed
                // For now, let's return a generic error response
                Json(json!({
                    "status": "error",
                    "message": "Failed to create user"
                }))
            }
        }

    }


}

pub async fn edit_user(
    Path(id): Path<i32>,
    edit_user_data: Json<EditUser>,
    pool: Arc<PgPool>
) -> impl IntoResponse {

    // let first_name = edit_user_data.first_name.clone();
    // let last_name = edit_user_data.last_name.clone();
    let email = edit_user_data.email.clone();

    // let completed = edit_todo_data.completed;

    // UPDATE users
    // SET first_name = coalesce($2, first_name),
    // last_name = coalesce($3, last_name),
    // email = coalesce($4, email)
    // WHERE id = $1
    // RETURNING *


    let update_result = query!(
        "
        UPDATE users
        SET email = coalesce($2, email)
        WHERE id = $1
        RETURNING *",
        id,
    email)
        .fetch_one(&*pool)
        .await;

    if update_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("User with ID {} edited", id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": "Failed to update user"
        }))
    }


}


pub async fn edit_user_password(
    Path(id): Path<i32>,
    edit_user_data: Json<EditUserPassoword>,
    pool: Arc<PgPool>
) -> impl IntoResponse {

    // let first_name = edit_user_data.first_name.clone();
    // let last_name = edit_user_data.last_name.clone();
    let email = edit_user_data.email.clone();
    let password  = edit_user_data.password.clone();
    // let completed = edit_todo_data.completed;

    // UPDATE users
    // SET first_name = coalesce($2, first_name),
    // last_name = coalesce($3, last_name),
    // email = coalesce($4, email)
    // WHERE id = $1
    // RETURNING *

    let hashed_password = hash_password(&password);
    let update_result = query!(
        "
        UPDATE users
        SET password = coalesce($2, password)
        WHERE id = $1
        RETURNING *",
        id,
    hashed_password)
        .fetch_one(&*pool)
        .await;

    if update_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("User's password with ID {} edited", id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": "Failed to update user's password "
        }))
    }


}


pub async fn delete_user(
    Path(id): Path<i32>,
    pool: Arc<PgPool>
) -> impl IntoResponse {
    println!("delete_todo id = {}", id);


    // Use the id to delete the item from the database
    let delete_result = query!(
        "DELETE FROM users WHERE id = $1",
        id
    )
        .execute(&*pool)
        .await;

    if delete_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("User with ID {} deleted", id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": format!("Failed to delete user with ID {}", id)
        }))
    }

}

fn hash_password(password: &str) -> String {

    let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");
    password_hash
}