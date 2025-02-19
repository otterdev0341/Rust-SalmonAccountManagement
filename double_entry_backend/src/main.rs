#[macro_use] extern crate rocket;

#[get("/")]
fn health() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    
    // ennv loaded from .env file
    dotenv::dotenv().ok();
    println!("Hello, world!");

    Ok(())
}
