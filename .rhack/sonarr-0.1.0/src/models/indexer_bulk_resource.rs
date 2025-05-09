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
pub struct IndexerBulkResource {
    #[serde(rename = "ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Option<Vec<i32>>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "applyTags", skip_serializing_if = "Option::is_none")]
    pub apply_tags: Option<models::ApplyTags>,
    #[serde(rename = "enableRss", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_rss: Option<Option<bool>>,
    #[serde(rename = "enableAutomaticSearch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_automatic_search: Option<Option<bool>>,
    #[serde(rename = "enableInteractiveSearch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_interactive_search: Option<Option<bool>>,
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Option<i32>>,
}

impl IndexerBulkResource {
    pub fn new() -> IndexerBulkResource {
        IndexerBulkResource {
            ids: None,
            tags: None,
            apply_tags: None,
            enable_rss: None,
            enable_automatic_search: None,
            enable_interactive_search: None,
            priority: None,
        }
    }
}

