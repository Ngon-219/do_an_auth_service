use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentIdRequest {
    pub student_id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentAddressRequest {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentCodeRequest {
    pub student_code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentInfoResponse {
    pub id: u64,
    pub wallet_address: String,
    pub student_code: String,
    pub full_name: String,
    pub email: String,
    pub is_active: bool,
    pub registered_at: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentStatusResponse {
    pub student_id: u64,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentIdResponse {
    pub student_id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemInfoResponse {
    pub owner: String,
    pub total_students: u64,
    pub total_managers: u64,
}
