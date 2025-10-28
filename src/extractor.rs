use crate::entities::user;
use crate::entities::user::Entity as UserModel;
use axum::extract::FromRequestParts;
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use do_an_lib::errors::common_errors::Error as AppErrors;
use do_an_lib::jwt::JwtManager;
use do_an_lib::structs::token_claims::{TokenClaims, UserRole};
use http::request::Parts;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::entities::sea_orm_active_enums::RoleEnum;
use crate::static_service::DATABASE_CONNECTION;

pub struct AuthClaims(pub TokenClaims);

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = AppErrors;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| AppErrors::unauthorized("Authorization header missing"))?;

        let jwt_manager = JwtManager::new("secret_key".to_string());

        let token_data = jwt_manager
            .decode_jwt(bearer.token())
            .map_err(|_| AppErrors::unauthorized("Invalid jwt token"))?;

        let db = DATABASE_CONNECTION.get().expect("DATABASE_CONNECTION not set");
        let user_info = UserModel::find()
            .filter(<user::Entity as sea_orm::EntityTrait>::Column::UserId.eq(token_data.user_id.clone()))
            .one(db)
            .await?;

        if user_info.is_none() {
            AppErrors::unauthorized("User not found");
        }

        let _path = parts.uri.path().to_string();
        let _method = &parts.method;

        let user_role = match user_info.unwrap().role{
            RoleEnum::Admin => UserRole::ADMIN,
            RoleEnum::Manager => UserRole::MANAGER,
            RoleEnum::Student => UserRole::STUDENT,
            RoleEnum::Teacher => UserRole::TEACHER,
        };

        let claims = TokenClaims {
            user_id: token_data.user_id,
            user_name: token_data.user_name,
            iap: token_data.iap,
            iat: token_data.iat,
            exp: token_data.exp,
            role: user_role,
        };

        Ok(AuthClaims(claims))
    }
}
