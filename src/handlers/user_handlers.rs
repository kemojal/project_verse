use axum::extract::{Path, State};
use std::sync::Arc;

use crate::models::user_models::{
    EditUser, EditUserPassoword, NewUser, User, UserEmail, UserId, UserPhoneNumber, UserToVerify,
    VerifyUser,
};
use crate::models::wallet_models::Balance;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Duration, Utc};
use lettre::message::Mailbox;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::json;
use sqlx::{query, query_as, PgPool};

use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error as SmtpError;
use lettre::{Message, SmtpTransport, Transport};

use reqwest::Client;

pub async fn get_users(
    State(pool): State<Arc<PgPool>>,
    // pool: Arc<PgPool>
) -> impl IntoResponse {
    let users: Vec<User> = query_as!(
        User,
        r#"
        SELECT id, email, password_hash, verification_code,  verified,  created_at, updated_at FROM users
        "#
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch appointments");

    Json(users)
}

pub async fn get_user_profile(
    Path(email): Path<String>,
    // pool: Arc<PgPool>
    State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    let user: Vec<User> = query_as!(
        User,
        "SELECT id, email, password_hash, verification_code,  verified,  created_at, updated_at FROM users WHERE email = $1",
        email
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch appointments");

    Json(user)
}

pub async fn get_user_balance(
    Path(email): Path<String>,
    State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    let db_user_id: Vec<UserId> = query_as!(UserId, "SELECT id FROM users WHERE email = $1", email)
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch user id");

    if !db_user_id.is_empty() {
        let user_id = &db_user_id[0].id; // Assuming UserId is a tuple struct with a single field
        let wallet_balance: Option<Balance> = query_as!(
            Balance,
            "SELECT balance FROM wallets WHERE user_id = $1",
            user_id
        )
        .fetch_one(&*pool)
        .await
        .ok();

        if let Some(balance) = wallet_balance {
            return Json(json!({ "balance": balance }));
        } else {
            return Json(json!({ "error": "No wallet found for this user" }));
        }
    } else {
        return Json(json!({ "error": "User not found" }));
    }
}

// pub async fn get_today_total_paid(
//     Path(email): Path<String>,
//     pool: Arc<PgPool>,
// ) -> impl IntoResponse {
//     let db_user_id: Vec<UserId> = query_as!(UserId, "SELECT id FROM users WHERE email = $1", email)
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch user id");

//     if !db_user_id.is_empty() {
//         let user_id = &db_user_id[0].id; // Assuming UserId is a tuple struct with a single field

//         // let today = Utc::now().date();
//         // let today_start = today.and_hms(0, 0, 0);
//         let today_start = DateTime::<Utc>::now()
//             .date()
//             .naive_utc()
//             .date()
//             .and_hms(0, 0, 0);
//         let today_end = today_start + chrono::Duration::days(1);
//         let today_end = today_start + chrono::Duration::days(1);

//         let total_paid: Option<BalanceAmount> = query_as!(
//             BalanceAmount,
//             "SELECT COALESCE(SUM(amount), 0) FROM transactions
//              WHERE sender_id = $1
//              AND transaction_date >= $2
//              AND transaction_date < $3",
//             user_id,
//             today_start,
//             today_end
//         )
//         .fetch_one(&*pool)
//         .await
//         .ok();

//         if let Some(total) = total_paid {
//             return Json(json!({ "total_paid": total }));
//         } else {
//             return Json(json!({ "error": "Error fetching total paid" }));
//         }
//     } else {
//         return Json(json!({ "error": "User not found" }));
//     }
// }

// pub async fn get_today_total_received(
//     Path(email): Path<String>,
//     pool: Arc<PgPool>,
// ) -> impl IntoResponse {
//     let db_user_id: Vec<UserId> = query_as!(UserId, "SELECT id FROM users WHERE email = $1", email)
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch user id");

//     if !db_user_id.is_empty() {
//         let user_id = &db_user_id[0].id; // Assuming UserId is a tuple struct with a single field

//         // let today = Utc::now().date();
//         // let today_start = today.and_hms(0, 0, 0);
//         let now = Utc::now();
//         let today_start: NaiveDateTime = now.naive_utc().date().and_hms_opt(0, 0, 0);
//         let today_end = today_start + chrono::Duration::days(1);

//         let total_received: Option<BalanceAmount> = query_as!(
//             BalanceAmount,
//             "SELECT COALESCE(SUM(amount), 0) FROM transactions
//              WHERE recipient_id = $1
//              AND transaction_date >= $2
//              AND transaction_date < $3",
//             user_id,
//             today_start,
//             today_end
//         )
//         .fetch_one(&*pool)
//         .await
//         .ok();

//         if let Some(total) = total_received {
//             return Json(json!({ "total_received": total }));
//         } else {
//             return Json(json!({ "error": "Error fetching total received" }));
//         }
//     } else {
//         return Json(json!({ "error": "User not found" }));
//     }
// }
// async fn get_today_total_paid(Path(email): Path<String>, pool: Arc<PgPool>) -> impl IntoResponse {
//     let now = Utc::now();
//     let today_start = Utc.timestamp(now.timestamp().date().and_hms(0, 0, 0).timestamp(), 0); // Start of the current day

//     let db_user_id: Vec<UserId> =
//         sqlx::query_as!(UserId, "SELECT id FROM users WHERE email = $1", email)
//             .fetch_all(&*pool)
//             .await
//             .expect("Failed to fetch user id");

//     if !db_user_id.is_empty() {
//         let user_id = db_user_id[0].id;
//         let total_paid: Option<f64> = sqlx::query!(
//             "SELECT SUM(amount) as total_paid FROM transactions WHERE sender_id = $1 AND transaction_date >= $2",
//             user_id,
//             today_start.naive_utc()
//         )
//         .fetch_one(&*pool)
//         .await
//         .map(|row| row.total_paid.map(|amount| amount.to_f64().unwrap_or(0.0))); // Convert BigDecimal to f64

//         match total_paid {
//             Some(amount) => Json(json!({ "total_paid": amount })),
//             None => Json(json!({ "total_paid": 0 })),
//         }
//     } else {
//         Json(json!({ "error": "User not found" }))
//     }
// }

pub async fn create_user(Json(new_user): Json<NewUser>, pool: Arc<PgPool>) -> impl IntoResponse {
    // let first_name = new_user.first_name;
    // let last_name = new_user.last_name;
    let email = new_user.email;
    let password = new_user.password;
    let phone_number = new_user.phone_number;
    let user_name = new_user.user_name;

    let users_emails: Option<UserEmail> =
        query_as!(UserEmail, "SELECT email FROM users WHERE email = $1", email)
            .fetch_optional(&*pool)
            .await
            .expect("Failed to fetch user");

    let users_phone_numbers: Option<UserPhoneNumber> = query_as!(
        UserPhoneNumber,
        "SELECT phone_number FROM users WHERE phone_number = $1",
        phone_number
    )
    .fetch_optional(&*pool)
    .await
    .expect("Failed to fetch user");

    if users_phone_numbers.is_some() {
        // Phone number already exists
        return Json(json!({
            "status": "error",
            "message": "User with this phone number already exists"
        }));
    } else if users_emails.is_some() {
        // Email already exists
        Json(json!({
            "status": "error",
            "message": "User with this email already exists"
        }))
    } else {
        let hashed_password = hash_password(&password);
        let verification_code = generate_verification_code();

        let phone_number_clone = phone_number.clone();

        // Send SMS
        let contact_number = match phone_number_clone {
            Some(number) => number,
            None => {
                return Json(json!({
                    "status": "error",
                    "message": "Phone number not provided"
                }));
            }
        };

        // Send SMS
        send_sms(&contact_number, &verification_code)
            .await
            .unwrap_or_else(|e| {
                println!("Error sending SMS: {:?}", e);
            });

        let email_body = format!(
            "Here's your temporary code for accessing Fat Fat:\n\n\
            code: *{}*\n\n\
            As part of our security measures, we require a verification code to complete your registration/process.\n\n\
            Please use this code within the next 10 minutes to proceed with the verification process. If you did not initiate this request, please ignore this email.\n\n\
            Thank you,\n\
            CEO, Jerry",
            verification_code
        );
        // let email_body = format!(
        //     "<html>
        //     <body>
        //         <p style='font-family: Arial, sans-serif; font-size: 14px; color: #333; background-color: #ff0057'>
        //             Thank you for choosing Fat Fat.<br><br>
        //             As part of our security measures, we require a verification code to complete your registration/process.<br><br>
        //             Your verification code is: <b>{}</b><br><br>
        //             Please use this code within the next 10 minutes to proceed with the verification process. If you did not initiate this request, please ignore this email.<br><br>
        //             Thank you,<br>
        //             CEO, Jerry
        //         </p>
        //     </body>
        //     </html>",
        //     verification_code
        // );
        let user_mail = email.clone();

        let email_message = Message::builder()
            .from("FatFat <kemo3855@gmail.com>".parse().unwrap())
            .to(format!("Receiver <{}>", user_mail.unwrap_or_default())
                .parse()
                .unwrap())
            .subject("Account verification for FatFat")
            .body(email_body)
            .unwrap();

        let creds = Credentials::new(
            "kemo3855@gmail.com".to_string(),
            "bmhv cwln qigw vzhc".to_string(),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email_message) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }

        // INSERT INTO users (first_name, last_name, email, password, registration_date)
        let query_result = query!(
            "
            INSERT INTO users (username, email, password_hash, verification_code, phone_number )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
            // first_name,
            // last_name,
            user_name,
            email,
            hashed_password,
            verification_code,
            phone_number
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

pub async fn verify_user(
    Path(email): Path<String>,
    Json(verification_data): Json<VerifyUser>,
    pool: Arc<PgPool>
    // State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    let code = verification_data.verification_code;

    if let Some(user) = query_as!(
        UserToVerify,
        "SELECT id, email, verified, verification_code, verification_code_created_at FROM users WHERE email = $1 AND verification_code = $2",
        email,
        code,
    )
    .fetch_optional(&*pool)
    .await
    .expect("Failed to fetch user")
    {
        // User found with the provided email and verification code
        if user.verified.unwrap_or(false) {
            // User is already verified
            return Json(json!({
                "status": "success",
                "message": "User is already verified"
            }));
        } else {
            // Check if the verification code has expired
            if let Some(code_creation_time) = user.verification_code_created_at {
                let current_time = Utc::now();
                let code_expiration_time = code_creation_time + Duration::minutes(10);

                let code_expiration_time_utc = DateTime::<Utc>::from_utc(code_expiration_time, Utc);
                if current_time > code_expiration_time_utc {
                    // Verification code has expired
                    return Json(json!({
                        "status": "error",
                        "message": "Verification code has expired"
                    }));
                } else {
                    // Update user's verified status to true
                    query!("UPDATE users SET verified = true WHERE id = $1", user.id)
                        .execute(&*pool)
                        .await
                        .expect("Failed to update user status");

                    return Json(json!({
                        "status": "success",
                        "message": "User verified successfully"
                    }));
                }
            } else {
                // Code creation time is None (verification code not found)
                return Json(json!({
                    "status": "error",
                    "message": "Verification code not found"
                }));
            }
        }
    } else {
        // No user found with the provided email and verification code
        return Json(json!({
            "status": "error",
            "message": "User with this email and verification code does not exist"
        }));
    }
}

pub async fn resend_verification_code(
    Path(email): Path<String>,
    // pool: Arc<PgPool>,
    State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    // Generate a new verification code
    let new_verification_code: String = generate_verification_code();

    // Update the verification code in the database for the user with the provided email
    let update_result = query!(
        "UPDATE users SET verification_code = $1, verification_code_created_at = CURRENT_TIMESTAMP WHERE email = $2 RETURNING id",
        new_verification_code,
        email,
    )
    .fetch_one(&*pool)
    .await;

    if let Ok(user) = update_result {
        // Send the email with the new verification code
        let email_body = format!(
            "Thank you for choosing ProjectVerse.\n\n\
            As part of our security measures, we require a verification code to complete your registration/process.\n\n\
            Your new verification code is: *{}*\n\n\
            Please use this code within the next 10 minutes to proceed with the verification process. If you did not initiate this request, please ignore this email.\n\n\
            Thank you,\n\
            CEO, Jerry",
            new_verification_code
        );

        let email_message = Message::builder()
            .from("ProjectVerse <kemo3855@gmail.com>".parse().unwrap())
            .to(format!("Receiver <{}>", email).parse().unwrap())
            .subject("New Verification Code for ProjectVerse")
            .body(email_body)
            .unwrap();

        let creds = Credentials::new(
            "kemo3855@gmail.com".to_string(),
            "bmhv cwln qigw vzhc".to_string(),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email_message) {
            Ok(_) => {
                // Email sent successfully
                Json(json!({
                    "status": "success",
                    "message": "New verification code sent successfully"
                }))
            }
            Err(e) => {
                // Failed to send email
                println!("Could not send email: {:?}", e);
                Json(json!({
                    "status": "error",
                    "message": "Failed to send verification code email"
                }))
            }
        }
    } else {
        // Failed to update verification code in the database
        Json(json!({
            "status": "error",
            "message": "Failed to update verification code"
        }))
    }
}

async fn send_sms(
    phone_number: &str,
    verification_code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let account_sid = "ACe0278dc21b695259a2d831d2a887fae5";
    let auth_token = "c9c388dd54160f89256e7e8b87b0d3aa";
    let service_sid = "VA61f2dfef6c1ed74eaffa5ff8a2aca098";
    let from_phone_number = "+14696208723";

    let body = format!("Your verification code is: {}", verification_code);

    let client = Client::new();
    let response = client
        .post(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            account_sid
        ))
        .basic_auth(account_sid, Some(auth_token))
        .form(&[
            ("To", phone_number),
            ("From", from_phone_number),
            ("Body", &body),
        ])
        .send()
        .await?;

    if response.status().is_success() {
        println!("SMS sent successfully!");
    } else {
        println!("Failed to send SMS: {:?}", response.text().await?);
    }

    Ok(())
}

fn generate_verification_code() -> String {
    let length = 6;
    let rng = thread_rng();

    let code: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    code
}

fn send_verification_email(email_address: &str, verification_code: &str) -> Result<(), SmtpError> {
    // Create the email message
    let email_message = Message::builder()
        .from(Mailbox::new(None, "Sender <sendermail>".parse().unwrap())) // Replace with sender info
        .to(Mailbox::new(None, email_address.parse().unwrap())) // Email address of the recipient
        .subject("Your login code for Fat fat")
        .body(format!("Your verification code is: {}", verification_code)) // Include verification code in the body
        .unwrap();

    // Replace with your SMTP credentials
    let creds = Credentials::new(
        "kemo3855@gmail.com".to_string(),
        "bmhv cwln qigw vzhc".to_string(),
    );

    // Setup the SMTP transport
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        // .smtp_utf8(true)
        // .authentication_mechanism(Mechanism::Login)
        // .hello_name(ClientId::Domain("example.com".into())) // Replace with your domain
        .build();

    // Send the email
    match mailer.send(&email_message) {
        Ok(_) => {
            println!("Email sent successfully to: {}", email_address);
            Ok(())
        }
        Err(e) => {
            eprintln!("Could not send email: {:?}", e);
            Err(e)
        }
    }
}

pub async fn edit_user(
    Path(id): Path<i32>,
    edit_user_data: Json<EditUser>,
    pool: Arc<PgPool>,
    // State(pool): State<Arc<PgPool>>,
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
        email
    )
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
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    // let first_name = edit_user_data.first_name.clone();
    // let last_name = edit_user_data.last_name.clone();
    let email = edit_user_data.email.clone();
    let password = edit_user_data.password.clone();
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
        SET password_hash = coalesce($2, password_hash)
        WHERE id = $1
        RETURNING *",
        id,
        hashed_password
    )
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
    // pool: Arc<PgPool>
    State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    println!("delete_todo id = {}", id);

    // Use the id to delete the item from the database
    let delete_result = query!("DELETE FROM users WHERE id = $1", id)
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


// struct Path<T>(T);
