pub mod character_professions;
pub mod guild;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}