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

/// JobsUploadRequestInnerDataValue : Value for the data labels
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsUploadRequestInnerDataValue {
}

impl JobsUploadRequestInnerDataValue {
    /// Value for the data labels
    pub fn new() -> JobsUploadRequestInnerDataValue {
        JobsUploadRequestInnerDataValue {
        }
    }
}

