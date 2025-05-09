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
pub struct IndexerConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "minimumAge", skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<i32>,
    #[serde(rename = "retention", skip_serializing_if = "Option::is_none")]
    pub retention: Option<i32>,
    #[serde(rename = "maximumSize", skip_serializing_if = "Option::is_none")]
    pub maximum_size: Option<i32>,
    #[serde(rename = "rssSyncInterval", skip_serializing_if = "Option::is_none")]
    pub rss_sync_interval: Option<i32>,
}

impl IndexerConfigResource {
    pub fn new() -> IndexerConfigResource {
        IndexerConfigResource {
            id: None,
            minimum_age: None,
            retention: None,
            maximum_size: None,
            rss_sync_interval: None,
        }
    }
}

