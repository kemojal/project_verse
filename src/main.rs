
mod db;
use db::create_db_pool;

use std::sync::Arc;
use axum::http::{HeaderValue, Method};


use axum::{Router, Server};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};


use sqlx::{ PgPool};

use tower_http::cors::{CorsLayer, Any};





mod routes;
mod models;
mod handlers;
mod middleware;

// use routes::barbershop_routes;
use routes::{
    workspace_routes::workspace_routes,
    user_routes::user_routes,
    auth_routes::auth_routes,
    issue_routes:: issue_routes,
    // barbershop_routes::barbershop_routes,
    // client_routes::client_routes,
    // subscription_routes::subscription_routes,
    // product_routes::product_routes,
    // appointment_routes::appointment_routes,
};


fn app_routes(pool: Arc<PgPool>) -> Router {
    let workspace_pool  = pool.clone();
    let user_pool  = pool.clone();
    let auth_pool = pool.clone();
    let issue_pool = pool.clone();

    Router::new()
        .nest("/api/workspace", workspace_routes(workspace_pool)) // Import and use auth routes
        .nest("/api/user", user_routes(user_pool))
        .nest("/api/auth", auth_routes(auth_pool))
        .nest("/api/issue", issue_routes(issue_pool))
}


// use crate::middleware::jwt_middleware::{jwt_middleware};
// fn app_routes(pool: Arc<PgPool>) -> Router {
//     let workspace_pool = pool.clone();
//     let user_pool = pool.clone();
//     let auth_pool = pool.clone();
//     let issue_pool = pool.clone();
//
//     let protected_workspace_routes = jwt_middleware
//         .nest("/api/workspace", workspace_routes(workspace_pool));
//
//     let protected_user_routes = jwt_middleware
//         .nest("/api/user", user_routes(user_pool));
//
//     let protected_auth_routes = jwt_middleware
//         .nest("/api/auth", auth_routes(auth_pool));
//
//     let protected_issue_routes = jwt_middleware
//         .nest("/api/issue", issue_routes(issue_pool));
//
//     Router::new()
//         .nest("/api/workspace", protected_workspace_routes)
//         .nest("/api/user", protected_user_routes)
//         .nest("/api/auth", protected_auth_routes)
//         .nest("/api/issue", protected_issue_routes)
// }


#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        // allow requests from any origin
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        // .allow_origin(Any)
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        ;


    // let pool = create_db_pool.await;
    let pool = create_db_pool().await;
    let app = app_routes(pool.clone().into()).layer(CorsLayer::permissive())
        ;

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}