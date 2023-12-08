use std::sync::Arc;
use axum::extract::Path;
use axum::response::{Json, IntoResponse, Response};
use serde_json::json;
use sqlx::{PgPool, query, query_as};
use crate::models::issue_models::{Issue, NewIssue};
use crate::models::workspace_models::Workspace;

pub async fn get_issues(pool: Arc<PgPool>) -> impl IntoResponse {
    let issues: Vec<Issue> = query_as!(
        Issue,
        r#"
        SELECT id, workspace_id, name, description, status, priority, created_at, updated_at
        FROM issues
        "#
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch issues");

    Json(issues)
}



// https://linear.app/kemo/my-issues/assigned
//https://linear.app/kemo/my-issues/created
//https://linear.app/kemo/my-issues/subscribed
//https://linear.app/kemo/my-issues/activity

//https://linear.app/bantaba/team/RAI/all

pub async fn get_issues_by_workspace_id(
    Path(workspace_id): Path<i32>,
    pool: Arc<PgPool>) -> impl IntoResponse {
    let issues: Vec<Issue> = query_as!(
        Issue,
        "
        SELECT id, workspace_id, name, description, status, priority, created_at, updated_at
        FROM issues
        WHERE workspace_id = $1
        ",
        workspace_id
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch workspaces");

    Json(issues)
}

// pub async fn get_issues_by_workspace_id(workspace_id: i32, pool: &PgPool) -> impl IntoResponse {
//     let issues: Vec<Issue> = query_as!(
//         Issue,
//         "
//         SELECT id, workspace_id, name, description, status, priority, assignee_id, created_at, updated_at
//         FROM issues
//         WHERE workspace_id = $1
//         ",
//         workspace_id
//     )
//         .fetch_all(pool)
//         .await?;
//
//     Ok(Response::json(&issues))
// }
//
// pub async fn create_issue(new_issue: NewIssue, pool: &PgPool) -> impl IntoResponse {
//     let result = query!(
//         "
//         INSERT INTO issues (workspace_id, name, description, status, priority, assignee_id, created_at, updated_at)
//         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
//         RETURNING *
//         ",
//         new_issue.workspace_id,
//         new_issue.name,
//         new_issue.description,
//         new_issue.status,
//         new_issue.priority,
//         new_issue.assignee_id,
//         new_issue.created_at,
//         new_issue.updated_at,
//     )
//         .fetch_one(pool)
//         .await;
//
//     match result {
//         Ok(row) => {
//             let new_id = row.id;
//             let response_json = json!({
//                 "status": "success",
//                 "message": "Issue added successfully",
//                 "new_id": new_id,
//             });
//             Ok(Response::json(&response_json))
//         }
//         Err(_) => {
//             let error_json = json!({
//                 "status": "error",
//                 "message": "Failed to create an issue",
//             });
//             Ok(Response::json(&error_json).status(StatusCode::InternalServerError))
//         }
//     }
// }
//
// pub async fn delete_issue(issue_id: i32, pool: &PgPool) -> impl IntoResponse {
//     let result = query!(
//         "DELETE FROM issues WHERE id = $1",
//         issue_id
//     )
//         .execute(pool)
//         .await;
//
//     if result.is_ok() {
//         let success_json = json!({
//             "status": "success",
//             "message": format!("Issue with ID {} deleted", issue_id),
//         });
//         Ok(Response::json(&success_json))
//     } else {
//         let error_json = json!({
//             "status": "error",
//             "message": format!("Failed to delete the issue with ID {}", issue_id),
//         });
//         Ok(Response::json(&error_json).status(StatusCode::InternalServerError))
//     }
// }