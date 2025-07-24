use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Validation, decode};

use crate::adapters::jwt::Keys;
use crate::{
    adapters::jwt::Claims, errors::auth_error::AuthenticationError,
    shared::extract_env::extract_env,
};

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthenticationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationError::from)?;

        let decoding_key = Keys::new(secret.as_bytes()).decoding;
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|err| {
                log::error!("failed to extract authorization header due to {}", err);
                AuthenticationError::MissingCredentials
            })?;
        // Decode the user data
        let mut jwt_validation = Validation::default();
        jwt_validation.set_audience(&["eckko.mobile"]);
        let token_data =
            decode::<Claims>(bearer.token(), &decoding_key, &jwt_validation).map_err(|err| {
                log::error!("failed to decode JWT token due to {}", err);
                AuthenticationError::InvalidToken
            })?;

        Ok(token_data.claims)
    }
}
