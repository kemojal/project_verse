use crate::models::issue_models::{Issue, NewIssue};
use crate::models::merchant_models::{Merchant, MerchantId, NewMerchant};
use crate::models::user_models::UserId;
use axum::extract::Path;
use axum::response::{IntoResponse, Json, Response};
use chrono::expect;
use serde_json::json;
use sqlx::{pool, query, query_as, PgPool};
use std::sync::Arc;

pub async fn create_merchant(
    Path(username): Path<String>,
    Json(new_merchant): Json<NewMerchant>,
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    let user_id: Vec<UserId> = query_as!(
        UserId,
        "
        SELECT id
        FROM users
        WHERE username = $1
        ",
        username
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch user");

    if let Some(first_user_id) = user_id.get(0) {
        let existing_merchant: Option<MerchantId> = query_as!(
            MerchantId,
            "
            SELECT id
            FROM merchants
            WHERE user_id = $1 AND business_name = $2
            ",
            first_user_id.id,
            new_merchant.business_name
        )
        .fetch_optional(&*pool)
        .await
        .expect("Failed to check for existing merchant");

        if let Some(_) = existing_merchant {
            // If an existing merchant with the same business_name is found,
            // return an error response indicating that the merchant already exists
            return Json(json!({
                "status": "error",
                "message": "Merchant with the same business name already exists for this user"
            }));
        }

        let result = query!(
        "
        INSERT INTO merchants (description, business_name, business_type, address, business_phone_number, website, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        ",
        new_merchant.description,
        new_merchant.business_name,
        new_merchant.business_type,
        new_merchant.address,
        new_merchant.business_phone_number,
        new_merchant.website,
       first_user_id.id
    )
            .fetch_one(&*pool)
            .await;

        match result {
            Ok(row) => {
                let new_id = row.id;
                // Update user_type in users table to 1 for the user who created the merchant
                let _ = query!(
                    "
                    UPDATE users
                    SET user_type = 1
                    WHERE id = $1
                    ",
                    first_user_id.id
                )
                .execute(&*pool)
                .await;

                return Json(json!({
                    "status": "success",
                    "message": "Merchant added successfully",
                    "new_id": new_id
                }));
            }
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                // Handle error case
                // You can return an error response or customize it as needed
                // For now, let's return a generic error response
                return Json(json!({
                    "status": "error",
                    "message": format!("Failed to create a merchant: {:?}", e)
                }));
            }
        }
    }
    Json(json!([]))
}

pub async fn get_merchant(Path(username): Path<String>, pool: Arc<PgPool>) -> impl IntoResponse {
    let user_id: Vec<UserId> = query_as!(
        UserId,
        "
        SELECT id
        FROM users
        WHERE username = $1
        ",
        username
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch user");

    if let Some(first_user_id) = user_id.get(0) {
        let merchant: Option<Merchant> = query_as!(
            Merchant,
            "
            SELECT *
            FROM merchants
            WHERE user_id = $1
            ",
            first_user_id.id
        )
        .fetch_optional(&*pool)
        .await
        .expect("Failed to fetch merchant");

        if let Some(merchant) = merchant {
            return Json(merchant);
        }
    }
    Json(Merchant {
        id: 0,
        latitude: None,
        longitude: None,
        created_at: None,
        edited_at: None,
        description: None,
        business_name: None,
        business_type: None,
        address: None,
        business_phone_number: None,
        website: None,
        user_id: Some(0),
    })
}

// pub async fn edit_merchant(
//     Path(username): Path<String>,
//     Json(new_merchant): Json<NewMerchant>,
//     pool: Arc<PgPool>,
// ) -> impl IntoResponse {
//     let user_id: Vec<UserId> = query_as!(
//         UserId,
//         "
//         SELECT id
//         FROM users
//         WHERE username = $1
//         ",
//         username
//     )
//     .fetch_all(&*pool)
//     .await
//     .expect("Failed to fetch user");

//     if let Some(first_user_id) = user_id.get(0) {
//         let result = query!(
//             "
//             UPDATE merchants
//             SET description = $1,
//                 business_name = $2,
//                 business_type = $3,
//                 address = $4,
//                 business_phone_number = $5,
//                 website = $6,
//                 edited_at = CURRENT_TIMESTAMP
//             WHERE user_id = $7
//             RETURNING *
//             ",
//             new_merchant.description,
//             new_merchant.business_name,
//             new_merchant.business_type,
//             new_merchant.address,
//             new_merchant.business_phone_number,
//             new_merchant.website,
//             first_user_id.id
//         )
//         .fetch_one(&*pool)
//         .await;

//         match result {
//             Ok(row) => {
//                 return Json(json!({
//                     "status": "success",
//                     "message": "Merchant updated successfully",
//                     "updated_merchant": row
//                 }));
//             }
//             Err(e) => {
//                 println!("Error updating merchant: {:?}", e);
//                 return Json(json!({
//                     "status": "error",
//                     "message": format!("Failed to update merchant: {:?}", e)
//                 }));
//             }
//         }
//     }
//     Json(json!([]))
// }

// pub async fn delete_merchant(Path(username): Path<String>, pool: Arc<PgPool>) -> impl IntoResponse {
//     let user_id: Vec<UserId> = query_as!(
//         UserId,
//         "
//         SELECT id
//         FROM users
//         WHERE username = $1
//         ",
//         username
//     )
//     .fetch_all(&*pool)
//     .await
//     .expect("Failed to fetch user");

//     if let Some(first_user_id) = user_id.get(0) {
//         let result = query!(
//             "
//             DELETE FROM merchants
//             WHERE user_id = $1
//             RETURNING id
//             ",
//             first_user_id.id
//         )
//         .fetch_one(&*pool)
//         .await;

//         match result {
//             Ok(row) => {
//                 let deleted_id = row.id;
//                 return Json(json!({
//                     "status": "success",
//                     "message": "Merchant deleted successfully",
//                     "deleted_id": deleted_id
//                 }));
//             }
//             Err(e) => {
//                 println!("Error deleting merchant: {:?}", e);
//                 return Json(json!({
//                     "status": "error",
//                     "message": format!("Failed to delete merchant: {:?}", e)
//                 }));
//             }
//         }
//     }
//     Json(json!({}))
// }
