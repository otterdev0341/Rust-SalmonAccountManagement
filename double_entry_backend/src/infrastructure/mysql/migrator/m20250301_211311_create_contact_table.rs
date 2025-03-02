use sea_orm_migration::{prelude::*, schema::*};

use super::{m20220101_000001_create_user_table::User, m20250301_212131_create_contact_type::ContactType};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contact::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Contact::Name))
                    .col(
                        ColumnDef::new(Contact::CompanyId)
                            .uuid()
                            .not_null()
                    ).col(
                        ColumnDef::new(Contact::UserId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_contact_user_id")
                        .from(Contact::Table, Contact::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Contact::ContactTypeId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contact_contact_type_id")
                            .from(Contact::Table, Contact::ContactTypeId)
                            .to(ContactType::Table, ContactType::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Contact::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Contact::UpdatedAt)
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
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Contact {
    #[sea_orm(iden = "Contact")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "company_id")]
    CompanyId,
    #[sea_orm(iden = "contact_type_id")]
    ContactTypeId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
