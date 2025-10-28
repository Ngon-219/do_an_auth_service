use anyhow::{Context, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use super::service::BlockchainService;
use crate::entities::wallet;

/// Get user's private key from database
pub async fn get_user_private_key(db: &DatabaseConnection, user_id: &Uuid) -> Result<String> {
    let wallet_info = wallet::Entity::find()
        .filter(wallet::Column::UserId.eq(*user_id))
        .one(db)
        .await
        .context("Failed to query wallet")?
        .ok_or_else(|| anyhow::anyhow!("Wallet not found for user"))?;

    Ok(wallet_info.private_key)
}

/// Create BlockchainService for a specific user
pub async fn get_user_blockchain_service(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<BlockchainService> {
    let private_key = get_user_private_key(db, user_id).await?;
    BlockchainService::new(&private_key).await
}

/// Create BlockchainService with admin private key (for admin operations)
pub async fn get_admin_blockchain_service() -> Result<BlockchainService> {
    use crate::config::APP_CONFIG;
    BlockchainService::new(&APP_CONFIG.admin_private_key).await
}
