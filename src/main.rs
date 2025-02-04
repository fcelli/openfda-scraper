mod api;
mod database;
mod models;

use api::ndc::fetch_and_save;
use database::get_db_connection;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let endpoint = env::var("NDC_ENDPOINT").expect("NDC_ENDPOINT is not set.");
    let year_start = env::var("YEAR_START")
        .expect("YEAR_START is not set.")
        .parse::<i32>()
        .expect("YEAR_START must be an integer.");
    let year_end = env::var("YEAR_END")
        .expect("YEAR_END is not set.")
        .parse::<i32>()
        .expect("YEAR_END must be an integer.");
    let limit = env::var("LIMIT")
        .expect("LIMIT is not set.")
        .parse::<i32>()
        .expect("LIMIT must be an integer.");

    let year_range = (year_start, year_end);

    let pool = get_db_connection(&db_url)
        .await
        .expect("An error occurred while fetching a connection to the database.");

    fetch_and_save(&pool, &endpoint, year_range, limit)
        .await
        .expect("An error occurred while fetching NDC data.");
}
