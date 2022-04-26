use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::env::get_db_url;

pub async fn init() -> Result<Pool<Postgres>, anyhow::Error> {
    let db_url = get_db_url().with_context(|| "Failed to create DB URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    println!("Conntected to the database at: {db_url}");

    // Run migrations when the backend server initializes
    //
    // Documentation: https://docs.rs/sqlx/0.5.10/sqlx/macro.migrate.html
    //
    // Path is relative to the root of the crate at Cargo.toml
    // The path needs to be static because the mgirations will be checked at compile-time
    sqlx::migrate!("./sql/migrations").run(&pool).await?;

    Ok(pool)
}

/// get_db is given an async_graphql Context and extracts a sqlx Pool<Postgres> connection from it
pub fn get_db<'a>(
    ctx: &async_graphql::Context<'a>,
) -> Result<&'a Pool<Postgres>, async_graphql::Error> {
    ctx.data::<Pool<Postgres>>()
}
