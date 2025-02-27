use std::error::Error;

use reqwest::Client;
use sqlx::{query, SqlitePool};

use crate::models::ndc::{ApiResponse, Product};

pub async fn fetch_and_save(
    pool: &SqlitePool,
    endpoint: &str,
    year_range: (i32, i32),
    limit: i32,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    for year in year_range.0..year_range.1 + 1 {
        let start_date = format!("{year}0101");
        let end_date = format!("{year}1231");
        println!(
            "Saving NDC data with marketing_start_date between {} and {}.",
            start_date, end_date
        );
        let mut skip: i32 = 0;
        loop {
            let ep = format!(
                "{endpoint}?search=marketing_start_date:[{start_date}+TO+{end_date}]&limit={limit}&skip={skip}"
            );

            let response = client.get(ep).send().await?;

            let api_response: ApiResponse = response.json().await?;

            save_to_db(pool, &api_response.results).await?;

            skip += limit;
            if skip >= api_response.meta.results.total {
                break;
            }
        }
    }
    Ok(())
}

async fn save_to_db(pool: &SqlitePool, data: &[Product]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    for product in data {
        query!(
            "INSERT OR IGNORE INTO product (
                product_ndc,
                generic_name,
                labeler_name,
                brand_name,
                finished,
                listing_expiration_date,
                marketing_category,
                dosage_form,
                product_type,
                marketing_start_date,
                application_number,
                brand_name_base
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
            product.product_ndc,
            product.generic_name,
            product.labeler_name,
            product.brand_name,
            product.finished,
            product.listing_expiration_date,
            product.marketing_category,
            product.dosage_form,
            product.product_type,
            product.marketing_start_date,
            product.application_number,
            product.brand_name_base,
        )
        .execute(&mut *tx)
        .await?;

        for package in &product.packaging {
            query!(
                "INSERT OR IGNORE INTO package (
                    package_ndc,
                    product_ndc,
                    description,
                    marketing_start_date,
                    sample
                )
                VALUES (?, ?, ?, ?, ?);",
                package.package_ndc,
                product.product_ndc,
                package.description,
                package.marketing_start_date,
                package.sample,
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(active_ingredients) = &product.active_ingredients {
            for active_ingredient in active_ingredients {
                query!(
                    "INSERT OR IGNORE INTO active_ingredient (
                    product_ndc,
                    name,
                    strength
                )
                VALUES (?, ?, ?);",
                    product.product_ndc,
                    active_ingredient.name,
                    active_ingredient.strength,
                )
                .execute(&mut *tx)
                .await?;
            }
        }
    }

    tx.commit().await?;

    Ok(())
}
