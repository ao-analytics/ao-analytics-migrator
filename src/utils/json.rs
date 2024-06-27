use ao_analytics_models::json;
use tracing::{info, warn};

pub fn get_localizations_from_file(path: &str) -> Option<Vec<json::localization::Localization>> {
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

pub fn get_locations_from_file(path: &str) -> Option<Vec<json::location::Location>> {
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
