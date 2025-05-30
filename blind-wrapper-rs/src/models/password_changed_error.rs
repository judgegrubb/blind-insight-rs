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
pub struct PasswordChangedError {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl PasswordChangedError {
    pub fn new() -> PasswordChangedError {
        PasswordChangedError {
            detail: None,
        }
    }
}

