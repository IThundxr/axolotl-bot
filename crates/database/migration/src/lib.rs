pub use sea_orm_migration::prelude::*;

mod m20260727_204818_create_users_table;
mod m20261008_120000_create_allowed_guilds_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260727_204818_create_users_table::Migration),
            Box::new(m20261008_120000_create_allowed_guilds_table::Migration),
        ]
    }
}
