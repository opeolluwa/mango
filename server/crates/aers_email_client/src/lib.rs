mod confirm_email;
mod email;
mod email_client;
mod errors;
mod forgotten_password;
mod password_updated;
mod welcome;

pub use confirm_email::ConfirmEmailTemplate;
pub use email::Email;
pub use email_client::EmailClient;
pub use email_client::EmailClientExt;
pub use errors::EmailError;
pub use forgotten_password::ForgottenPasswordTemplate;
pub use password_updated::PasswordUpdatedTemplate;
pub use welcome::WelcomeTemplate;