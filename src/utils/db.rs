use aodata_models::json;
use sqlx::{postgres::PgQueryResult, PgPool};

pub struct Item {
    pub unique_name: String,
    pub enchantment_level: i32,
    pub shop_sub_category_id: Option<String>,
    pub tier: Option<i32>,
    pub weight: Option<f32>
}

pub async fn insert_locations(
    pool: &PgPool,
    locations: &Vec<json::Location>,
) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await.unwrap();

    let mut location_ids: Vec<String> = Vec::new();
    let mut location_names: Vec<String> = Vec::new();

    for location in locations {
        location_ids.push(location.id.to_string());
        location_names.push(location.name.to_string());
    }

    sqlx::query!(
        "
INSERT INTO location (
id,
name)
SELECT DISTINCT ON (id) id, name FROM UNNEST(
    $1::VARCHAR[],
    $2::VARCHAR[])
AS t(id, name)
ON CONFLICT (id) DO UPDATE
    SET name = EXCLUDED.name",
        &location_ids,
        &location_names
    )
    .execute(pool)
    .await?;

    transaction.commit().await.unwrap();

    return Ok(());
}

pub async fn insert_items(
    pool: &PgPool,
    json_items: &Vec<json::Item>,
) -> Result<PgQueryResult, sqlx::Error> {

    let mut items: Vec<Item> = Vec::new();

    for json_item in json_items {
        let mut unique_name = json_item.unique_name.clone();

        let parsed_enchantment_level = json_item.enchantment_level.as_ref()
            .map_or(0, |enchantment_level| enchantment_level.parse::<i32>().unwrap_or(0));

        if parsed_enchantment_level > 0 {
            unique_name = format!("{}@{}", unique_name, parsed_enchantment_level);
        }

        let shop_sub_category = json_item.shop_sub_category.clone();

        let parsed_tier = json_item.tier.as_ref()
            .and_then(|tier| tier.parse::<i32>().ok());

        let parsed_weight = json_item.weight.as_ref()
            .and_then(|weight| weight.parse::<f32>().ok());


        items.push(Item {
            unique_name: unique_name,
            enchantment_level: parsed_enchantment_level,
            shop_sub_category_id: shop_sub_category,
            tier: parsed_tier,
            weight: parsed_weight
        });

        if let Some(json_enchantment) = &json_item.enchantments {
            for enchantment in &json_enchantment.enchantment {
                let enchantment_level = enchantment.enchantment_level.parse::<i32>().unwrap_or(0);

                let unique_name = format!("{}@{}", json_item.unique_name.clone(), enchantment_level);

                let weight = enchantment.weight.as_ref()
                    .and_then(|weight| weight.parse::<f32>().ok()).or(parsed_weight);


                items.push(Item {
                    unique_name: unique_name,
                    enchantment_level: enchantment_level,
                    shop_sub_category_id: json_item.shop_sub_category.clone(),
                    tier: parsed_tier.clone(),
                    weight: weight
                });
            }
        }
    }

    let item_unique_names: Vec<String> = items.iter().map(|item| item.unique_name.clone()).collect();
    let enchantment_levels: Vec<i32> = items.iter().map(|item| item.enchantment_level).collect();
    let shop_sub_category_ids: Vec<Option<String>> = items.iter().map(|item| item.shop_sub_category_id.clone()).collect();
    let tiers: Vec<Option<i32>> = items.iter().map(|item| item.tier.clone()).collect();
    let weights: Vec<Option<f32>> = items.iter().map(|item| item.weight.clone()).collect();

    let transaction = pool.begin().await.unwrap();

    let result = sqlx::query(
        "
INSERT INTO item_data (
    item_unique_name,
    enchantment_level,
    shop_sub_category_id,
    tier,
    weight)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::INT[],
    $3::VARCHAR[],
    $4::INT[],
    $5::FLOAT8[])
ON CONFLICT (item_unique_name) DO UPDATE
    SET enchantment_level = EXCLUDED.enchantment_level,
        shop_sub_category_id = EXCLUDED.shop_sub_category_id,
        tier = EXCLUDED.tier,
        weight = EXCLUDED.weight",
    )
    .bind(&item_unique_names)
    .bind(&enchantment_levels)
    .bind(&shop_sub_category_ids)
    .bind(&tiers)
    .bind(&weights)
    .execute(pool)
    .await;

    transaction.commit().await.unwrap();

    return result;
}

