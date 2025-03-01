pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20250224_200022_create_company_table;
mod m20250301_211311_create_contact_table;
mod m20250301_212123_create_contact_detail;
mod m20250301_212131_create_contact_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // indy table : the table that independent from other table
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20250224_200022_create_company_table::Migration),
            Box::new(m20250301_212131_create_contact_type::Migration),
            
            // relation table : the table that depend on other table
            Box::new(m20250301_211311_create_contact_table::Migration),

            // like relation table, but this table is depend on relation table
            Box::new(m20250301_212123_create_contact_detail::Migration),
            
        ]
    }
}
