use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "confirm_email.html")]

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfirmEmailTemplate {
    pub email: String,
    pub verification_link: String,
}

impl ConfirmEmailTemplate {
    pub fn new(email: &str, verification_link: &str) -> Self {
        Self {
            email: email.to_string(),
            verification_link: verification_link.to_string(),
        }
    }
}
