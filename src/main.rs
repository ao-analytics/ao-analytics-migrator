use aodata_models::json;
use bytes::Bytes;
use tracing::{info, warn};

#[macro_use]
extern crate dotenv_codegen;

mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_url = dotenv!("DATABASE_URL");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap();

    info!("Starting migration...");

    let localizations_bytes = get_bytes_from_url(dotenv!("LOCALIZATIONS_URL"))
        .await
        .unwrap();

    info!("Got localizations...");

    let locations_bytes = get_bytes_from_url(dotenv!("LOCATIONS_URL")).await.unwrap();

    info!("Got locations...");

    let localizations: Vec<json::Localization> =
        serde_json::from_slice(&localizations_bytes).unwrap();
    let locations: &Vec<json::Location> = &serde_json::from_slice(&locations_bytes).unwrap();

    let result = utils::db::insert_items(&pool, &localizations).await;

    match result {
        Ok(_) => info!("Inserted items"),
        Err(e) => warn!("Error inserting items: {}", e),
    }

    let result = utils::db::insert_localizations(&pool, localizations).await;

    match result {
        Ok(_) => info!("Inserted localizations"),
        Err(e) => warn!("Error inserting localizations: {}",e),
    }

    let result = utils::db::insert_locations(&pool, locations).await;

    match result {
        Ok(_) => info!("Inserted locations"),
        Err(e) => warn!("Error inserting locations: {}", e),
    }

    pool.close().await;
}

async fn get_bytes_from_url(url: &str) -> Result<Bytes, reqwest::Error> {
    let bytes = reqwest::get(url).await?.bytes().await?;

    Ok(bytes)
}
