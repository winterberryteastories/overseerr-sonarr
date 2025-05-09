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
pub struct SeriesTitleInfo {
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "titleWithoutYear", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_without_year: Option<Option<String>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "allTitles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub all_titles: Option<Option<Vec<String>>>,
}

impl SeriesTitleInfo {
    pub fn new() -> SeriesTitleInfo {
        SeriesTitleInfo {
            title: None,
            title_without_year: None,
            year: None,
            all_titles: None,
        }
    }
}

