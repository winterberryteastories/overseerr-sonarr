/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: v4.0.12.2823
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Revision {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "real", skip_serializing_if = "Option::is_none")]
    pub real: Option<i32>,
    #[serde(rename = "isRepack", skip_serializing_if = "Option::is_none")]
    pub is_repack: Option<bool>,
}

impl Revision {
    pub fn new() -> Revision {
        Revision {
            version: None,
            real: None,
            is_repack: None,
        }
    }
}

