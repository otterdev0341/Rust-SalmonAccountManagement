
use migration::config::a_init_openapi_config::init_openapi;
use migration::infrastructure::faring::cors::CORS;
use migration::infrastructure::handler::controller::a_init_routes::init_controller_setup;
use migration::infrastructure::mysql::migrator::Migrator;
use migration::{config::db_config::DBConfig, infrastructure::mysql::connection::mysql_connect::mysql_connec};
use tracing_subscriber;
use sea_orm_migration::MigratorTrait;
use utoipa_swagger_ui::SwaggerUi;
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
    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![health])
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
