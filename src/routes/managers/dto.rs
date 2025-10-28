use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddManagerRequest {
    pub manager_address: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RemoveManagerRequest {
    pub manager_address: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ManagerResponse {
    pub address: String,
    pub is_manager: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ManagerListResponse {
    pub managers: Vec<String>,
    pub total_count: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CheckManagerRequest {
    pub address: String,
}
