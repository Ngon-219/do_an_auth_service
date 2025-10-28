use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post, put},
};

use super::dto::{
    StudentAddressRequest, StudentCodeRequest, StudentIdResponse,
    StudentInfoResponse, StudentStatusResponse, SystemInfoResponse,
};
use crate::static_service::BLOCKCHAIN_SERVICES;

pub fn create_route() -> Router {
    Router::new()
        .route("/api/v1/students/{student_id}", get(get_student_by_id))
        .route("/api/v1/students/by-address", post(get_student_id_by_address))
        .route("/api/v1/students/by-code", post(get_student_id_by_code))
        .route("/api/v1/students/{student_id}/deactivate", put(deactivate_student))
        .route("/api/v1/students/{student_id}/activate", put(activate_student))
        .route("/api/v1/students/check-active", post(check_student_active))
        .route("/api/v1/system/info", get(get_system_info))
}

/// Get student information by ID
#[utoipa::path(
    get,
    path = "/api/v1/students/{student_id}",
    params(
        ("student_id" = u64, Path, description = "Student ID")
    ),
    responses(
        (status = 200, description = "Student retrieved successfully", body = StudentInfoResponse),
        (status = 404, description = "Student not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Students"
)]
pub async fn get_student_by_id(
    Path(student_id): Path<u64>,
) -> Result<(StatusCode, Json<StudentInfoResponse>), (StatusCode, String)> {
    let blockchain = BLOCKCHAIN_SERVICES
        .get()
        .expect("BLOCKCHAIN_SERVICES not set");

    let student = blockchain.get_student(student_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get student: {}", e),
        )
    })?;

    let response = StudentInfoResponse {
        id: student.id,
        wallet_address: student.wallet_address,
        student_code: student.student_code,
        full_name: student.full_name,
        email: student.email,
        is_active: student.is_active,
        registered_at: student.registered_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Get student ID by wallet address
#[utoipa::path(
    post,
    path = "/api/v1/students/by-address",
    request_body = StudentAddressRequest,
    responses(
        (status = 200, description = "Student ID retrieved", body = StudentIdResponse),
        (status = 404, description = "Student not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Students"
)]
pub async fn get_student_id_by_address(
    Json(payload): Json<StudentAddressRequest>,
) -> Result<(StatusCode, Json<StudentIdResponse>), (StatusCode, String)> {
    let blockchain = BLOCKCHAIN_SERVICES
        .get()
        .expect("BLOCKCHAIN_SERVICES not set");

    let student_id = blockchain
        .get_student_id_by_address(&payload.address)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get student ID: {}", e),
            )
        })?;

    if student_id == 0 {
        return Err((StatusCode::NOT_FOUND, "Student not found".to_string()));
    }

    let response = StudentIdResponse { student_id };

    Ok((StatusCode::OK, Json(response)))
}

/// Get student ID by student code
#[utoipa::path(
    post,
    path = "/api/v1/students/by-code",
    request_body = StudentCodeRequest,
    responses(
        (status = 200, description = "Student ID retrieved", body = StudentIdResponse),
        (status = 404, description = "Student not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Students"
)]
pub async fn get_student_id_by_code(
    Json(payload): Json<StudentCodeRequest>,
) -> Result<(StatusCode, Json<StudentIdResponse>), (StatusCode, String)> {
    let blockchain = BLOCKCHAIN_SERVICES
        .get()
        .expect("BLOCKCHAIN_SERVICES not set");

    let student_id = blockchain
        .get_student_id_by_code(&payload.student_code)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get student ID: {}", e),
            )
        })?;

    if student_id == 0 {
        return Err((StatusCode::NOT_FOUND, "Student not found".to_string()));
    }

    let response = StudentIdResponse { student_id };

    Ok((StatusCode::OK, Json(response)))
}

/// Deactivate a student
#[utoipa::path(
    put,
    path = "/api/v1/students/{student_id}/deactivate",
    params(
        ("student_id" = u64, Path, description = "Student ID")
    ),
    responses(
        (status = 200, description = "Student deactivated", body = StudentStatusResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Students"
)]
pub async fn deactivate_student(
    Path(student_id): Path<u64>,
) -> Result<(StatusCode, Json<StudentStatusResponse>), (StatusCode, String)> {
    let blockchain = BLOCKCHAIN_SERVICES
        .get()
        .expect("BLOCKCHAIN_SERVICES not set");

    blockchain
        .deactivate_student(student_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to deactivate student: {}", e),
            )
        })?;

    let response = StudentStatusResponse {
        student_id,
        is_active: false,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Activate a student
#[utoipa::path(
    put,
    path = "/api/v1/students/{student_id}/activate",
    params(
        ("student_id" = u64, Path, description = "Student ID")
    ),
    responses(
        (status = 200, description = "Student activated", body = StudentStatusResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Students"
)]
pub async fn activate_student(
    Path(student_id): Path<u64>,
) -> Result<(StatusCode, Json<StudentStatusResponse>), (StatusCode, String)> {
    let blockchain = BLOCKCHAIN_SERVICES
        .get()
        .expect("BLOCKCHAIN_SERVICES not set");

    blockchain.activate_student(student_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to activate student: {}", e),
        )
    })?;

    let response = StudentStatusResponse {
        student_id,
        is_active: true,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Check if student is active by address
#[utoipa::path(
    post,
    path = "/api/v1/students/check-active",
    request_body = StudentAddressRequest,
    responses(
        (status = 200, description = "Student status retrieved", body = StudentStatusResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Students"
)]
pub async fn check_student_active(
    Json(payload): Json<StudentAddressRequest>,
) -> Result<(StatusCode, Json<StudentStatusResponse>), (StatusCode, String)> {
    let blockchain = BLOCKCHAIN_SERVICES
        .get()
        .expect("BLOCKCHAIN_SERVICES not set");

    let is_active = blockchain
        .is_active_student(&payload.address)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to check student status: {}", e),
            )
        })?;

    let student_id = blockchain
        .get_student_id_by_address(&payload.address)
        .await
        .unwrap_or(0);

    let response = StudentStatusResponse {
        student_id,
        is_active,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Get system information
#[utoipa::path(
    get,
    path = "/api/v1/system/info",
    responses(
        (status = 200, description = "System info retrieved", body = SystemInfoResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "System"
)]
pub async fn get_system_info(
) -> Result<(StatusCode, Json<SystemInfoResponse>), (StatusCode, String)> {
    let blockchain = BLOCKCHAIN_SERVICES
        .get()
        .expect("BLOCKCHAIN_SERVICES not set");

    let (owner, student_count, manager_count) =
        blockchain.get_contract_info().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get contract info: {}", e),
            )
        })?;

    let response = SystemInfoResponse {
        owner,
        total_students: student_count,
        total_managers: manager_count,
    };

    Ok((StatusCode::OK, Json(response)))
}

