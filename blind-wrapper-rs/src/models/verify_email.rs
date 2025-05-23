/*
 * Blind Insight REST API
 *
 * End-to-end encrypted datastore
 *
 * The version of the OpenAPI document: 10.6.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyEmail {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    #[serde(rename = "signature")]
    pub signature: String,
}

impl VerifyEmail {
    pub fn new(user_id: String, email: String, timestamp: i32, signature: String) -> VerifyEmail {
        VerifyEmail {
            user_id,
            email,
            timestamp,
            signature,
        }
    }
}

