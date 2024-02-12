use aodata_models::json;


pub fn get_localizations_from_file(path: &str) -> Option<Vec<json::Localization>> {
    return match std::fs::read_to_string(path) {
        Ok(content) => {
            match serde_json::from_str(&content) {
                Ok(localizations) => Some(localizations),
                Err(e) => {
                    println!("Error parsing localizations file: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Error reading localizations file: {}", e);
            None
        }
    };
}

pub fn get_locations_from_file(path: &str) -> Option<Vec<json::Location>> {
    return match std::fs::read_to_string(path) {
        Ok(content) => {
            match serde_json::from_str(&content) {
                Ok(locations) => Some(locations),
                Err(e) => {
                    println!("Error parsing locations file: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Error reading locations file: {}", e);
            None
        }
    }
}
