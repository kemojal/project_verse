use std::sync::Arc;
use axum::extract::Path;
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use sqlx::PgPool;
use crate::handlers::user_handlers::{create_user, delete_user, edit_user, get_users};
use crate::models::user_models::{EditUser, NewUser};


pub fn user_routes(pool: Arc<PgPool>) -> Router {
    let get_pool = Arc::clone(&pool);


    let userPool = Arc::clone(&pool);
    let getUserPool = Arc::clone(&pool);
    let editUserPool = Arc::clone(&pool);
    let editUserPasswordPool = Arc::clone(&pool);
    let deleteUserPool = Arc::clone(&pool);


    Router::new()
        .route("/", get( move || {get_users(get_pool)  }))
        .route("/create", post(move |Json(new_user): Json<NewUser>| {
            create_user(axum::Json(new_user),userPool.clone())
        }))
        .route("/edit/:id", put(move |path: Path<i32>, Json(edit_user_data): Json<EditUser>| {
            edit_user(path, Json(edit_user_data), editUserPool)
        }))
        .route("/edit/:id/password", put(move |path: Path<i32>, Json(edit_user_data): Json<EditUser>| {
            edit_user(path, Json(edit_user_data), editUserPasswordPool)
        }))
        .route("/delete/:id", delete(move |path: Path<i32>| {

            delete_user(path,  deleteUserPool)
        }))

}