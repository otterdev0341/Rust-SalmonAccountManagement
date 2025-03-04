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
                    .table(Info::Table)
                        .if_not_exists()
                    .col(
                        ColumnDef::new(Info::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            
                    )
                    .col(string(Info::Title))
                    .col(string(Info::Content))
                    .col(
                        ColumnDef::new(Info::UserId)
                            .uuid()
                            .not_null()
                            
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_info_user_id")
                            .from(Info::Table, Info::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Info::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Info::UpdatedAt)
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
            .drop_table(Table::drop().table(Info::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Info {
    #[sea_orm(iden = "Info")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "title")]
    Title,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "content")]
    Content,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
