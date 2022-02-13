use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::env::get_db_url;

pub async fn init() -> Result<Pool<Postgres>, anyhow::Error> {
    let db_url = get_db_url()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    println!("Conntected to the database at: {db_url}");

    // Documentation: https://docs.rs/sqlx/0.5.10/sqlx/macro.migrate.html
    // Path is relative to the root of the crate at Cargo.toml
    // The path needs to be static because the mgirations will be checked at compile-time
    sqlx::migrate!("./sql/migrations").run(&pool).await?;

    Ok(pool)
}
