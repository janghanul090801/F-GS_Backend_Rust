use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // =========================
        // USERS
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Name).not_null())
                    .col(integer(User::StudentId).not_null())
                    .col(text(User::SelfIntroduction).not_null())
                    .col(string_uniq(User::Email).not_null())
                    .col(string(User::Password).not_null())
                    .col(timestamp(User::CreatedAt).not_null())
                    .col(string(User::PortfolioPath).not_null())
                    .col(string(User::ProfilePath).not_null())
                    .col(double(User::Rating).not_null())
                    .col(boolean(User::IsActive).default(false))
                    .col(boolean(User::IsAdmin).default(false))
                    .col(boolean(User::IsWillDeleted).default(false))
                    .to_owned(),
            )
            .await?;

        // =========================
        // REVIEWS
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(Review::Table)
                    .if_not_exists()
                    .col(pk_auto(Review::Id))
                    .col(float(Review::Score).not_null())
                    .col(text(Review::Content).not_null())
                    .col(integer(Review::UserId).not_null())
                    .col(integer(Review::TargetUserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Review::Table, Review::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Review::Table, Review::TargetUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // =========================
        // USER_TAGS (M2M)
        // =========================
        manager
            .create_table(
                Table::create()
                    .table(UserTag::Table)
                    .if_not_exists()
                    .col(integer(UserTag::UserId).not_null())
                    .col(integer(UserTag::TagId).not_null())
                    .primary_key(
                        Index::create()
                            .col(UserTag::UserId)
                            .col(UserTag::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserTag::Table, UserTag::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserTag::Table, UserTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    StudentId,
    SelfIntroduction,
    Email,
    Password,
    CreatedAt,
    PortfolioPath,
    ProfilePath,
    Rating,
    IsActive,
    IsAdmin,
    IsWillDeleted,
}

#[derive(DeriveIden)]
enum Review {
    Table,
    Id,
    Score,
    Content,
    UserId,
    TargetUserId,
}

#[derive(DeriveIden)]
enum UserTag {
    Table,
    UserId,
    TagId,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
}
