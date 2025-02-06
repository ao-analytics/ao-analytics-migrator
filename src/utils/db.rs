use crate::models::json;
use sqlx::PgPool;
use tracing::warn;

pub async fn insert_locations(
    pool: &PgPool,
    locations: &Vec<json::Location>,
) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await?;

    let mut ids: Vec<String> = Vec::new();
    let mut location_ids: Vec<Option<i16>> = Vec::new();
    let mut location_names: Vec<String> = Vec::new();

    for location in locations {
        let location_id = match location.id.parse::<i16>() {
            Ok(location_id) => Some(location_id),
            Err(_) => {
                if location.id.starts_with("BLACKBANK-") {
                    match location.id.split('-').rev().next().unwrap().parse::<i16>() {
                        Ok(location_id) => Some(location_id),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        };
        ids.push(location.id.clone());
        location_ids.push(location_id);

        location_names.push(location.name.to_string());
    }

    sqlx::query!(
        "
INSERT INTO location (
    id)
SELECT DISTINCT ON (id) id FROM UNNEST(
    $1::SMALLINT[])
AS t(id)
ON CONFLICT DO NOTHING",
        &location_ids.iter().filter_map(|f| *f).collect::<Vec<i16>>(),
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "
INSERT INTO location_data (
    id,
    location_id,
    name)
SELECT DISTINCT ON (id)
    id,
    location_id,
    name
FROM UNNEST(
    $1::TEXT[],
    $2::SMALLINT[],
    $3::TEXT[])
AS t(id, location_id, name)
ON CONFLICT (id) DO UPDATE
    SET location_id = EXCLUDED.location_id,
        name = EXCLUDED.name",
        &ids,
        &location_ids as _,
        &location_names
    )
    .execute(pool)
    .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn insert_localizations(
    pool: &PgPool,
    localizations: &Vec<json::Localization>,
) -> Result<(), sqlx::Error> {
    let mut item_unique_names: Vec<String> = Vec::new();
    let mut item_group_names: Vec<String> = Vec::new();
    let mut enchantment_levels: Vec<i16> = Vec::new();

    let mut descriptions_item_unique_names: Vec<String> = Vec::new();
    let mut descriptions_langs: Vec<String> = Vec::new();
    let mut descriptions_values: Vec<String> = Vec::new();

    let mut names_item_unique_names: Vec<String> = Vec::new();
    let mut names_langs: Vec<String> = Vec::new();
    let mut names_values: Vec<String> = Vec::new();

    for localization in localizations {
        let item_unique_name = &localization.unique_name;

        item_group_names.push(item_unique_name.split('@').next().unwrap().to_string());
        enchantment_levels.push(
            item_unique_name
                .split('@')
                .rev()
                .next()
                .map_or(0, |str| str.parse().unwrap_or(0)),
        );

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
INSERT INTO item_group (name)
SELECT DISTINCT ON (name)
    name
FROM UNNEST(
    $1::VARCHAR[])
AS t(name)
ON CONFLICT DO NOTHING",
        &item_group_names,
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "
INSERT INTO item (
    unique_name,
    enchantment_level,
    item_group_name)
SELECT DISTINCT ON (unique_name)
    unique_name,
    enchantment_level,
    item_group_name
FROM UNNEST(
    $1::VARCHAR[],
    $2::SMALLINT[],
    $3::VARCHAR[])
AS t(unique_name, enchantment_level, item_group_name)
ON CONFLICT DO NOTHING",
        &item_unique_names,
        &enchantment_levels,
        &item_group_names,
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
    item_data: Vec<json::Item>,
) -> Result<(), sqlx::Error> {
    let mut item_unique_names = Vec::new();
    let mut item_data_values = Vec::new();

    for item in item_data {
        let item_group_name = item
            .as_object()
            .and_then(|o| o.get("@uniquename"))
            .and_then(|u| u.as_str().map(|u| u.to_string()));

        let unique_name = match item_group_name {
            Some(unique_name) => unique_name,
            None => {
                warn!("Failed to grab @uniquename from {}", item);
                continue;
            }
        };

        item_unique_names.push(unique_name);
        item_data_values.push(item);
    }

    let transaction = pool.begin().await?;

    sqlx::query!(
        "
INSERT INTO item_data (
    item_group_name,
    data)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::JSONB[])
ON CONFLICT (item_group_name) DO UPDATE
    SET data = EXCLUDED.data",
        &item_unique_names,
        &item_data_values
    )
    .execute(pool)
    .await?;

    transaction.commit().await?;

    Ok(())
}
