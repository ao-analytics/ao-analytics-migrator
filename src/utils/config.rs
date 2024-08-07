use config::Config;

#[derive(Debug, Config)]
pub struct Config {
    pub items_path: String,
    pub locations_path: String,
    pub localizations_path: String,
    pub items_url: String,
    pub locations_url: String,
    pub localizations_url: String,
    pub database_url: String,
    pub skip_download_if_exists: bool,
}
