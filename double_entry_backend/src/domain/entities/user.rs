//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "User")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Binary(16)")]
    pub id: Vec<u8>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::company::Entity")]
    Company,
    #[sea_orm(has_many = "super::contact::Entity")]
    Contact,
    #[sea_orm(has_many = "super::contact_detail::Entity")]
    ContactDetail,
    #[sea_orm(has_many = "super::contact_type::Entity")]
    ContactType,
    #[sea_orm(has_many = "super::info::Entity")]
    Info,
    #[sea_orm(has_many = "super::project::Entity")]
    Project,
    #[sea_orm(has_many = "super::project_status::Entity")]
    ProjectStatus,
}

impl Related<super::company::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Company.def()
    }
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::contact_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContactDetail.def()
    }
}

impl Related<super::contact_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContactType.def()
    }
}

impl Related<super::info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Info.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::project_status::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectStatus.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
