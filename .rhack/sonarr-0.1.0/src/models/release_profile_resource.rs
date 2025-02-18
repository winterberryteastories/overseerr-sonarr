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
pub struct ReleaseProfileResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "required", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub required: Option<Option<Vec<String>>>,
    #[serde(rename = "ignored", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ignored: Option<Option<Vec<String>>>,
    #[serde(rename = "indexerId", skip_serializing_if = "Option::is_none")]
    pub indexer_id: Option<i32>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
}

impl ReleaseProfileResource {
    pub fn new() -> ReleaseProfileResource {
        ReleaseProfileResource {
            id: None,
            name: None,
            enabled: None,
            required: None,
            ignored: None,
            indexer_id: None,
            tags: None,
        }
    }
}

