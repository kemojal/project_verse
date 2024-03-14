use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use sqlx::PgPool;
use std::sync::Arc;
// use crate::handlers::issue_handlers::{create_issue, get_issues, get_issues_by_workspace_id, get_my_issues_all, get_my_issues_created, get_issues_by_workspace_slug};
// use crate::handlers::workspace_handlers::create_workspace;
// use crate::models::issue_models::NewIssue;
// use crate::models::workspace_models::NewWorkspace;
use crate::handlers::merchant_handlers::{create_merchant, get_merchant};
use crate::models::merchant_models::NewMerchant;

pub fn merchant_routes(pool: Arc<PgPool>) -> Router {
    let get_pool = Arc::clone(&pool);
    let get_slug_pool = Arc::clone(&pool);
    let get_user_pool = Arc::clone(&pool);
    let get_my_merchant_pool = Arc::clone(&pool);
    let create_pool = Arc::clone(&pool);
    let get_my_issues_created_pool = Arc::clone(&pool);
    let delete_pool = Arc::clone(&pool);
    let get_issues_by_workspace_id_pool = Arc::clone(&pool);

    Router::new()
        // .route("/", get( move || {get_issues(get_pool)  }))
        // .route("/:user_id", get( move |path: Path<i32>,| {
        //     get_issues_by_workspace_id(path, get_user_pool)
        // }))
        .route("/:username/merchants", get( move |path: Path<String>,| {
            get_merchant(path, get_my_merchant_pool)
        }))
        // .route("/:username/my_issues/created", get( move |path: Path<String>| {
        //     get_my_issues_created(path, get_my_issues_created_pool)
        // }))
        .route(
            "/:username/create",
            post(
                move |path: Path<String>, Json(new_merchant): Json<NewMerchant>| {
                    create_merchant(path, Json(new_merchant), create_pool)
                },
            ),
        )
    // .route("/workspaces/:workspace_slug", get( move |path: Path<String>| {
    //     get_issues_by_workspace_slug(path, get_issues_by_workspace_id_pool)
    // }))
}
