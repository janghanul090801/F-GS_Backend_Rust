use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // =========================
        // TAGS
        // =========================
        manager.create_table(
            Table::create()
                .table(Tag::Table)
                .if_not_exists()
                .col(ColumnDef::new(Tag::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Tag::Name).string().unique_key().not_null())
                .col(ColumnDef::new(Tag::BgColor).string_len(8).not_null())
                .col(ColumnDef::new(Tag::FontColor).string_len(8).not_null())
                .col(ColumnDef::new(Tag::Usage).integer().default(0).not_null())
                .col(ColumnDef::new(Tag::TagType).string_len(1).not_null())
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;


        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    BgColor,
    FontColor,
    Usage,
    TagType,
}