use reqwest::Client;
use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::models::blizzard_structs::TokenResponse;

lazy_static! {
    static ref ACCESS_TOKEN: Mutex<Option<String>> = Mutex::new(None);
}

pub async fn get_oauth_token() -> Result<String, Box<dyn std::error::Error>> {
    {
        let token = ACCESS_TOKEN.lock().unwrap();
        if let Some(existing_token) = token.clone() {
            return Ok(existing_token);
        }
    }

    let client_id = env::var("BLIZZARD_CLIENT_ID").expect("Missing BLIZZARD_CLIENT_ID");
    let client_secret = env::var("BLIZZARD_CLIENT_SECRET").expect("Missing BLIZZARD_CLIENT_SECRET");

    
    let client = Client::new();
    let response = client
        .post("https://us.battle.net/oauth/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?;

    if !response.status().is_success() {
        println!("Error response body: {:?}", response.text().await?);
        return Err("Failed to get token".into());
    }

    let token_response: TokenResponse = response.json().await?;

    {
        let mut token = ACCESS_TOKEN.lock().unwrap();
        *token = Some(token_response.access_token.clone());
    }

    Ok(token_response.access_token)
}

pub fn get_global_token() -> String {
    let token = ACCESS_TOKEN.lock().unwrap();
    token.clone().unwrap_or_else(|| {
        eprintln!("Warning: Token not set. You should call get_oauth_token first.");
        "Token not set".to_string()
    })
}