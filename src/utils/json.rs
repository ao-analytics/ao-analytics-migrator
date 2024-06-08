use aodata_models::json;
use tracing::{info, warn};

pub fn get_localizations_from_file(path: &str) -> Option<Vec<json::Localization>> {
    return match std::fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(localizations) => Some(localizations),
            Err(e) => {
                warn!("Error parsing localizations file: {}", e);
                None
            }
        },
        Err(e) => {
            warn!("Error reading localizations file: {}", e);
            None
        }
    };
}

pub fn get_locations_from_file(path: &str) -> Option<Vec<json::Location>> {
    return match std::fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(locations) => Some(locations),
            Err(e) => {
                warn!("Error parsing locations file: {}", e);
                None
            }
        },
        Err(e) => {
            warn!("Error reading locations file: {}", e);
            None
        }
    };
}

pub fn get_items_from_file(path: &str) -> Option<Vec<json::Item>> {
    return match std::fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str::<json::ItemsJson>(&content) {
            Ok(items) => {
                let mut items_vec: Vec<json::Item> = Vec::new();
                items_vec.push(items.items.hideout_item);
                items
                    .items
                    .tracking_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .farmable_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .simple_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .consumable_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .consumable_from_inventory_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .equipment_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .weapon
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .mount
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .furniture_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .mount_skin
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .journal_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .labourer_contract
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .transformation_weapon
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items
                    .items
                    .crystal_league_item
                    .into_iter()
                    .for_each(|item| items_vec.push(item));
                items_vec.push(items.items.kill_trophy);
                Some(items_vec)
            }
            Err(e) => {
                warn!("Error parsing items file: {}", e);
                None
            }
        },
        Err(e) => {
            warn!("Error reading items file: {}", e);
            None
        }
    };
}

pub fn get_shop_categories_from_file(path: &str) -> Option<Vec<json::ShopCategory>> {
    return match std::fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str::<json::ItemsJson>(&content) {
            Ok(items) => Some(items.items.shop_categories.shop_category),
            Err(e) => {
                warn!("Error parsing shop categories file: {}", e);
                None
            }
        },
        Err(e) => {
            warn!("Error reading shop categories file: {}", e);
            None
        }
    };
}

pub async fn save_file_to_disk(path: &str, content: &str) {
    match std::fs::write(path, content) {
        Ok(_) => info!("File saved to disk: {}", path),
        Err(e) => warn!("Error saving file to disk: {}", e),
    }
}

pub async fn get_file_from_url(url: &str) -> Option<String> {
    let result = reqwest::get(url).await;

    let response = match result {
        Ok(response) => response,
        Err(e) => {
            warn!("Error downloading file: {} from {}", e, url);
            return None;
        }
    };

    let content = match response.text().await {
        Ok(content) => content,
        Err(e) => {
            warn!("Error reading downloaded file: {}", e);
            return None;
        }
    };

    return Some(content);
}

pub async fn download_file_to_disk(url: &str, path: &str) {
    let content = get_file_from_url(url).await;

    match content {
        Some(content) => save_file_to_disk(path, &content).await,
        None => (),
    }
}
