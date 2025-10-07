use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDetails {
    pub kullanici_adi: String,
    pub kullanici_kodu: String,
    pub ad: String,
    pub soyad: String,
    pub rol: String,
}

impl Default for UserDetails {
    fn default() -> Self {
        UserDetails {
            kullanici_adi: String::new(),
            kullanici_kodu: String::new(),
            ad: String::new(),
            soyad: String::new(),
            rol: String::new(),
        }
    }
}

pub async fn get_user_details(client: &Client) -> Result<UserDetails, Box<dyn Error>> {
    let response = client
        .get("https://deysis.deu.edu.tr/api/User")
        .send()
        .await?;

    if response.status().is_success() {
        let resp_json: Value = response.json().await?;
        if resp_json["sonuc"].as_bool().unwrap_or(false) {
            let data_str = resp_json["data"].as_str().ok_or("Missing data field")?;
            let user_details: UserDetails = serde_json::from_str(data_str)?;
            Ok(user_details)
        } else {
            Err("API returned unsuccessful result".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}