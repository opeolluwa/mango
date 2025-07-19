use crate::errors::service_error::ServiceError;

pub fn debug_message_content(message: &str) -> Result<(), ServiceError> {
    log::debug!("Got the folloowing message {:#?}", message);
    Ok(())
}
