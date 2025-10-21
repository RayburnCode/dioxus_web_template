// database/src/lib.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
pub mod entities;
 
pub use migration::Migrator;
 
pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    // Use ? to propagate errors directly - no map_err needed
    let conn = Database::connect(database_url).await?;
    Migrator::up(&conn, None).await?;
    Ok(conn)
}

/// Get the database URL from the environment, or exit with error if not set.
pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("Error: DATABASE_URL environment variable not set ({err}). Please set it in your .env file or environment.");
        std::process::exit(1);
    })
}