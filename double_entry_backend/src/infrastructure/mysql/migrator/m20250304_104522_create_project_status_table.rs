use sea_orm_migration::{prelude::*, schema::*};

use super::m20220101_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(ProjectStatus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectStatus::Id)
                            .uuid()
                            .not_null()
                            .default("Uuid()")
                            .primary_key(),
                    )
                    .col(string(ProjectStatus::Name).not_null().unique_key())
                    .col(
                        ColumnDef::new(ProjectStatus::UserId)
                            .uuid()
                            .not_null()                          
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_status_user_id")
                            .from(ProjectStatus::Table, ProjectStatus::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(ProjectStatus::Description).not_null())
                    .col(
                        ColumnDef::new(ProjectStatus::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(ProjectStatus::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(ProjectStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ProjectStatus {
    #[sea_orm(iden = "ProjectStatus")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
