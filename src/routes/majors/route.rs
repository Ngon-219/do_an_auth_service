use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use super::dto::{CreateMajorRequest, MajorListResponse, MajorResponse, UpdateMajorRequest};
use crate::entities::major;
use crate::static_service::DATABASE_CONNECTION;

pub fn create_route() -> Router {
    Router::new()
        .route("/api/v1/majors", post(create_major))
        .route("/api/v1/majors", get(get_all_majors))
        .route("/api/v1/majors/{major_id}", get(get_major))
        .route("/api/v1/majors/{major_id}", put(update_major))
        .route("/api/v1/majors/{major_id}", delete(delete_major))
}

/// Create a new major
#[utoipa::path(
    post,
    path = "/api/v1/majors",
    request_body = CreateMajorRequest,
    responses(
        (status = 201, description = "Major created", body = MajorResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Majors"
)]
pub async fn create_major(
    Json(payload): Json<CreateMajorRequest>,
) -> Result<(StatusCode, Json<MajorResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let major_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let major_model = major::ActiveModel {
        major_id: Set(major_id),
        name: Set(payload.name),
        founding_date: Set(payload.founding_date),
        department_id: Set(payload.department_id),
        create_at: Set(now),
        update_at: Set(now),
    };

    let major = major_model.insert(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create major: {}", e),
        )
    })?;

    let response = MajorResponse {
        major_id: major.major_id,
        name: major.name,
        founding_date: major.founding_date,
        department_id: major.department_id,
        create_at: major.create_at,
        update_at: major.update_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Get all majors
#[utoipa::path(
    get,
    path = "/api/v1/majors",
    responses(
        (status = 200, description = "Majors retrieved", body = MajorListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Majors"
)]
pub async fn get_all_majors() -> Result<(StatusCode, Json<MajorListResponse>), (StatusCode, String)>
{
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let majors = major::Entity::find().all(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get majors: {}", e),
        )
    })?;

    let response = MajorListResponse {
        total: majors.len(),
        majors: majors
            .into_iter()
            .map(|m| MajorResponse {
                major_id: m.major_id,
                name: m.name,
                founding_date: m.founding_date,
                department_id: m.department_id,
                create_at: m.create_at,
                update_at: m.update_at,
            })
            .collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Get major by ID
#[utoipa::path(
    get,
    path = "/api/v1/majors/{major_id}",
    params(
        ("major_id" = Uuid, Path, description = "Major ID")
    ),
    responses(
        (status = 200, description = "Major retrieved", body = MajorResponse),
        (status = 404, description = "Major not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Majors"
)]
pub async fn get_major(
    Path(major_id): Path<Uuid>,
) -> Result<(StatusCode, Json<MajorResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let major = major::Entity::find()
        .filter(major::Column::MajorId.eq(major_id))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get major: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Major not found".to_string()))?;

    let response = MajorResponse {
        major_id: major.major_id,
        name: major.name,
        founding_date: major.founding_date,
        department_id: major.department_id,
        create_at: major.create_at,
        update_at: major.update_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Update major
#[utoipa::path(
    put,
    path = "/api/v1/majors/{major_id}",
    params(
        ("major_id" = Uuid, Path, description = "Major ID")
    ),
    request_body = UpdateMajorRequest,
    responses(
        (status = 200, description = "Major updated", body = MajorResponse),
        (status = 404, description = "Major not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Majors"
)]
pub async fn update_major(
    Path(major_id): Path<Uuid>,
    Json(payload): Json<UpdateMajorRequest>,
) -> Result<(StatusCode, Json<MajorResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let major = major::Entity::find()
        .filter(major::Column::MajorId.eq(major_id))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to find major: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Major not found".to_string()))?;

    let mut active_model: major::ActiveModel = major.into();

    if let Some(name) = payload.name {
        active_model.name = Set(name);
    }
    if let Some(founding_date) = payload.founding_date {
        active_model.founding_date = Set(founding_date);
    }
    if let Some(department_id) = payload.department_id {
        active_model.department_id = Set(Some(department_id));
    }
    active_model.update_at = Set(Utc::now().naive_utc());

    let updated = active_model.update(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update major: {}", e),
        )
    })?;

    let response = MajorResponse {
        major_id: updated.major_id,
        name: updated.name,
        founding_date: updated.founding_date,
        department_id: updated.department_id,
        create_at: updated.create_at,
        update_at: updated.update_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Delete major
#[utoipa::path(
    delete,
    path = "/api/v1/majors/{major_id}",
    params(
        ("major_id" = Uuid, Path, description = "Major ID")
    ),
    responses(
        (status = 204, description = "Major deleted"),
        (status = 404, description = "Major not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Majors"
)]
pub async fn delete_major(Path(major_id): Path<Uuid>) -> Result<StatusCode, (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let major = major::Entity::find()
        .filter(major::Column::MajorId.eq(major_id))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to find major: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Major not found".to_string()))?;

    let active_model: major::ActiveModel = major.into();
    active_model.delete(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete major: {}", e),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}
