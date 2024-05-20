pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn to_string(&self) -> String {
        format!(
            "DatabaseConfig {{ host: {}, port: {}, username: {}, password: {}, database: {} }}",
            self.host, self.port, self.username, self.password, self.database
        )
    }
}