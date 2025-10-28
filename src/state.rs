use sea_orm::DatabaseConnection;
use crate::blockchain::BlockchainService;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub blockchain: BlockchainService,
}

impl AppState {
    pub fn new(db: DatabaseConnection, blockchain: BlockchainService) -> Self {
        Self { db, blockchain }
    }
}

// Implement FromRef to allow extracting individual parts of the state
impl axum::extract::FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl axum::extract::FromRef<AppState> for BlockchainService {
    fn from_ref(state: &AppState) -> Self {
        state.blockchain.clone()
    }
}

