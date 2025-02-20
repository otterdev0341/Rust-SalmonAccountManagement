
use migration::infrastructure::mysql::migrator::Migrator;
use migration::{config::db_config::DBConfig, infrastructure::mysql::connection::mysql_connect::mysql_connec};

use tracing_subscriber;
use sea_orm_migration::MigratorTrait;
#[macro_use] extern crate rocket;

#[get("/")]
fn health() -> &'static str {
    "server is running!!"
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    
    // ennv loaded from .env file
    dotenv::dotenv().ok();
    
    // init logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // init migrator
    let database_config = DBConfig::default();
    let db = match mysql_connec(&database_config).await {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
            panic!("Failed to connect to the database: {:?}", e);
            
        }
    };
    Migrator::up(&db, None).await.unwrap();
    // Migrator::fresh(&db).await.unwrap();

    // inittial rocket
    rocket::build()
        .mount("/", routes![health])
        .launch()
        .await
        .map_err(|e| {
            eprintln!("Error: {:?}", e);
            e
        })?;

    Ok(())
}
