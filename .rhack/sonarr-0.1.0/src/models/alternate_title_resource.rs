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
pub struct AlternateTitleResource {
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "seasonNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<Option<i32>>,
    #[serde(rename = "sceneSeasonNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scene_season_number: Option<Option<i32>>,
    #[serde(rename = "sceneOrigin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scene_origin: Option<Option<String>>,
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
}

impl AlternateTitleResource {
    pub fn new() -> AlternateTitleResource {
        AlternateTitleResource {
            title: None,
            season_number: None,
            scene_season_number: None,
            scene_origin: None,
            comment: None,
        }
    }
}

