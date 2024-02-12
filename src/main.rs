use aodata_models::json;
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

    let localizations: Vec<json::Localization> = utils::json::get_localizations_from_file(dotenv!("LOCALIZATIONS_PATH")).unwrap();
    let locations: &Vec<json::Location> = &utils::json::get_locations_from_file(dotenv!("LOCATIONS_PATH")).unwrap();

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