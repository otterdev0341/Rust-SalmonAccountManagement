#[derive(Debug)]
pub struct JwtConfig{
    pub secret_key: String,
    pub expiration: i64
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret_key: std::env::var("JWT_SECRET").unwrap_or("".to_string()),
            expiration: std::env::var("JWT_EXPIRATION").unwrap_or("".to_string()).parse::<i64>().unwrap_or(0)
        }
    }
}