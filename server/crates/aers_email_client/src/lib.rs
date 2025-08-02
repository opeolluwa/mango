mod confirm_email;
mod email;
mod email_client;
mod errors;

pub use confirm_email::ConfirmEmailTemplate;
pub use email::Email;
pub use email_client::EmailClient;
pub use email_client::EmailClientExt;
pub use errors::EmailError;
