use axum::extract::FromRequestParts;
use async_trait::async_trait;
use axum::http::request::Parts;
use axum::http::StatusCode;
use crate::error::AppError;

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: i32,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_id = 1;
        Ok(AuthUser { user_id })
    }
}
