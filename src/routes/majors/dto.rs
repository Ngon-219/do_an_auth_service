use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateMajorRequest {
    pub name: String,
    #[schema(example = "2025-10-28T16:54:21")]
    pub founding_date: NaiveDateTime,
    pub department_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateMajorRequest {
    pub name: Option<String>,
    pub founding_date: Option<NaiveDateTime>,
    pub department_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MajorResponse {
    pub major_id: Uuid,
    pub name: String,
    pub founding_date: NaiveDateTime,
    pub department_id: Option<Uuid>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MajorListResponse {
    pub majors: Vec<MajorResponse>,
    pub total: usize,
}
