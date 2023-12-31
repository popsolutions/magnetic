#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20231225_210048_products;
mod m20231225_222642_sales;
mod m20240104_060700_resources;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20231225_210048_products::Migration),
            Box::new(m20231225_222642_sales::Migration),
            Box::new(m20240104_060700_resources::Migration),
        ]
    }
}