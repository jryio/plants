use async_graphql::SimpleObject;
use sqlx::types::Uuid;

// Defining Enums using Async Grphql
// https://async-graphql.github.io/async-graphql/en/define_enum.html
// Note: Async Graphql will convert enum names to CONSTANT_CASE, we can manually rename to
// PascalCase by using #[graphql(name = "PascalCase")]
//
// Defining Enums using sqlx::postgres::types
// https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html#enumerations
// Note we don't want to crease a Postgres ENUM type because they are fixed strings which cannot be modified.
// Instead we create a table and join by id to simulate an enum. By deriving `sqlx::Type` we can tell sqlx that
// The string value of the
// #[derive(async_graphql::Enum, sqlx::Type, Copy, Clone, Eq, PartialEq)]
// #[sqlx(rename_all = "PascalCase")] // https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#rename_all
// pub enum Person {
//     #[graphql(name = "Jacob")]
//     Jacob,
//     #[graphql(name = "Magda")]
//     Magda,
//     #[graphql(name = "None")]
//     None,
// }

#[derive(SimpleObject, sqlx::Type)]
pub struct Person {
    id: Uuid,
    name: String,
}
