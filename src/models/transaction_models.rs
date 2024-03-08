

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub sender_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub transaction_type: Option<NaiveDateTime>,
    pub transaction_date: Option<NaiveDateTime>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub sender_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub amount: Option<sqlx::types::BigDecimal>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub transaction_type: Option<String>,
}

// pub struct DepositAmount {
//     pub amount: Option<sqlx::types::BigDecimal>,
// }

// pub struct WithdrawAmount {
//     pub amount: Option<sqlx::types::BigDecimal>,
// }


