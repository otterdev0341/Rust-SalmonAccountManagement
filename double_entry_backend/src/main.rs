
use std::sync::Arc;

use migration::application::usecase::a_init_usecases::init_usecase_setup;
use migration::config::a_init_openapi_config::init_openapi;
use migration::infrastructure::faring::cors::CORS;
use migration::infrastructure::handler::controller::a_init_routes::init_controller_setup;
use migration::infrastructure::mysql::migrator::Migrator;
use migration::{config::db_config::DBConfig, infrastructure::mysql::connection::mysql_connect::mysql_connec};
use tracing_subscriber;
use sea_orm_migration::MigratorTrait;
use utoipa_swagger_ui::SwaggerUi;
extern crate rocket;





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
    let migrate_status = Migrator::fresh(&db).await;
    if migrate_status.is_err() {
        eprintln!("Failed to migrate: {:?}", migrate_status);
        panic!("Failed to migrate: {:?}", migrate_status);
    }
    // Migrator::fresh(&db).await;
    // Migrator::up(&db, None).await;
    let db_arc = Arc::new(db);


    // inittial rocket
    let _rocket = rocket::build()
        .attach(CORS)
        .attach(init_usecase_setup(Arc::clone(&db_arc)))
        .attach(init_controller_setup())
        .mount("/", 
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", 
                init_openapi(),  
            )
        )
        .launch()
        .await
        .map_err(|e| {
            eprintln!("Error: {:?}", e);
            e
        })?;

    Ok(())
}
