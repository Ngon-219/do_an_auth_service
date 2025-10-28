use axum::{
    Json, Router,
    extract::Multipart,
    http::StatusCode,
    routing::post,
};
use calamine::{DataType, Reader, Xlsx, open_workbook_from_rs};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, Set};
use std::io::Cursor;
use uuid::Uuid;

use super::dto::{BulkUserError, BulkUserResponse, CreateUserRequest, ExcelUserRow, UserResponse};
use crate::blockchain::BlockchainService;
use crate::entities::sea_orm_active_enums::RoleEnum;
use crate::entities::{user, wallet};
use crate::static_service::{BLOCKCHAIN_SERVICES, DATABASE_CONNECTION};

pub fn create_route() -> Router {
    Router::new()
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/bulk", post(create_users_bulk))
}

/// Handler for creating a single user
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users"
)]
pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, String)> {
    let db = DATABASE_CONNECTION.get().expect("DATABASE_CONNECTION not set");
    let blockchain = BLOCKCHAIN_SERVICES.get().expect("BLOCKCHAIN_SERVICES not set");
    
    let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to hash password: {}", e),
        )
    })?;

    
    let (wallet_address, wallet_private_key) =
        BlockchainService::generate_wallet().map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate wallet: {}", e),
            )
        })?;

    let user_id = Uuid::new_v4();
    let wallet_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let user_model = user::ActiveModel {
        user_id: Set(user_id),
        first_name: Set(payload.first_name.clone()),
        last_name: Set(payload.last_name.clone()),
        address: Set(payload.address.clone()),
        email: Set(payload.email.clone()),
        password: Set(hashed_password),
        is_priority: Set(false),
        cccd: Set(payload.cccd.clone()),
        phone_number: Set(payload.phone_number.clone()),
        is_first_login: Set(true),
        create_at: Set(now),
        update_at: Set(now),
        role: Set(payload.role.clone()),
    };

    let user = user_model.insert(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {}", e),
        )
    })?;

    let wallet_model = wallet::ActiveModel {
        wallet_id: Set(wallet_id),
        user_id: Set(user_id),
        address: Set(wallet_address.clone()),
        private_key: Set(wallet_private_key.clone()), // Store encrypted in production!
        chain_type: Set("ethereum".to_string()),
        public_key: Set(wallet_address.clone()),
        status: Set("active".to_string()),
        network_id: Set("1".to_string()), // Mainnet by default
        last_used_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    wallet_model.insert(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create wallet: {}", e),
        )
    })?;

    // Register on blockchain
    match payload.role {
        RoleEnum::Student => {
            let student_code = payload.student_code.ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    "Student code is required for students".to_string(),
                )
            })?;

            let full_name = format!("{} {}", payload.first_name, payload.last_name);

            blockchain
                .register_student(&wallet_address, &student_code, &full_name, &payload.email)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to register student on blockchain: {}", e),
                    )
                })?;
        }
        RoleEnum::Manager => {
            // Use addManager instead of assignRole for managers
            blockchain
                .add_manager(&wallet_address)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to add manager on blockchain: {}", e),
                    )
                })?;
        }
        RoleEnum::Teacher | RoleEnum::Admin => {
            // For Teacher and Admin, use assignRole (requires owner)
            let role_code = match payload.role {
                RoleEnum::Admin => 3,
                RoleEnum::Teacher => 2,
                _ => 0,
            };

            blockchain
                .assign_role(&wallet_address, role_code)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to assign role on blockchain (you need to be contract owner): {}", e),
                    )
                })?;
        }
    }

    let response = UserResponse {
        user_id: user.user_id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        role: user.role,
        wallet_address,
        wallet_private_key,
        is_first_login: user.is_first_login,
        created_at: user.create_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler for bulk user creation from Excel file
#[utoipa::path(
    post,
    path = "/api/v1/users/bulk",
    request_body(content = String, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Bulk user creation completed", body = BulkUserResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users"
)]
pub async fn create_users_bulk(
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<BulkUserResponse>), (StatusCode, String)> {
    // Get DB and blockchain from global state
    let db = DATABASE_CONNECTION.get().expect("DATABASE_CONNECTION not set");
    let blockchain = BLOCKCHAIN_SERVICES.get().expect("BLOCKCHAIN_SERVICES not set");
    let mut file_data: Option<Vec<u8>> = None;

    // Extract file from multipart
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to read multipart: {}", e),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Failed to read file: {}", e),
                )
            })?;
            file_data = Some(data.to_vec());
            break;
        }
    }

    let file_data =
        file_data.ok_or_else(|| (StatusCode::BAD_REQUEST, "No file provided".to_string()))?;

    // Parse Excel file
    let cursor = Cursor::new(file_data);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to open Excel file: {}", e),
        )
    })?;

    let sheet_names = workbook.sheet_names().to_owned();
    let first_sheet = sheet_names.first().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "Excel file has no sheets".to_string(),
        )
    })?;

    let range = workbook.worksheet_range(first_sheet).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to read sheet: {}", e),
        )
    })?;

    let mut users_data: Vec<ExcelUserRow> = Vec::new();
    let mut errors: Vec<BulkUserError> = Vec::new();

    // Parse rows (skip header row)
    for (idx, row) in range.rows().enumerate().skip(1) {
        let row_num = idx + 1;

        let parse_result: Result<ExcelUserRow, String> = (|| {
            let get_cell = |col: usize| -> Result<String, String> {
                row.get(col)
                    .ok_or_else(|| format!("Missing column {}", col))?
                    .as_string()
                    .map(|s| s.to_string())
                    .ok_or_else(|| format!("Invalid data in column {}", col))
            };

            let user_row = ExcelUserRow {
                first_name: get_cell(0)?,
                last_name: get_cell(1)?,
                address: get_cell(2)?,
                email: get_cell(3)?,
                password: get_cell(4)?,
                cccd: get_cell(5)?,
                phone_number: get_cell(6)?,
                role: get_cell(7)?,
                student_code: row
                    .get(8)
                    .and_then(|cell| cell.as_string())
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty()),
            };

            user_row.validate()?;
            Ok(user_row)
        })();

        match parse_result {
            Ok(user_row) => users_data.push(user_row),
            Err(error) => {
                errors.push(BulkUserError {
                    row: row_num,
                    email: row
                        .get(3)
                        .and_then(|c| c.as_string())
                        .unwrap_or("unknown".to_string())
                        .to_string(),
                    error,
                });
            }
        }
    }

    let total_records = users_data.len() + errors.len();
    let mut successful = 0;

    // Prepare data for batch blockchain registration
    let mut student_addresses = Vec::new();
    let mut student_codes = Vec::new();
    let mut student_names = Vec::new();
    let mut student_emails = Vec::new();

    // Process each user
    for user_data in users_data.iter() {
        // Generate wallet
        let (wallet_address, wallet_private_key) = match BlockchainService::generate_wallet() {
            Ok(wallet) => wallet,
            Err(e) => {
                errors.push(BulkUserError {
                    row: 0,
                    email: user_data.email.clone(),
                    error: format!("Failed to generate wallet: {}", e),
                });
                continue;
            }
        };

        let user_id = Uuid::new_v4();
        let wallet_id = Uuid::new_v4();
        let now = Utc::now().naive_utc();

        let hashed_password = match bcrypt::hash(&user_data.password, bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(e) => {
                errors.push(BulkUserError {
                    row: 0,
                    email: user_data.email.clone(),
                    error: format!("Failed to hash password: {}", e),
                });
                continue;
            }
        };

        let role = match user_data.parse_role() {
            Ok(r) => r,
            Err(e) => {
                errors.push(BulkUserError {
                    row: 0,
                    email: user_data.email.clone(),
                    error: e,
                });
                continue;
            }
        };

        // Insert user into database
        let user_model = user::ActiveModel {
            user_id: Set(user_id),
            first_name: Set(user_data.first_name.clone()),
            last_name: Set(user_data.last_name.clone()),
            address: Set(user_data.address.clone()),
            email: Set(user_data.email.clone()),
            password: Set(hashed_password),
            is_priority: Set(false),
            cccd: Set(user_data.cccd.clone()),
            phone_number: Set(user_data.phone_number.clone()),
            is_first_login: Set(true),
            create_at: Set(now),
            update_at: Set(now),
            role: Set(role.clone()),
        };

        if let Err(e) = user_model.insert(db).await {
            errors.push(BulkUserError {
                row: 0,
                email: user_data.email.clone(),
                error: format!("Failed to create user: {}", e),
            });
            continue;
        }

        // Create wallet record
        let wallet_model = wallet::ActiveModel {
            wallet_id: Set(wallet_id),
            user_id: Set(user_id),
            address: Set(wallet_address.clone()),
            private_key: Set(wallet_private_key.clone()),
            chain_type: Set("ethereum".to_string()),
            public_key: Set(wallet_address.clone()),
            status: Set("active".to_string()),
            network_id: Set("1".to_string()),
            last_used_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        if let Err(e) = wallet_model.insert(db).await {
            errors.push(BulkUserError {
                row: 0,
                email: user_data.email.clone(),
                error: format!("Failed to create wallet: {}", e),
            });
            continue;
        }

        // Collect student data for batch registration
        if role == RoleEnum::Student {
            if let Some(student_code) = &user_data.student_code {
                student_addresses.push(wallet_address.clone());
                student_codes.push(student_code.clone());
                student_names.push(format!("{} {}", user_data.first_name, user_data.last_name));
                student_emails.push(user_data.email.clone());
            }
        }

        successful += 1;
    }

    // Batch register students on blockchain (max 50 at a time)
    if !student_addresses.is_empty() {
        for chunk in 0..(student_addresses.len() + 49) / 50 {
            let start = chunk * 50;
            let end = std::cmp::min(start + 50, student_addresses.len());

            if let Err(e) = blockchain
                .register_students_batch(
                    student_addresses[start..end].to_vec(),
                    student_codes[start..end].to_vec(),
                    student_names[start..end].to_vec(),
                    student_emails[start..end].to_vec(),
                )
                .await
            {
                tracing::error!("Failed to register batch on blockchain: {}", e);
                // Don't fail the entire operation, just log the error
            }
        }
    }

    let response = BulkUserResponse {
        total_records,
        successful,
        failed: errors.len(),
        errors,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
