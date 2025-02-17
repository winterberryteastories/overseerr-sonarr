use std::time::Duration;
use tokio::time;

use std::env;

use env_logger::Env;
use log::{info, trace};

use sonarr::apis::configuration::{Configuration as SonarrConfig, ApiKey};
use sonarr::apis::{command_api, tag_api, series_api, series_editor_api, quality_profile_api};
use sonarr::models::command_resource::CommandResource;
use sonarr::models::series_resource::SeriesResource;
use sonarr::models::series_editor_resource::SeriesEditorResource;
use sonarr::models::series_types::SeriesTypes;
use sonarr::models::apply_tags::ApplyTags;

#[derive(Debug)]
struct Config<'a> {
    sonarr_config: SonarrConfig,
    tag_id: i32,
    anime_tag_ids: &'a [i32],
    additional_language: Option<String>,
    additional_language_quality_profile: Option<String>,
    additional_language_root_folder: Option<String>,
}

async fn update_series_quality_profile(
    config: &Config<'_>, series_editor: &mut SeriesEditorResource,
    quality_profile: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sonarr_config = &config.sonarr_config;

    let quality_profiles = quality_profile_api::list_quality_profile(sonarr_config).await?;
    let quality_profile = quality_profiles.iter().find(|&cp| {
        match &cp.name {
            Some(Some(name)) => {
                name == quality_profile
            },
            _ => false
        }
    }).expect("Could not find quality profile");

    series_editor.quality_profile_id = Some(Some(quality_profile.id.unwrap()));
    Ok(())
}

async fn process_series(
    config: &Config<'_>, series: &SeriesResource)
    -> Result<(), Box<dyn std::error::Error>> {
        info!("Processing series: {}", series.title.clone()
            .unwrap_or(Some("?".to_string())).unwrap_or("?".to_string()));

        let sonarr_config = &config.sonarr_config;

        let is_anime = match &series.tags {
            Some(Some(tags)) => config.anime_tag_ids.iter().any(|id| tags.contains(id)),
            _ => false,
        };

        let is_additional_language = match &config.additional_language {
            Some(additional_language) => {
                match &series.original_language {
                    Some(original_language) => {
                        match &original_language.name {
                            Some(Some(name)) => {
                                name == additional_language
                            },
                            _ => false,
                        }
                    },
                    _ => false,
                }
            },
            _ => false,
        };

        let series_id = series.id.unwrap();
        let mut series_editor = SeriesEditorResource {
            series_ids: Some(Some(vec![series_id])),
            ..Default::default()
        };

        if is_anime {
            trace!("Series is anime, updating SeriesType to Standard");
            // let's set all anime to series type Standard.
            series_editor.series_type = Some(SeriesTypes::Standard);
        } else if is_additional_language {
            match &config.additional_language_root_folder {
                Some(root_folder) => {
                    trace!("Original language is {}, updating root folder to {}",
                        config.additional_language.clone().unwrap(),
                        &root_folder,
                    );
                    series_editor.root_folder_path = Some(Some(root_folder.to_string()))
                },
                _ => (),
            }

            match &config.additional_language_quality_profile {
                Some(quality_profile) => {
                    trace!("Original language is {}, updating quality profile to {}",
                        config.additional_language.clone().unwrap(),
                        &quality_profile,
                    );
                    update_series_quality_profile(config,
                        &mut series_editor, quality_profile).await?
                },
                _ => (),
            }
        }

        series_editor.apply_tags = Some(ApplyTags::Remove);
        series_editor.tags = Some(Some(vec![config.tag_id]));

        // update series
        series_editor_api::put_series_editor(sonarr_config, Some(series_editor)).await?;

        trace!("Removed tag from series");

        // manually search series
        let command_resource = CommandResource {
            name: Some(Some("SeriesSearch".to_string())),
            series_id: Some(series_id),
            ..Default::default()
        };
        command_api::create_command(sonarr_config, Some(command_resource)).await?;

        trace!("Removed tag from series");

        Ok(())
}

async fn main_loop(config: &Config<'_>)
    -> Result<(), Box<dyn std::error::Error>> {
        let mut vec_series = series_api::list_series(&config.sonarr_config, None, None).await?;
        vec_series.retain(|s| {
            let tags = match &s.tags {
                Some(Some(tags)) => tags,
                _ => return false,
            };
            tags.contains(&config.tag_id)
        });

        for series in vec_series {
            process_series(config, &series).await?;
        }
        Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenvy::dotenv()?;

    let sonarr_url = env::var("SONARR_URL")?;
    let sonarr_api_key = env::var("SONARR_API_KEY")?;
    let sonarr_config = SonarrConfig {
       base_path: sonarr_url.to_string(),
       api_key: Some(ApiKey {
           key: sonarr_api_key.to_string(),
           prefix: None,
       }),
       ..Default::default()
    };

    let interval: u64 = env::var("RUN_INTERVAL")
        .unwrap_or("60".to_string()).parse()?;
    let mut interval = time::interval(Duration::from_secs(interval));

    let queue_tag_name = env::var("QUEUE_TAG_NAME")
        .unwrap_or("overseerr-queue".to_string());

    let anime_tags: Vec<String> = match env::var("ANIME_TAG_NAMES") {
        Ok(anime_tag_names) =>
            anime_tag_names
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
        _ => vec![],
    };

    let additional_language = env::var("ADDITIONAL_LANGUAGE").ok();
    let additional_language_quality_profile = env::var("ADDITIONAL_LANGUAGE_QUALITY_PROFILE").ok();
    let additional_language_root_folder = env::var("ADDITIONAL_LANGUAGE_ROOT_FOLDER").ok();

    let tags = tag_api::list_tag(&sonarr_config).await?;
    let tag = tags.iter().find(|&tag| {
        match &tag.label {
            Some(Some(label)) => {
                label == &queue_tag_name
            },
            _ => false
        }
    }).expect("Could not find overseerr-queue tag");
    let tag_id = tag.id.expect("Could not find {queue_tag_name} tag");

    let anime_tag_ids: Vec<i32> = anime_tags.iter().map(|tag_name| {
        tags.iter()
            .find(|tag| matches!(&tag.label, Some(Some(label)) if label == tag_name))
            .and_then(|tag| tag.id)
            .unwrap_or_else(|| panic!("Could not find tag: {}", tag_name))
    }).collect();
    let anime_tag_ids: &[i32] = &anime_tag_ids;

    let config = Config {
        sonarr_config,
        tag_id,
        anime_tag_ids,
        additional_language,
        additional_language_quality_profile,
        additional_language_root_folder,
    };

    info!("Started with interval: {interval:?}");
    trace!("Config: {config:?}");

    loop {
        interval.tick().await;
        main_loop(&config).await?
    }
}
