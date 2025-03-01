use sea_orm_migration::{prelude::*, schema::*};

use super::m20250301_211311_create_contact_table::Contact;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(ContactDetail::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactDetail::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ContactDetail::ContactId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contact_detail_contact_id")
                            .from(ContactDetail::Table, ContactDetail::ContactId)
                            .to(Contact::Table, Contact::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(ContactDetail::MobilePhone1))
                    .col(string(ContactDetail::MobilePhone2))
                    .col(string(ContactDetail::MobilePhone3))
                    .col(string(ContactDetail::Email))
                    .col(string(ContactDetail::Address))
                    .col(
                        ColumnDef::new(ContactDetail::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(ContactDetail::UpdatedAt)
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
            .drop_table(Table::drop().table(ContactDetail::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ContactDetail {
    #[sea_orm(iden = "ContactDetail")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "contact_id")]
    ContactId,
    #[sea_orm(iden = "mobile_phone_1")]
    MobilePhone1,
    #[sea_orm(iden = "mobile_phone_2")]
    MobilePhone2,
    #[sea_orm(iden = "mobile_phone_3")]
    MobilePhone3,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "address")]
    Address,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
