use sqlx::SqlitePool;

const DATABASE_URL_ENV: &str = "DATABASE_URL";

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}

impl AppState {
    pub fn from_env() -> Self {
        let database_url =
            std::env::var(DATABASE_URL_ENV).expect("DATABASE_URL must be set in .env file");

        let pool = SqlitePool::connect_lazy(&database_url)
            .expect("Failed to create database connection pool");

        AppState { pool }
    }
}
