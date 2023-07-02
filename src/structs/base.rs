pub struct AuthGuard;

use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: u16,
    pub refresh_expires_in: u16,
    pub refresh_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenApplication {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}