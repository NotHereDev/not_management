use diesel::r2d2::ConnectionManager;
use once_cell::sync::Lazy;
use r2d2::{Pool, PooledConnection};

#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
    Postgresql(diesel::PgConnection),
    Mysql(diesel::MysqlConnection),
    Sqlite(diesel::SqliteConnection),
}

pub type DbPool = Pool<ConnectionManager<AnyConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<AnyConnection>>;

pub static DB: Lazy<DbPool> = Lazy::new(|| {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<AnyConnection>::new(url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
});

pub fn get_connection() -> Result<DbPooledConnection, String> {
    DB.get().map_err(|e| format!("Failed to get database connection: {}", e))
}