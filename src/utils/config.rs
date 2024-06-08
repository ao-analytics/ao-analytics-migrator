use tracing::error;

pub struct Config {
    pub items_path: String,
    pub locations_path: String,
    pub localizations_path: String,
    pub items_url: String,
    pub locations_url: String,
    pub localizations_url: String,

    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Option<Self> {
        let db_url = get_var_from_env_or_dotenv("DATABASE_URL")?;
        let items_path = get_var_from_env_or_dotenv("ITEMS_PATH")?;
        let locations_path = get_var_from_env_or_dotenv("LOCATIONS_PATH")?;
        let localizations_path = get_var_from_env_or_dotenv("LOCALIZATIONS_PATH")?;
        let items_url = get_var_from_env_or_dotenv("ITEMS_URL")?;
        let locations_url = get_var_from_env_or_dotenv("LOCATIONS_URL")?;
        let localizations_url = get_var_from_env_or_dotenv("LOCALIZATIONS_URL")?;

        Some(Config {
            db_url,
            items_path,
            locations_path,
            localizations_path,
            items_url,
            locations_url,
            localizations_url,
        })
    }
}

fn get_var_from_env_or_dotenv(name: &str) -> Option<String> {
    let var = std::env::var(name).or(dotenv::var(name));

    match var {
        Ok(var) => Some(var),
        Err(_) => {
            error!("{} is not set", name);
            return None;
        }
    }
}
