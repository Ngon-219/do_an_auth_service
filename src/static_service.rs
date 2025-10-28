use anyhow::anyhow;
use crate::config::APP_CONFIG;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;
use crate::blockchain::BlockchainService;

pub static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub static BLOCKCHAIN_SERVICES: OnceCell<BlockchainService> = OnceCell::const_new();

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

pub async fn get_blockchain_connection() -> &'static BlockchainService {
    BLOCKCHAIN_SERVICES
        .get_or_init(|| async {
            let blockchain = BlockchainService::new()
                .await
                .map_err(|e| anyhow!("Failed to initalize blockchain service: {}", e)).unwrap();
            blockchain
        })
        .await
}