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
pub struct RecordsSearchRequestFiltersInner {
    /// Label to search for given value
    #[serde(rename = "label")]
    pub label: String,
    /// Value to search for given label
    #[serde(rename = "value")]
    pub value: String,
}

impl RecordsSearchRequestFiltersInner {
    pub fn new(label: String, value: String) -> RecordsSearchRequestFiltersInner {
        RecordsSearchRequestFiltersInner {
            label,
            value,
        }
    }
}

