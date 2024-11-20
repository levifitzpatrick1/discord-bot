pub mod character_professions;
pub mod character_score;
pub mod guild;
pub mod recipe_materials;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}