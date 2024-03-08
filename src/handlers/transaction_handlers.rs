use axum::extract::Path;
use axum::http::response;
use std::sync::Arc;

use crate::models::transaction_models::{NewTransaction, Transaction};
use crate::models::user_models::{
    EditUser, EditUserPassoword, NewUser, SignUpUserEmail, User, UserEmail, UserId,
    UserPhoneNumber, UserToVerify, VerifyUser,
};
use crate::models::wallet_models::Wallet;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use lettre::message::Mailbox;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::json;
use sqlx::{query, query_as, PgPool};

use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error as SmtpError;
use lettre::{Message, SmtpTransport, Transport};

use reqwest::{Client, StatusCode};

pub async fn get_user_transactions(Path(phone_number): Path<String>, pool: Arc<PgPool>) -> impl IntoResponse {
    // Fetch user_id from the users table based on phone number
    let user_id: Option<UserId> = match query_as!(
            UserId,
            "SELECT id FROM users WHERE phone_number = $1",
            phone_number
        )
        .fetch_optional(&*pool)
        .await {
            Ok(result) => result,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };

    // If user_id is found, fetch transactions
    if let Some(user_id) = user_id {
        let transactions: Vec<Transaction> = match query_as!(
            Transaction,
            "SELECT sender_id, recipient_id, amount, currency, status, transaction_type, transaction_date FROM transactions WHERE sender_id = $1",
            user_id
        )
        .fetch_all(&*pool)
        .await {
            Ok(result) => result,
            Err(_) =>     ,
        };
        json(&transactions)
    } else {
        // If user_id is not found, return empty array
        json(&[])
    }
}

// pub async fn get_user_profile(Path(email): Path<String>, pool: Arc<PgPool>) -> impl IntoResponse {
//     let user: Vec<User> = query_as!(
//         User,
//         "SELECT id, email, password_hash, verification_code,  verified,  created_at, updated_at FROM users WHERE email = $1",
//         email
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch appointments");

//     Json(user)
// }


