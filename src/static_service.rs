use crate::config::APP_CONFIG;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

pub static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_database_connection() -> &'static DatabaseConnection {
    DATABASE_CONNECTION
        .get_or_init(|| async {
            let database_url = &APP_CONFIG.database_url;
            Database::connect(database_url)
                .await
                .expect("Database connection failed")
        })
        .await
}
