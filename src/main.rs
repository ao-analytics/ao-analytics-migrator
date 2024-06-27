use std::path::Path;

use ao_analytics_models::json;
use tracing::{info, warn};
use utils::json::download_file_to_disk;

mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = utils::config::Config::from_env();

    let config = match config {
        Some(config) => config,
        None => {
            panic!("Failed to initialize config");
        }
    };

    if !config.skip_download_if_exists || !Path::new(&config.locations_path).exists() {
        download_file_to_disk(&config.locations_url, &config.locations_path).await;
    }

    if !config.skip_download_if_exists || !Path::new(&config.localizations_path).exists() {
        download_file_to_disk(&config.localizations_url, &config.localizations_path).await;
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap();

    info!("Starting migration...");

    let localizations: Vec<json::localization::Localization> =
        utils::json::get_localizations_from_file(&config.localizations_path).unwrap();
    let locations: &Vec<json::location::Location> =
        &utils::json::get_locations_from_file(&config.locations_path).unwrap();

    let result = utils::db::insert_localizations(&pool, &localizations).await;

    match result {
        Ok(_) => info!("Inserted localizations"),
        Err(e) => warn!("Error inserting localizations: {}", e),
    }

    let result = utils::db::insert_locations(&pool, locations).await;

    match result {
        Ok(_) => info!("Inserted locations"),
        Err(e) => warn!("Error inserting locations: {}", e),
    }

    pool.close().await;
}
