use deeb::Collection;
use deeb::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{api_client::EmptyData, flowlet_context::FlowletContext, util::FlowletResult};

use super::Api;

#[derive(Collection, Deserialize, Serialize)]
pub struct Auth {
    pub _id: ulid::Ulid,
    pub flowlet_token: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to login user.")]
    LoginFailed,
    #[error("Update not supported.")]
    UpdateNotSupported,
    #[error("Failed to find auth.")]
    AuthReadFailed,
    #[error("Auth object not found.")]
    AuthNotFound,
}

#[derive(Serialize)]
pub struct CreateAuthInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

impl Api for Auth {
    type CreateInput = CreateAuthInput;

    async fn create(
        flowlet_context: &FlowletContext,
        input: Self::CreateInput,
    ) -> FlowletResult<Self> {
        let deeb = &flowlet_context.flowlet_db.deeb;
        let client = &flowlet_context.api_client;
        let CreateAuthInput { email, password } = input;

        Auth::delete_many(&deeb, Query::All, None)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                AuthError::LoginFailed
            })?;

        let res = client
            .post::<LoginPayload, LoginResponse>("/auth/login", &LoginPayload { email, password })
            .await
            .map_err(|e| {
                log::error!("Network error: {:?}", e);
                AuthError::LoginFailed
            })?;

        if res.data.is_none() {
            log::error!("Expected data in type response.");
            return Err(Box::new(AuthError::LoginFailed));
        }

        let auth = Auth {
            _id: ulid::Ulid::new(),
            flowlet_token: res.data.unwrap().token,
        };
        let auth = Auth::insert_one(&deeb, auth, None).await.map_err(|e| {
            log::error!("{:?}", e);
            AuthError::LoginFailed
        })?;

        Ok(auth)
    }

    type UpdateInput = EmptyData;

    async fn update(_: &FlowletContext, _: Self::UpdateInput) -> FlowletResult<Self> {
        return Err(Box::new(AuthError::UpdateNotSupported));
    }

    type ReadInput = EmptyData;

    async fn read(
        flowlet_context: &FlowletContext,
        _: Self::ReadInput,
    ) -> FlowletResult<Option<Self>> {
        let deeb = &flowlet_context.flowlet_db.deeb;

        let auth = Auth::find_one(deeb, Query::All, None).await.map_err(|e| {
            log::error!("{:?}", e);
            AuthError::AuthReadFailed
        })?;

        if auth.is_none() {
            return Err(Box::new(AuthError::AuthNotFound));
        }

        Ok(auth)
    }
}
