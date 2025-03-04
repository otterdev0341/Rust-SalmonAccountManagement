use sea_orm_migration::prelude::*;

use super::{m20250304_123019_create_info_table::Info, m20250304_132642_create_project_table::Project};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(ProjectInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectInfo::ProjectId)
                            .uuid()
                            .not_null()
                            
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_info_project_id")
                            .from(ProjectInfo::Table, ProjectInfo::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(ProjectInfo::InfoId)
                            .uuid()
                            .not_null()
                            
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_info_info_id")
                            .from(ProjectInfo::Table, ProjectInfo::InfoId)
                            .to(Info::Table, Info::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(ProjectInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ProjectInfo {
    #[sea_orm(iden = "Project_X_Info")]
    Table,
    #[sea_orm(iden = "project_id")]
    ProjectId,
    #[sea_orm(iden = "info_id")]
    InfoId
    
}
