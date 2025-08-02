use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "forgotten_password.html")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForgottenPasswordTemplate {
    pub otp: String,
}

impl ForgottenPasswordTemplate {
    pub fn new(otp: &str) -> Self {
        Self {
            otp: otp.to_string(),
        }
    }
}
