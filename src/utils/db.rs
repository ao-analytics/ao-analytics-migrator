use ao_analytics_models::json;
use sqlx::PgPool;

pub async fn insert_locations(
    pool: &PgPool,
    locations: &Vec<json::location::Location>,
) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await?;

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

    transaction.commit().await?;

    Ok(())
}

pub async fn insert_localizations(
    pool: &PgPool,
    localizations: &Vec<json::localization::Localization>,
) -> Result<(), sqlx::Error> {
    let mut item_unique_names: Vec<String> = Vec::new();

    let mut descriptions_item_unique_names: Vec<String> = Vec::new();
    let mut descriptions_langs: Vec<String> = Vec::new();
    let mut descriptions_values: Vec<String> = Vec::new();

    let mut names_item_unique_names: Vec<String> = Vec::new();
    let mut names_langs: Vec<String> = Vec::new();
    let mut names_values: Vec<String> = Vec::new();

    for localization in localizations {
        let item_unique_name = &localization.unique_name;

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

    Ok(())
}

pub async fn insert_item_data(
    pool: &PgPool,
    item_data: &Vec<json::item::Item>,
) -> Result<(), sqlx::Error> {
    let mut item_unique_names = Vec::new();
    let mut item_data_values = Vec::new();

    for item in item_data {
        let mut unique_name = item.unique_name.clone();
        if let Some(enchantment_level) = &item.enchantment_level {
            if enchantment_level > &0 {
                unique_name.push_str(&format!("@{}", enchantment_level));
            }
        }

        item_unique_names.push(unique_name);
        item_data_values.push(serde_json::to_value(item).unwrap());
    }

    let transaction = pool.begin().await?;

    sqlx::query!(
        "
INSERT INTO item_data (
    item_unique_name,
    data)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::JSONB[])
ON CONFLICT (item_unique_name) DO UPDATE
    SET data = EXCLUDED.data",
        &item_unique_names,
        &item_data_values
    )
    .execute(pool)
    .await?;

    transaction.commit().await?;

    Ok(())
}
