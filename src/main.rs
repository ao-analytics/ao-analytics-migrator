use aodata_models::json;
use tracing::{info, warn};

#[macro_use]
extern crate dotenv_codegen;

mod utils;

#[tokio::main]
async fn main() {
    let config = utils::config::Config::from_env();

    let config = match config {
        Some(config) => config,
        None => {
            panic!("Failed to initialize config");
        }
    };

    tracing_subscriber::fmt::init();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
        .unwrap();

    info!("Starting migration...");

    let shop_categories =
        utils::json::get_shop_categories_from_file(&config.items_path).unwrap();
    let result = utils::db::insert_shop_categories(&pool, &shop_categories).await;

    match result {
        Ok(_) => info!("Inserted shop categories"),
        Err(e) => warn!("Error inserting shop categories: {}", e),
    }

    let result = utils::db::insert_shop_sub_categories(&pool, &shop_categories).await;

    match result {
        Ok(_) => info!("Inserted shop subcategories"),
        Err(e) => warn!("Error inserting shop subcategories: {}", e),
    }

    let localizations: Vec<json::Localization> =
        utils::json::get_localizations_from_file(&config.localizations_path).unwrap();
    let locations: &Vec<json::Location> =
        &utils::json::get_locations_from_file(&config.locations_path).unwrap();

    let result = utils::db::insert_localizations(&pool, &localizations).await;

    match result {
        Ok(_) => info!("Inserted localizations"),
        Err(e) => warn!("Error inserting localizations: {}", e),
    }

    let items = utils::json::get_items_from_file(dotenv!("ITEMS_PATH")).unwrap();
    let result = utils::db::insert_items(&pool, &items).await;

    match result {
        Ok(_) => info!("Inserted items"),
        Err(e) => warn!("Error inserting items: {}", e),
    }

    let result = utils::db::insert_locations(&pool, locations).await;

    match result {
        Ok(_) => info!("Inserted locations"),
        Err(e) => warn!("Error inserting locations: {}", e),
    }

    pool.close().await;
}
