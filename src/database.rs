use sqlx::{Error, SqlitePool};

pub async fn get_db_connection(db_url: &str) -> Result<SqlitePool, Error> {
    SqlitePool::connect(db_url).await
}
