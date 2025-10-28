use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use super::dto::{
    CreateDepartmentRequest, DepartmentListResponse, DepartmentResponse, UpdateDepartmentRequest,
};
use crate::entities::department;
use crate::static_service::DATABASE_CONNECTION;

pub fn create_route() -> Router {
    Router::new()
        .route("/api/v1/departments", post(create_department))
        .route("/api/v1/departments", get(get_all_departments))
        .route("/api/v1/departments/{department_id}", get(get_department))
        .route(
            "/api/v1/departments/{department_id}",
            put(update_department),
        )
        .route(
            "/api/v1/departments/{department_id}",
            delete(delete_department),
        )
}

/// Create a new department
#[utoipa::path(
    post,
    path = "/api/v1/departments",
    request_body = CreateDepartmentRequest,
    responses(
        (status = 201, description = "Department created", body = DepartmentResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
pub async fn create_department(
    Json(payload): Json<CreateDepartmentRequest>,
) -> Result<(StatusCode, Json<DepartmentResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let department_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let department_model = department::ActiveModel {
        department_id: Set(department_id),
        name: Set(payload.name),
        founding_date: Set(payload.founding_date),
        dean: Set(payload.dean),
        create_at: Set(now),
        update_at: Set(now),
    };

    let department = department_model.insert(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create department: {}", e),
        )
    })?;

    let response = DepartmentResponse {
        department_id: department.department_id,
        name: department.name,
        founding_date: department.founding_date,
        dean: department.dean,
        create_at: department.create_at,
        update_at: department.update_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Get all departments
#[utoipa::path(
    get,
    path = "/api/v1/departments",
    responses(
        (status = 200, description = "Departments retrieved", body = DepartmentListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
pub async fn get_all_departments()
-> Result<(StatusCode, Json<DepartmentListResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let departments = department::Entity::find().all(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get departments: {}", e),
        )
    })?;

    let response = DepartmentListResponse {
        total: departments.len(),
        departments: departments
            .into_iter()
            .map(|d| DepartmentResponse {
                department_id: d.department_id,
                name: d.name,
                founding_date: d.founding_date,
                dean: d.dean,
                create_at: d.create_at,
                update_at: d.update_at,
            })
            .collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Get department by ID
#[utoipa::path(
    get,
    path = "/api/v1/departments/{department_id}",
    params(
        ("department_id" = Uuid, Path, description = "Department ID")
    ),
    responses(
        (status = 200, description = "Department retrieved", body = DepartmentResponse),
        (status = 404, description = "Department not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
pub async fn get_department(
    Path(department_id): Path<Uuid>,
) -> Result<(StatusCode, Json<DepartmentResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let department = department::Entity::find()
        .filter(department::Column::DepartmentId.eq(department_id))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get department: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Department not found".to_string()))?;

    let response = DepartmentResponse {
        department_id: department.department_id,
        name: department.name,
        founding_date: department.founding_date,
        dean: department.dean,
        create_at: department.create_at,
        update_at: department.update_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Update department
#[utoipa::path(
    put,
    path = "/api/v1/departments/{department_id}",
    params(
        ("department_id" = Uuid, Path, description = "Department ID")
    ),
    request_body = UpdateDepartmentRequest,
    responses(
        (status = 200, description = "Department updated", body = DepartmentResponse),
        (status = 404, description = "Department not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
pub async fn update_department(
    Path(department_id): Path<Uuid>,
    Json(payload): Json<UpdateDepartmentRequest>,
) -> Result<(StatusCode, Json<DepartmentResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let department = department::Entity::find()
        .filter(department::Column::DepartmentId.eq(department_id))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to find department: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Department not found".to_string()))?;

    let mut active_model: department::ActiveModel = department.into();

    if let Some(name) = payload.name {
        active_model.name = Set(name);
    }
    if let Some(founding_date) = payload.founding_date {
        active_model.founding_date = Set(founding_date);
    }
    if let Some(dean) = payload.dean {
        active_model.dean = Set(dean);
    }
    active_model.update_at = Set(Utc::now().naive_utc());

    let updated = active_model.update(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update department: {}", e),
        )
    })?;

    let response = DepartmentResponse {
        department_id: updated.department_id,
        name: updated.name,
        founding_date: updated.founding_date,
        dean: updated.dean,
        create_at: updated.create_at,
        update_at: updated.update_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Delete department
#[utoipa::path(
    delete,
    path = "/api/v1/departments/{department_id}",
    params(
        ("department_id" = Uuid, Path, description = "Department ID")
    ),
    responses(
        (status = 204, description = "Department deleted"),
        (status = 404, description = "Department not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
pub async fn delete_department(
    Path(department_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = DATABASE_CONNECTION
        .get()
        .expect("DATABASE_CONNECTION not set");

    let department = department::Entity::find()
        .filter(department::Column::DepartmentId.eq(department_id))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to find department: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Department not found".to_string()))?;

    let active_model: department::ActiveModel = department.into();
    active_model.delete(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete department: {}", e),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}
