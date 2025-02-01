use std::error::Error;

use reqwest::Client;
use sqlx::{query, SqlitePool};

use crate::models::ndc::{ApiResponse, NDCProduct};

const ENDPOINT: &str = "https://api.fda.gov/drug/ndc.json?limit=1000";

pub async fn fetch_and_save(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let response = client.get(ENDPOINT).send().await?;

    let api_response: ApiResponse = response.json().await?;

    save_to_db(pool, &api_response.results).await?;

    Ok(())
}

async fn save_to_db(pool: &SqlitePool, data: &[NDCProduct]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    for product in data {
        query!(
            "INSERT OR IGNORE INTO ndc_product (
                product_ndc,
                generic_name,
                labeler_name,
                brand_name,
                finished,
                listing_expiration_date,
                marketing_category,
                dosage_form,
                spl_id,
                product_type,
                marketing_start_date,
                product_id,
                application_number,
                brand_name_base
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
            product.product_ndc,
            product.generic_name,
            product.labeler_name,
            product.brand_name,
            product.finished,
            product.listing_expiration_date,
            product.marketing_category,
            product.dosage_form,
            product.spl_id,
            product.product_type,
            product.marketing_start_date,
            product.product_id,
            product.application_number,
            product.brand_name_base,
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
