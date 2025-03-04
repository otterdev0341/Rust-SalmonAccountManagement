use sea_orm_migration::{prelude::*, schema::*};

use super::{m20220101_000001_create_user_table::User, m20250224_200022_create_company_table::Company, m20250304_104522_create_project_status_table::ProjectStatus};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Project::Name).not_null())
                    .col(string(Project::Description))
                    .col(
                        ColumnDef::new(Project::CompanyId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_company_id")
                            .from(Project::Table, Project::CompanyId)
                            .to(Company::Table, Company::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Project::UserId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_user_id")
                            .from(Project::Table, Project::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Project::ProjectStatusId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_project_status_id")
                            .from(Project::Table, Project::ProjectStatusId)
                            .to(ProjectStatus::Table, ProjectStatus::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Project::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Project::UpdatedAt)
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
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Project {
    #[sea_orm(iden = "Project")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "company_id")]
    CompanyId,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "project_status_id")]
    ProjectStatusId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
