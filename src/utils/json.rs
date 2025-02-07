use crate::models::json;
use tracing::{info, warn};

pub fn get_localizations_from_file(path: &str) -> Option<Vec<json::Localization>> {
    match std::fs::read_to_string(path) {
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
    }
}

pub fn get_locations_from_file(path: &str) -> Option<Vec<json::Location>> {
    match std::fs::read_to_string(path) {
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
    }
}

pub fn get_items_from_file(path: &str) -> Option<Vec<json::Item>> {
    match std::fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str::<json::Root>(&content) {
            Ok(items) => {
                let mut items_vec: Vec<json::Item> = Vec::new();
                items_vec.push(items.items.hideout_item);
                items
                    .items
                    .tracking_item
                    .into_iter()
                    .chain(items.items.trash_item)
                    .chain(items.items.farmable_item)
                    .chain(items.items.simple_item)
                    .chain(items.items.siegebanner)
                    .chain(items.items.consumable_item)
                    .chain(items.items.consumable_from_inventory_item)
                    .chain(items.items.equipment_item)
                    .chain(items.items.weapon)
                    .chain(items.items.mount)
                    .chain(items.items.furniture_item)
                    .chain(items.items.mount_skin)
                    .chain(items.items.journal_item)
                    .chain(items.items.labourer_contract)
                    .chain(items.items.transformation_weapon)
                    .chain(items.items.crystal_league_item)
                    .chain(items.items.kill_trophy)
                    .for_each(|item| items_vec.push(item));
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
    }
}

pub async fn save_file_to_disk(path: &str, content: &str) {
    match std::fs::write(path, content) {
        Ok(_) => info!("File saved to disk: {}", path),
        Err(e) => warn!("Error saving file to disk: {}", e),
    }
}

pub async fn get_file_from_url(url: &str) -> Option<String> {
    let client = reqwest::Client::builder().use_rustls_tls().build();

    let client = match client {
        Ok(client) => client,
        Err(e) => {
            warn!("Error creating client: {}", e);
            return None;
        }
    };

    let result = client.get(url).send().await;

    let response = match result {
        Ok(response) => response,
        Err(e) => {
            warn!("Error downloading file: {}", e);
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

    Some(content)
}

pub async fn download_file_to_disk(url: &str, path: &str) {
    let content = get_file_from_url(url).await;

    if let Some(content) = content {
        save_file_to_disk(path, &content).await;
    }
}
