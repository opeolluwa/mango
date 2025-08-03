use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCachedUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,

}