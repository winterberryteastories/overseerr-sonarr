use std::time::Duration;
use tokio::time;

use clap::Parser;

use std::env;
use dotenvy::dotenv;

use sonarr::apis::configuration::{Configuration, ApiKey};
use sonarr::apis::{command_api, tag_api, series_api, series_editor_api, quality_profile_api};
use sonarr::models::command_resource::CommandResource;
use sonarr::models::series_resource::SeriesResource;
use sonarr::models::series_editor_resource::SeriesEditorResource;
use sonarr::models::series_types::SeriesTypes;
use sonarr::models::apply_tags::ApplyTags;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Time interval to run (in seconds)
    #[arg(short, long, default_value_t = 60)]
    interval: u64,
}

async fn update_series_quality_profile(
    sonarr_config: &Configuration, series_editor: &mut SeriesEditorResource,
    quality_profile: &str) -> Result<(), Box<dyn std::error::Error>> {

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
    sonarr_config: &Configuration, series: &SeriesResource,
    tag_id: i32, anime_tag_id: i32)
    -> Result<(), Box<dyn std::error::Error>> {

        let is_anime = match &series.tags {
            Some(Some(tags)) => tags.clone(),
            _ => Vec::new(),
        }.contains(&anime_tag_id);

        let original_language = match &series.original_language {
            Some(original_language) => {
                match &original_language.name {
                    Some(Some(name)) => {
                        if name == "German" {
                            Some(name)
                        } else {
                            None
                        }
                    },
                    _ => None,
                }
            }
            _ => None,
        };

        let series_id = series.id.unwrap();
        let mut series_editor = SeriesEditorResource {
            series_ids: Some(Some(vec![series_id])),
            ..Default::default()
        };

        if is_anime {
            // let's set all anime to series type Standard and see how it goes
            series_editor.series_type = Some(SeriesTypes::Standard);

            if original_language.is_some() {
                // english anime shows
                update_series_quality_profile(sonarr_config,
                    &mut series_editor, "[DE.Anime] WEB + BR 1080p").await?;
            };
        } else if original_language.is_some() {
            // german regular shows
            series_editor.root_folder_path = Some(Some("/data/media/tv-de".to_string()));

            update_series_quality_profile(sonarr_config,
                &mut series_editor, "[DE] WEB + BR 1080p").await?;
        }

        series_editor.apply_tags = Some(ApplyTags::Remove);
        series_editor.tags = Some(Some(vec![tag_id]));

        // update series
        series_editor_api::put_series_editor(sonarr_config, Some(series_editor)).await?;

        // manually search series
        let command_resource = CommandResource {
            name: Some(Some("SeriesSearch".to_string())),
            // command_name: Some(Some("SeriesSearch".to_string())),
            series_id: Some(series_id),
            ..Default::default()
        };
        command_api::create_command(sonarr_config, Some(command_resource)).await?;

        Ok(())
}

async fn main_loop(sonarr_config: &Configuration, tag_id: i32, anime_tag_id: i32)
    -> Result<(), Box<dyn std::error::Error>> {
        let mut vec_series = series_api::list_series(&sonarr_config, None, None).await?;
        vec_series.retain(|s| {
            let tags = match &s.tags {
                Some(Some(tags)) => tags,
                _ => return false,
            };
            tags.contains(&tag_id)
        });

        for series in vec_series {
            process_series(sonarr_config, &series, tag_id, anime_tag_id).await?;
        }
        Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut interval = time::interval(Duration::from_secs(args.interval));

    dotenvy::dotenv()?;

    let sonarr_url = env::var("SONARR_URL")?;
    let sonarr_api_key = env::var("SONARR_API_KEY")?;
    let sonarr_config = Configuration {
       base_path: sonarr_url.to_string(),
       api_key: Some(ApiKey {
           key: sonarr_api_key.to_string(),
           prefix: None,
       }),
       ..Default::default()
    };

    let tags = tag_api::list_tag(&sonarr_config).await?;
    let tag = tags.iter().find(|&tag| {
        match &tag.label {
            Some(Some(label)) => {
                label == "overseerr-queue"
            },
            _ => false
        }
    }).expect("Could not find overseerr-queue tag");
    let tag_id = tag.id.expect("Could not find overseerr-queue tag");
    let anime_tag = tags.iter().find(|&tag| {
        match &tag.label {
            Some(Some(label)) => {
                label == "anime"
            },
            _ => false
        }
    }).expect("Could not find anime tag");
    let anime_tag_id = anime_tag.id.expect("Could not find anime tag");

    loop {
        interval.tick().await;
        main_loop(&sonarr_config, tag_id, anime_tag_id).await?
    }
}
