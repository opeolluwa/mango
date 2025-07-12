use std::fmt::Debug;

use axum::extract::{FromRequest, FromRequestParts, Request};
use serde::{Serialize, de::DeserializeOwned};

use crate::{adapters::jwt::Claims, errors::auth_error::AuthenticationError};

pub struct AuthenticatedRequest<T>
where
    T: Debug + Serialize + DeserializeOwned,
{
    pub data: T,
    pub claims: Claims,
}

impl<S, T> FromRequest<S> for AuthenticatedRequest<T>
where
    T: FromRequest<S> + Send + Serialize + DeserializeOwned + Debug,
    Claims: FromRequestParts<S> + Send,
    S: Send + Sync,
{
    type Rejection = AuthenticationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let claims: crate::adapters::jwt::JwtCredentials =
            Claims::from_request_parts(&mut parts.clone(), state)
                .await
                .map_err(|_| AuthenticationError::MissingCredentials)?;

        let data = T::from_request(Request::from_parts(parts, body), state)
            .await
            .map_err(|_| AuthenticationError::MissingCredentials)?;

        //todo: validate the data
        Ok(Self { data, claims })
    }
}
