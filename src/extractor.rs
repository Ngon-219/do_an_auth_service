use axum::extract::FromRequestParts;
use do_an_lib::jwt::JwtManager;
use do_an_lib::errors::common_errors::Error as AppErrors;
use http::request::Parts;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use do_an_lib::structs::token_claims::{TokenClaims, UserRole};

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

        let token_data =
            jwt_manager.decode_jwt(bearer.token()).map_err(|_| AppErrors::unauthorized("Invalid jwt token"))?;

        // let user_info = Account::get_account_by_user_id(&token_data.user_id).await?;
        //
        // let _ = user_info.ok_or_else(|| {
        //     AppError::unauthorized("The user belonging to this token no longer exists")
        // })?;

        let path = parts.uri.path().to_string();
        let method = &parts.method;

        // if !verify_scope(path, method, token_data.clone()) {
        //     return Err(AppError::forbidden(
        //         "You do not have permission to access this resource",
        //     ));
        // }

        let claims = TokenClaims {
            user_id: token_data.user_id,
            user_name: token_data.user_name,
            iap: token_data.iap,
            iat: token_data.iat,
            exp: token_data.exp,
            role: UserRole::ADMIN,
        };

        Ok(AuthClaims(claims))
    }
}
