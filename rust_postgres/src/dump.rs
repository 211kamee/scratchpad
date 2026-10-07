use std::{env, error::Error};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let postgres_uri: String = env::var("POSTGRES_URI").expect("ERR: POSTGRES_URI env not set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_uri)
        .await?;

    let res: Vec<sqlx::postgres::PgRow> = sqlx::query("SELECT 1 + 1 as sum, 2+2 as sun2")
        .fetch_all(&pool)
        .await?;

    print!("{:?}", res);

    return Ok(());
}
// SELECT datname FROM pg_database;
