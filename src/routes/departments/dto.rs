use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateDepartmentRequest {
    pub name: String,
    #[schema(example = "2025-10-28T16:54:21")]
    pub founding_date: NaiveDateTime,
    pub dean: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateDepartmentRequest {
    pub name: Option<String>,
    pub founding_date: Option<NaiveDateTime>,
    pub dean: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentResponse {
    pub department_id: Uuid,
    pub name: String,
    pub founding_date: NaiveDateTime,
    pub dean: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepartmentListResponse {
    pub departments: Vec<DepartmentResponse>,
    pub total: usize,
}
