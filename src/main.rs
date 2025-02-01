mod api;
mod database;
mod models;

use api::ndc::fetch_and_save;
use database::get_db_connection;
use dotenv::dotenv;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    let pool = get_db_connection(&db_url)
        .await
        .expect("Database could not be initialised.");

    fetch_and_save(&pool)
        .await
        .expect("An error occurred while fetching NDC data.");
}
