pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20250224_200022_create_company_table;
mod m20250301_211311_create_contact_table;
mod m20250301_212123_create_contact_detail;
mod m20250301_212131_create_contact_type;
mod m20250304_104522_create_project_status_table;
mod m20250304_123019_create_info_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20250224_200022_create_company_table::Migration),
            Box::new(m20250301_211311_create_contact_table::Migration),
            Box::new(m20250301_212123_create_contact_detail::Migration),
            Box::new(m20250301_212131_create_contact_type::Migration),
            Box::new(m20250304_104522_create_project_status_table::Migration),
            Box::new(m20250304_123019_create_info_table::Migration),
        ]
    }
}