pub async fn insert_shop_categories(
    pool: &PgPool,
    shop_categories: &Vec<json::ShopCategory>,
) -> Result<PgQueryResult, sqlx::Error> {
    let mut ids: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();

    for shop_category in shop_categories {
        ids.push(shop_category.id.clone());
        names.push(shop_category.value.clone());
    }

    let transaction = pool.begin().await.unwrap();

    let result = sqlx::query!(
        "
INSERT INTO shop_category (
    id,
    name)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::VARCHAR[])
ON CONFLICT DO NOTHING",
        &ids,
        &names
    )
    .execute(pool)
    .await;

    transaction.commit().await.unwrap();

    return result;
}

pub async fn insert_shop_sub_categories(
    pool: &PgPool,
    shop_categories: &Vec<json::ShopCategory>,
) -> Result<PgQueryResult, sqlx::Error> {
    let mut ids: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut shop_category_ids: Vec<String> = Vec::new();

    for shop_category in shop_categories {
        for shop_sub_category in &shop_category.shop_sub_category {
            ids.push(shop_sub_category.id.clone());
            names.push(shop_sub_category.value.clone());
            shop_category_ids.push(shop_category.id.clone());
        }
    }

    let transaction = pool.begin().await.unwrap();

    let result = sqlx::query!(
        "
INSERT INTO shop_sub_category (
    id,
    name,
    shop_category_id)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::VARCHAR[],
    $3::VARCHAR[])
ON CONFLICT DO NOTHING",
        &ids,
        &names,
        &shop_category_ids
    )
    .execute(pool)
    .await;

    transaction.commit().await.unwrap();

    return result;
}

pub async fn insert_localizations(
    pool: &PgPool,
    localizations: &Vec<json::Localization>,
) -> Result<(), sqlx::Error> {
    let mut item_unique_names: Vec<String> = Vec::new();

    let mut descriptions_item_unique_names: Vec<String> = Vec::new();
    let mut descriptions_langs: Vec<String> = Vec::new();
    let mut descriptions_values: Vec<String> = Vec::new();

    let mut names_item_unique_names: Vec<String> = Vec::new();
    let mut names_langs: Vec<String> = Vec::new();
    let mut names_values: Vec<String> = Vec::new();

    for localization in localizations {
        let item_unique_name = &localization.item;

        item_unique_names.push(item_unique_name.clone());

        if let Some(localized_descriptions) = &localization.localized_descriptions {
            localized_descriptions.iter().for_each(|(lang, value)| {
                descriptions_item_unique_names.push(item_unique_name.clone());
                descriptions_langs.push(lang.clone());
                descriptions_values.push(value.clone());
            });
        }

        if let Some(localized_names) = &localization.localized_names {
            localized_names.iter().for_each(|(lang, value)| {
                names_item_unique_names.push(item_unique_name.clone());
                names_langs.push(lang.clone());
                names_values.push(value.clone());
            });
        }
    }

    let transaction = pool.begin().await?;

    sqlx::query!(
        "
INSERT INTO item (
    unique_name)
SELECT DISTINCT ON (unique_name) unique_name FROM UNNEST(
    $1::VARCHAR[])
AS t(unique_name)
ON CONFLICT DO NOTHING",
        &item_unique_names
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "
INSERT INTO localized_description (
    item_unique_name,
    lang,
    description)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::VARCHAR[],
    $3::VARCHAR[])
ON CONFLICT (item_unique_name, lang) DO NOTHING",
        &descriptions_item_unique_names,
        &descriptions_langs,
        &descriptions_values
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "
INSERT INTO localized_name (
    item_unique_name,
    lang,
    name)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::VARCHAR[],
    $3::VARCHAR[])
ON CONFLICT (item_unique_name, lang) DO NOTHING",
        &names_item_unique_names,
        &names_langs,
        &names_values
    )
    .execute(pool)
    .await?;

    transaction.commit().await?;

    return Ok(());
}

/* rust magic
fn wrap<'a>(el: Option<&'a String>) -> Option<&'a str> {
    Some(el?.as_str())
}
*/
