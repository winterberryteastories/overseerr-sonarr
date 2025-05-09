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
pub struct ImportListResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Option<Vec<models::Field>>>,
    #[serde(rename = "implementationName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation_name: Option<Option<String>>,
    #[serde(rename = "implementation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<Option<String>>,
    #[serde(rename = "configContract", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config_contract: Option<Option<String>>,
    #[serde(rename = "infoLink", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub info_link: Option<Option<String>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::ProviderMessage>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "presets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presets: Option<Option<Vec<models::ImportListResource>>>,
    #[serde(rename = "enableAutomaticAdd", skip_serializing_if = "Option::is_none")]
    pub enable_automatic_add: Option<bool>,
    #[serde(rename = "searchForMissingEpisodes", skip_serializing_if = "Option::is_none")]
    pub search_for_missing_episodes: Option<bool>,
    #[serde(rename = "shouldMonitor", skip_serializing_if = "Option::is_none")]
    pub should_monitor: Option<models::MonitorTypes>,
    #[serde(rename = "monitorNewItems", skip_serializing_if = "Option::is_none")]
    pub monitor_new_items: Option<models::NewItemMonitorTypes>,
    #[serde(rename = "rootFolderPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<Option<String>>,
    #[serde(rename = "qualityProfileId", skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<i32>,
    #[serde(rename = "seriesType", skip_serializing_if = "Option::is_none")]
    pub series_type: Option<models::SeriesTypes>,
    #[serde(rename = "seasonFolder", skip_serializing_if = "Option::is_none")]
    pub season_folder: Option<bool>,
    #[serde(rename = "listType", skip_serializing_if = "Option::is_none")]
    pub list_type: Option<models::ImportListType>,
    #[serde(rename = "listOrder", skip_serializing_if = "Option::is_none")]
    pub list_order: Option<i32>,
    #[serde(rename = "minRefreshInterval", skip_serializing_if = "Option::is_none")]
    pub min_refresh_interval: Option<String>,
}

impl ImportListResource {
    pub fn new() -> ImportListResource {
        ImportListResource {
            id: None,
            name: None,
            fields: None,
            implementation_name: None,
            implementation: None,
            config_contract: None,
            info_link: None,
            message: None,
            tags: None,
            presets: None,
            enable_automatic_add: None,
            search_for_missing_episodes: None,
            should_monitor: None,
            monitor_new_items: None,
            root_folder_path: None,
            quality_profile_id: None,
            series_type: None,
            season_folder: None,
            list_type: None,
            list_order: None,
            min_refresh_interval: None,
        }
    }
}

