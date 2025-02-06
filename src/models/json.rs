#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Root {
    pub items: Items,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Items {
    #[serde(rename = "shopcategories")]
    pub shop_categories: ShopCategories,
    #[serde(rename = "hideoutitem")]
    pub hideout_item: Item,
    #[serde(rename = "trackingitem")]
    pub tracking_item: Vec<Item>,
    #[serde(rename = "trashitem")]
    pub trash_item: Vec<Item>,
    #[serde(rename = "farmableitem")]
    pub farmable_item: Vec<Item>,
    #[serde(rename = "simpleitem")]
    pub simple_item: Vec<Item>,
    #[serde(rename = "siegebanner")]
    pub siegebanner: Vec<Item>,
    #[serde(rename = "consumableitem")]
    pub consumable_item: Vec<Item>,
    #[serde(rename = "consumablefrominventoryitem")]
    pub consumable_from_inventory_item: Vec<Item>,
    #[serde(rename = "equipmentitem")]
    pub equipment_item: Vec<Item>,
    #[serde(rename = "weapon")]
    pub weapon: Vec<Item>,
    #[serde(rename = "mount")]
    pub mount: Vec<Item>,
    #[serde(rename = "furnitureitem")]
    pub furniture_item: Vec<Item>,
    #[serde(rename = "mountskin")]
    pub mount_skin: Vec<Item>,
    #[serde(rename = "journalitem")]
    pub journal_item: Vec<Item>,
    #[serde(rename = "labourercontract")]
    pub labourer_contract: Vec<Item>,
    #[serde(rename = "transformationweapon")]
    pub transformation_weapon: Vec<Item>,
    #[serde(rename = "crystalleagueitem")]
    pub crystal_league_item: Vec<Item>,
    #[serde(rename = "killtrophy")]
    pub kill_trophy: Vec<Item>,
}

pub use serde_json::Value as Item;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ShopCategories {
    #[serde(rename = "shopcategory")]
    pub shop_categories: Vec<ShopCategory>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ShopCategory {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "shopsubcategory")]
    pub shop_sub_category: Vec<ShopSubCategory>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ShopSubCategory {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(serde::Deserialize)]
pub struct Localization {
    #[serde(rename = "LocalizedNames")]
    pub localized_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LocalizedDescriptions")]
    pub localized_descriptions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UniqueName")]
    pub unique_name: String,
    #[allow(dead_code)]
    #[serde(rename = "Index")]
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct Location {
    #[serde(rename = "Index")]
    pub id: String,
    #[serde(rename = "UniqueName")]
    pub name: String,
}
