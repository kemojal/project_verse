use std::sync::Arc;
use axum::extract::Path;

use axum::Json;
use axum::response::IntoResponse;
use bcrypt::{DEFAULT_COST, hash};
use lettre::message::Mailbox;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use serde_json::json;
use sqlx::{PgPool, query, query_as};
use crate::models::user_models::{EditUser, EditUserPassoword, NewUser, SignUpUserEmail, User, UserEmail};


use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::Error as SmtpError;

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

pub async fn signup_user(Json(new_user): Json<SignUpUserEmail>, pool: Arc<PgPool>) -> impl IntoResponse {

    let email = new_user.email;

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
        let verification_code = generate_verification_code();

        // Assuming email and verification_code are Option<String> from earlier in your code

if let Some(email) = email {
    // Unwrap the verification_code Option to get the inner String
    if let Some(inner_verification_code) = verification_code {
        // Call the function with the unwrapped values (Strings)
        if let Err(err) = send_verification_email(&email, &inner_verification_code) {
            // Handle the error if the email sending fails
            eprintln!("Failed to send verification email: {:?}", err);
        }
    } else {
        eprintln!("Verification code is missing!");
        // Handle missing verification code error
    }
} else {
    eprintln!("Email is missing!");
    // Handle missing email error
}


//         let email_message = Message::builder()
//             .from("Sender <kemo3855@gmail.com>".parse().unwrap())
//             .to("Receiver <kemo3855@yahoo.com>".parse().unwrap())
//             .subject("Sending email with Rust")
//             .body(String::from("This is my first email"))
//             .unwrap();

//         let creds = Credentials::new("kemo3855@gmail.com".to_string(), "bmhv cwln qigw vzhc".to_string());

// // Open a remote connection to gmail
//         let mailer = SmtpTransport::relay("smtp.gmail.com")
//             .unwrap()
//             .credentials(creds)
//             .build();

// // Send the email
//         match mailer.send(&email_message) {
//             Ok(_) => println!("Email sent successfully!"),
//             Err(e) => panic!("Could not send email: {:?}", e),
//         }

        // INSERT INTO users (first_name, last_name, email, password, registration_date)
        let query_result = query!(

            "
            INSERT INTO users (email, verification_code )
            VALUES ($1, $2)
            RETURNING *",
            email,
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
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                // Handle error case
                // You can return an error response or customize it as needed
                // For now, let's return a generic error response
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to create a workspace: {:?}", e)
                }))
            }
        }

    }


}

fn generate_verification_code() -> String {
    let length = 11;
    let  rng = thread_rng();

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
        .subject("Your login code for Projectverse")
        .body(format!("Your verification code is: {}", verification_code)) // Include verification code in the body
        .unwrap();

    // Replace with your SMTP credentials
    let creds = Credentials::new("kemo3855@gmail.com".to_string(), "bmhv cwln qigw vzhc".to_string());

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