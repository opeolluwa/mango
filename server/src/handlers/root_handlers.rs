use crate::adapters::api_response::{ApiResponse, ApiResponseBuilder, EmptyResponseBody};
use crate::adapters::authentication::CreateUserRequest;
use crate::events::channels::EventChannel;
use crate::events::redis::{RedisClient, RedisClientExt};
use crate::services::root_serice::RootServiceTrait;
use crate::{errors::app_error::AppError, services::root_serice::RootService};
use axum::extract::State;

pub async fn health_check(
    State(root_service): State<RootService>,
) -> Result<ApiResponse<EmptyResponseBody>, AppError> {
    let mut redis_client = RedisClient::new().await.unwrap();

    let res = redis_client
        .publish_message(
            &EventChannel::Default,
            &CreateUserRequest {
                email: "opeolluwa".to_string(),
                password: "test".to_string(),
            },
        )
        .await;
    println!(" publish event{:#?} ", res);

    root_service.health_check()?;
    Ok(ApiResponseBuilder::new()
        .message("service is healthy")
        .build())
}
