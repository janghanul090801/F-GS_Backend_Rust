use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // =========================
        // POSTS
        // =========================
        manager.create_table(
            Table::create()
                .table(Post::Table)
                .if_not_exists()
                .col(ColumnDef::new(Post::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Post::Title).string().not_null())
                .col(ColumnDef::new(Post::Content).text().not_null())
                .col(ColumnDef::new(Post::Category).string_len(100).not_null())
                .col(ColumnDef::new(Post::MaxRecruits).integer().not_null())
                .col(ColumnDef::new(Post::CurrentRecruits).integer().not_null())
                .col(ColumnDef::new(Post::IsFinished).boolean().default(false).not_null())
                .col(ColumnDef::new(Post::CreatedAt).timestamp().not_null())
                .col(ColumnDef::new(Post::UpdatedAt).timestamp().not_null())
                .col(ColumnDef::new(Post::Deadline).timestamp().not_null())
                .col(ColumnDef::new(Post::AuthorId).integer().not_null())
                .col(ColumnDef::new(Post::Attachments).array(ColumnType::Text).not_null())
                .col(ColumnDef::new(Post::Apply).integer().default(0).not_null())
                .col(ColumnDef::new(Post::IsNew).boolean().default(true).not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(Post::Table, Post::AuthorId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        // =========================
        // FORMS
        // =========================
        manager.create_table(
            Table::create()
                .table(Form::Table)
                .if_not_exists()
                .col(ColumnDef::new(Form::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Form::PostId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(Form::Table, Form::PostId)
                        .to(Post::Table, Post::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        // =========================
        // QUESTIONS
        // =========================
        manager.create_table(
            Table::create()
                .table(Question::Table)
                .if_not_exists()
                .col(ColumnDef::new(Question::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Question::QuestionType).string().not_null())
                .col(ColumnDef::new(Question::Label).string().not_null())
                .col(ColumnDef::new(Question::Required).boolean().not_null())
                .col(ColumnDef::new(Question::IsFile).boolean().not_null())
                .col(ColumnDef::new(Question::FormId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(Question::Table, Question::FormId)
                        .to(Form::Table, Form::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        // =========================
        // FORM ANSWERS
        // =========================
        manager.create_table(
            Table::create()
                .table(FormAnswer::Table)
                .if_not_exists()
                .col(ColumnDef::new(FormAnswer::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(FormAnswer::FormId).integer().not_null())
                .col(ColumnDef::new(FormAnswer::AnswererId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(FormAnswer::Table, FormAnswer::FormId)
                        .to(Form::Table, Form::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        // =========================
        // QUESTION ANSWERS
        // =========================
        manager.create_table(
            Table::create()
                .table(QuestionAnswer::Table)
                .if_not_exists()
                .col(ColumnDef::new(QuestionAnswer::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(QuestionAnswer::FormAnswerId).integer().not_null())
                .col(ColumnDef::new(QuestionAnswer::QuestionId).integer().not_null())
                .col(ColumnDef::new(QuestionAnswer::Answer).text().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(QuestionAnswer::Table, QuestionAnswer::FormAnswerId)
                        .to(FormAnswer::Table, FormAnswer::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        // =========================
        // M2M TABLES
        // =========================

        manager.create_table(
            Table::create()
                .table(PostTag::Table)
                .if_not_exists()
                .col(ColumnDef::new(PostTag::PostId).integer().not_null())
                .col(ColumnDef::new(PostTag::TagId).integer().not_null())
                .primary_key(
                    Index::create()
                        .col(PostTag::PostId)
                        .col(PostTag::TagId)
                )
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(UserRecruit::Table)
                .if_not_exists()
                .col(ColumnDef::new(UserRecruit::PostId).integer().not_null())
                .col(ColumnDef::new(UserRecruit::UserId).integer().not_null())
                .primary_key(
                    Index::create()
                        .col(UserRecruit::PostId)
                        .col(UserRecruit::UserId)
                )
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(PostVolunteer::Table)
                .if_not_exists()
                .col(ColumnDef::new(PostVolunteer::PostId).integer().not_null())
                .col(ColumnDef::new(PostVolunteer::UserId).integer().not_null())
                .primary_key(
                    Index::create()
                        .col(PostVolunteer::PostId)
                        .col(PostVolunteer::UserId)
                )
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(PostVolunteer::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(UserRecruit::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(PostTag::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(QuestionAnswer::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(FormAnswer::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Question::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Form::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Post::Table).to_owned()).await?;
        Ok(())
    }
}



#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Content,
    Category,
    MaxRecruits,
    CurrentRecruits,
    IsFinished,
    CreatedAt,
    UpdatedAt,
    Deadline,
    AuthorId,
    Attachments,
    Apply,
    IsNew,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
    PostId,
}

#[derive(DeriveIden)]
enum Question {
    Table,
    Id,
    QuestionType,
    Label,
    Required,
    IsFile,
    FormId,
}

#[derive(DeriveIden)]
enum FormAnswer {
    Table,
    Id,
    FormId,
    AnswererId,
}

#[derive(DeriveIden)]
enum QuestionAnswer {
    Table,
    Id,
    FormAnswerId,
    QuestionId,
    Answer,
}

#[derive(DeriveIden)]
enum PostTag {
    Table,
    PostId,
    TagId,
}
#[derive(DeriveIden)]
enum UserRecruit {
    Table,
    PostId,
    UserId,
}

#[derive(DeriveIden)]
enum PostVolunteer {
    Table,
    PostId,
    UserId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

