use std::env;

#[derive(Clone)]  // Adiciona esta linha
pub struct Config {
    pub database_url: String,
    pub server_addr: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_addr: env::var("SERVER_ADDR").expect("SERVER_ADDR must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}