pub async fn send_money(
    Path(phone_number): Path<String>,
    Json(new_transaction): Json<NewTransaction>,
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    // ... (omitted for brevity)
    let sender_id = new_transaction.sender_id;
    let recipient_id = new_transaction.recipient_id;
    let amount = new_transaction.amount;
    let currency = new_transaction.currency;
    let status = new_transaction.status;
    let transaction_type = new_transaction.transaction_type;

    let user_id: Vec<UserId> = query_as!(
        UserId,
        "
        SELECT id
        FROM users
        WHERE phone_number = $1
        ",
        phone_number
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch user");

    

    if let Some(first_user_id) = user_id.get(0) {
        let user_wallet_result: Result<Vec<Wallet>, sqlx::Error> = sqlx::query_as!(
            Wallet,
            "
            SELECT id, user_id, balance, currency, created_at, updated_at
            FROM wallets
            WHERE user_id = $1
            ",
            sender_id
        )
        .fetch_all(&*pool)
        .await;

        match user_wallet_result {
            Ok(user_wallet) => {
                if let Some(wallet) = user_wallet.get(0) {
                    // println!("Sender's wallet balance: {:?}", wallet.balance);
                    // println!("Amount to be sent: {:?}", amount);
                    if wallet.balance >= amount && wallet.currency == currency {
                        // Perform money transfer logic here
                        // Update sender's wallet balance
                        let updated_sender_balance = match (wallet.balance.clone(), amount.clone())
                        {
                            (Some(balance), Some(amt)) => {
                                let new_balance = balance - amt;
                                Some(new_balance)
                            }
                            _ => None,
                        };

                        if let Some(updated_balance) = updated_sender_balance {
                            let now: NaiveDateTime = Utc::now().naive_utc();
                            query!(
                                "
                                UPDATE wallets
                                SET balance = $1, updated_at = $2
                                WHERE id = $3
                                ",
                                updated_balance,
                                now,
                                wallet.id
                            )
                            .fetch_all(&*pool)
                            .await
                            .expect("Failed to update sender's wallet balance");
                        } else {
                            // Handle case when balance or amount is None
                            return (
                                StatusCode::BAD_REQUEST,
                                Json(json!({
                                    "status": "error",
                                    "message": "Invalid wallet balance or amount"
                                })),
                            );
                        }

                        // Update receiver's wallet balance
                        // Update receiver's wallet balance
                        let receiver_wallet: Result<Vec<Wallet>, sqlx::Error> = query_as!(
                            Wallet,
                            "
                            SELECT id, user_id, balance, currency, created_at, updated_at
                            FROM wallets
                            WHERE user_id = $1
                            ",
                            recipient_id
                        )
                        .fetch_all(&*pool)
                        .await;

                        match receiver_wallet {
                            Ok(recipient_wallet) => {
                                if let Some(receiver_wallet) = recipient_wallet.get(0) {
                                    let updated_receiver_balance =
                                        match (receiver_wallet.balance.clone(), amount.clone()) {
                                            (Some(balance), Some(amt)) => {
                                                let new_balance = balance + amt;
                                                Some(new_balance)
                                            }
                                            // (None, Some(amt)) => Some(amt),
                                            _ => None,
                                        };

                                    let now: NaiveDateTime = Utc::now().naive_utc();

                                    if let Some(updated_receiver_balance) = updated_receiver_balance
                                    {
                                        let now: NaiveDateTime = Utc::now().naive_utc();
                                        query!(
                                            "
                                UPDATE wallets
                                SET balance = $1, updated_at = $2
                                WHERE id = $3
                                ",
                                            updated_receiver_balance,
                                            now,
                                            receiver_wallet.id
                                        )
                                        .fetch_all(&*pool)
                                        .await
                                        .expect("Failed to update sender's wallet balance");

                                    
                                    } else {
                                        // Handle case when balance or amount is None
                                        return (
                                            StatusCode::BAD_REQUEST,
                                            Json(json!({
                                                "status": "error",
                                                "message": "Invalid wallet balance or amount"
                                            })),
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error fetching user wallet: {:?}", e);
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(
                                        json!({"status": "error", "message": "Failed to fetch user wallet"}),
                                    ),
                                );
                            }
                        }

                        // Insert transaction record

                        let transaction_result = query!(

                            "
                            INSERT INTO transactions (sender_id, recipient_id, amount, currency, status,transaction_type  )
                            VALUES ($1, $2, $3, $4, $5, $6)
                            RETURNING *",
                            
                            sender_id,
                            recipient_id,
                            amount,
                            currency, 
                            status, 
                            transaction_type
                        )
                            .fetch_one(&*pool)
                            .await;

                        return (StatusCode::OK, Json(json!({"status": "success"})));
                    } else {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(json!({"status": "error", "message": "Insufficient funds"})),
                        );
                    }
                } else {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"status": "error", "message": "User wallet not found"})),
                    );
                }
            }
            Err(e) => {
                println!("Error fetching user wallet: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"status": "error", "message": "Failed to fetch user wallet"})),
                );
            }
        }
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"status": "error", "message": "User not found"})),
        );
    }
}



// pub async fn edit_user(
//     Path(id): Path<i32>,
//     edit_user_data: Json<EditUser>,
//     pool: Arc<PgPool>
// ) -> impl IntoResponse {

//     // let first_name = edit_user_data.first_name.clone();
//     // let last_name = edit_user_data.last_name.clone();
//     let email = edit_user_data.email.clone();

//     // let completed = edit_todo_data.completed;

//     // UPDATE users
//     // SET first_name = coalesce($2, first_name),
//     // last_name = coalesce($3, last_name),
//     // email = coalesce($4, email)
//     // WHERE id = $1
//     // RETURNING *

//     let update_result = query!(
//         "
//         UPDATE users
//         SET email = coalesce($2, email)
//         WHERE id = $1
//         RETURNING *",
//         id,
//     email)
//         .fetch_one(&*pool)
//         .await;

//     if update_result.is_ok() {
//         // Return a success response
//         Json(json!({
//             "status": "success",
//             "message": format!("User with ID {} edited", id)
//         }))
//     } else {
//         // Return an error response
//         Json(json!({
//             "status": "error",
//             "message": "Failed to update user"
//         }))
//     }

// }

// pub async fn delete_user(
//     Path(id): Path<i32>,
//     pool: Arc<PgPool>
// ) -> impl IntoResponse {
//     println!("delete_todo id = {}", id);

//     // Use the id to delete the item from the database
//     let delete_result = query!(
//         "DELETE FROM users WHERE id = $1",
//         id
//     )
//         .execute(&*pool)
//         .await;

//     if delete_result.is_ok() {
//         // Return a success response
//         Json(json!({
//             "status": "success",
//             "message": format!("User with ID {} deleted", id)
//         }))
//     } else {
//         // Return an error response
//         Json(json!({
//             "status": "error",
//             "message": format!("Failed to delete user with ID {}", id)
//         }))
//     }

// }
