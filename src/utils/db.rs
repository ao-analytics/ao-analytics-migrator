use aodata_models::json;
use sqlx::{postgres::PgQueryResult, PgPool};

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
ON CONFLICT DO
NOTHING",
        &location_ids,
        &location_names
    )
    .execute(pool)
    .await?;

    transaction.commit().await.unwrap();

    return Ok(())
}

pub async fn insert_items(
    pool: &PgPool,
    items: &Vec<json::Localization>,
) -> Result<PgQueryResult, sqlx::Error> {

    let transaction = pool.begin().await.unwrap();

    let mut item_ids: Vec<i32> = Vec::new();
    let mut item_unique_names: Vec<String> = Vec::new();

    for item in items {
        let item_id: i32 = match item.id.parse() {
            Ok(id) => id,
            Err(_) => continue,
        };

        if item_ids.contains(&item_id) {
            continue;
        }

        item_ids.push(item_id);
        item_unique_names.push(item.item.to_string());
    }

    let result = sqlx::query!(
        "
INSERT INTO item (
    id,
    unique_name)
SELECT * FROM UNNEST(
    $1::INTEGER[],
    $2::VARCHAR[])
ON CONFLICT DO
    NOTHING",
        &item_ids,
        &item_unique_names
    )
    .execute(pool)
    .await?;

    transaction.commit().await.unwrap();

    return Ok(result);
}

