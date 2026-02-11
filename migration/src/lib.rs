pub use sea_orm_migration::prelude::*;

mod m20220101_000002_create_user_domain;
mod m20220101_000003_create_post_domain;
mod m20220101_000001_create_tag_domain;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_tag_domain::Migration),
            Box::new(m20220101_000002_create_user_domain::Migration),
            Box::new(m20220101_000003_create_post_domain::Migration),
        ]
    }
}
