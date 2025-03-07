
#[derive(Debug)]
pub struct DBConfig {
    pub db_host: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_database_name: String
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("DB_PASSWORD").unwrap_or("password".to_string()),
            db_database_name: std::env::var("DB_DATABASE_NAME").unwrap_or("note_tag".to_string())
        }
    }
}