pub async fn insert_localizations(
    pool: &PgPool,
    localizations: Vec<json::Localization>,
) -> Result<(), sqlx::Error> {

    // Probably a better way to do this...
    
    let mut descriptions_item_unique_names: Vec<String> = Vec::new();
    let mut descriptions_en_us: Vec<Option<String>> = Vec::new();
    let mut descriptions_de_de: Vec<Option<String>> = Vec::new();
    let mut descriptions_fr_fr: Vec<Option<String>> = Vec::new();
    let mut descriptions_ru_ru: Vec<Option<String>> = Vec::new();
    let mut descriptions_pl_pl: Vec<Option<String>> = Vec::new();
    let mut descriptions_es_es: Vec<Option<String>> = Vec::new();
    let mut descriptions_pt_br: Vec<Option<String>> = Vec::new();
    let mut descriptions_it_it: Vec<Option<String>> = Vec::new();
    let mut descriptions_zh_cn: Vec<Option<String>> = Vec::new();
    let mut descriptions_ko_kr: Vec<Option<String>> = Vec::new();
    let mut descriptions_ja_jp: Vec<Option<String>> = Vec::new();
    let mut descriptions_zh_tw: Vec<Option<String>> = Vec::new();
    let mut descriptions_id_id: Vec<Option<String>> = Vec::new();
    let mut descriptions_tr_tr: Vec<Option<String>> = Vec::new();
    let mut descriptions_ar_sa: Vec<Option<String>> = Vec::new();
    let mut names_item_unique_names: Vec<String> = Vec::new();
    let mut names_en_us: Vec<Option<String>> = Vec::new();
    let mut names_de_de: Vec<Option<String>> = Vec::new();
    let mut names_fr_fr: Vec<Option<String>> = Vec::new();
    let mut names_ru_ru: Vec<Option<String>> = Vec::new();
    let mut names_pl_pl: Vec<Option<String>> = Vec::new();
    let mut names_es_es: Vec<Option<String>> = Vec::new();
    let mut names_pt_br: Vec<Option<String>> = Vec::new();
    let mut names_it_it: Vec<Option<String>> = Vec::new();
    let mut names_zh_cn: Vec<Option<String>> = Vec::new();
    let mut names_ko_kr: Vec<Option<String>> = Vec::new();
    let mut names_ja_jp: Vec<Option<String>> = Vec::new();
    let mut names_zh_tw: Vec<Option<String>> = Vec::new();
    let mut names_id_id: Vec<Option<String>> = Vec::new();
    let mut names_tr_tr: Vec<Option<String>> = Vec::new();
    let mut names_ar_sa: Vec<Option<String>> = Vec::new();

    for localization in localizations {
        let item_unique_name = &localization.item;

        if let Some(localized_descriptions) = localization.localized_descriptions {
            descriptions_item_unique_names.push(item_unique_name.to_string());
            descriptions_en_us.push(localized_descriptions.en_us);
            descriptions_de_de.push(localized_descriptions.de_de);
            descriptions_fr_fr.push(localized_descriptions.fr_fr);
            descriptions_ru_ru.push(localized_descriptions.ru_ru);
            descriptions_pl_pl.push(localized_descriptions.pl_pl);
            descriptions_es_es.push(localized_descriptions.es_es);
            descriptions_pt_br.push(localized_descriptions.pt_br);
            descriptions_it_it.push(localized_descriptions.it_it);
            descriptions_zh_cn.push(localized_descriptions.zh_cn);
            descriptions_ko_kr.push(localized_descriptions.ko_kr);
            descriptions_ja_jp.push(localized_descriptions.ja_jp);
            descriptions_zh_tw.push(localized_descriptions.zh_tw);
            descriptions_id_id.push(localized_descriptions.id_id);
            descriptions_tr_tr.push(localized_descriptions.tr_tr);
            descriptions_ar_sa.push(localized_descriptions.ar_sa);
        }

        if let Some(localized_names) = localization.localized_names {
            names_item_unique_names.push(item_unique_name.to_string());
            names_en_us.push(localized_names.en_us);
            names_de_de.push(localized_names.de_de);
            names_fr_fr.push(localized_names.fr_fr);
            names_ru_ru.push(localized_names.ru_ru);
            names_pl_pl.push(localized_names.pl_pl);
            names_es_es.push(localized_names.es_es);
            names_pt_br.push(localized_names.pt_br);
            names_it_it.push(localized_names.it_it);
            names_zh_cn.push(localized_names.zh_cn);
            names_ko_kr.push(localized_names.ko_kr);
            names_ja_jp.push(localized_names.ja_jp);
            names_zh_tw.push(localized_names.zh_tw);
            names_id_id.push(localized_names.id_id);
            names_tr_tr.push(localized_names.tr_tr);
            names_ar_sa.push(localized_names.ar_sa);
        }
    }

    let description_sql = r"
INSERT INTO localized_description (
    item_unique_name,
    en_us,
    de_de,
    fr_fr,
    ru_ru,
    pl_pl,
    es_es,
    pt_br,
    it_it,
    zh_cn,
    ko_kr,
    ja_jp,
    zh_tw,
    id_id,
    tr_tr,
    ar_sa)
SELECT * FROM UNNEST(
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11,
    $12,
    $13,
    $14,
    $15,
    $16)
ON CONFLICT (item_unique_name) DO
    UPDATE SET
        en_us = EXCLUDED.en_us,
        de_de = EXCLUDED.de_de,
        fr_fr = EXCLUDED.fr_fr,
        ru_ru = EXCLUDED.ru_ru,
        pl_pl = EXCLUDED.pl_pl,
        es_es = EXCLUDED.es_es,
        pt_br = EXCLUDED.pt_br,
        it_it = EXCLUDED.it_it,
        zh_cn = EXCLUDED.zh_cn,
        ko_kr = EXCLUDED.ko_kr,
        ja_jp = EXCLUDED.ja_jp,
        zh_tw = EXCLUDED.zh_tw,
        id_id = EXCLUDED.id_id,
        tr_tr = EXCLUDED.tr_tr,
        ar_sa = EXCLUDED.ar_sa";

    let names_sql = r"
INSERT INTO localized_name (
    item_unique_name,
    en_us,
    de_de,
    fr_fr,
    ru_ru,
    pl_pl,
    es_es,
    pt_br,
    it_it,
    zh_cn,
    ko_kr,
    ja_jp,
    zh_tw,
    id_id,
    tr_tr,
    ar_sa)
SELECT * FROM UNNEST(
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11,
    $12,
    $13,
    $14,
    $15,
    $16)
ON CONFLICT (item_unique_name) DO
    UPDATE SET
        en_us = EXCLUDED.en_us::VARCHAR,
        de_de = EXCLUDED.de_de::VARCHAR,
        fr_fr = EXCLUDED.fr_fr::VARCHAR,
        ru_ru = EXCLUDED.ru_ru::VARCHAR,
        pl_pl = EXCLUDED.pl_pl::VARCHAR,
        es_es = EXCLUDED.es_es::VARCHAR,
        pt_br = EXCLUDED.pt_br::VARCHAR,
        it_it = EXCLUDED.it_it::VARCHAR,
        zh_cn = EXCLUDED.zh_cn::VARCHAR,
        ko_kr = EXCLUDED.ko_kr::VARCHAR,
        ja_jp = EXCLUDED.ja_jp::VARCHAR,
        zh_tw = EXCLUDED.zh_tw::VARCHAR,
        id_id = EXCLUDED.id_id::VARCHAR,
        tr_tr = EXCLUDED.tr_tr::VARCHAR,
        ar_sa = EXCLUDED.ar_sa::VARCHAR";

    let transaction = pool.begin().await?;

    sqlx::query(description_sql)
        .bind(descriptions_item_unique_names)
        .bind(descriptions_en_us)
        .bind(descriptions_de_de)
        .bind(descriptions_fr_fr)
        .bind(descriptions_ru_ru)
        .bind(descriptions_pl_pl)
        .bind(descriptions_es_es)
        .bind(descriptions_pt_br)
        .bind(descriptions_it_it)
        .bind(descriptions_zh_cn)
        .bind(descriptions_ko_kr)
        .bind(descriptions_ja_jp)
        .bind(descriptions_zh_tw)
        .bind(descriptions_id_id)
        .bind(descriptions_tr_tr)
        .bind(descriptions_ar_sa)
        .fetch_all(pool)
        .await?;

    sqlx::query(names_sql)
        .bind(names_item_unique_names)
        .bind(names_en_us)
        .bind(names_de_de)
        .bind(names_fr_fr)
        .bind(names_ru_ru)
        .bind(names_pl_pl)
        .bind(names_es_es)
        .bind(names_pt_br)
        .bind(names_it_it)
        .bind(names_zh_cn)
        .bind(names_ko_kr)
        .bind(names_ja_jp)
        .bind(names_zh_tw)
        .bind(names_id_id)
        .bind(names_tr_tr)
        .bind(names_ar_sa)
        .fetch_all(pool)
        .await?;

    transaction.commit().await?;

    return Ok(())
}

/* rust magic
fn wrap<'a>(el: Option<&'a String>) -> Option<&'a str> {
    Some(el?.as_str())
}
*/