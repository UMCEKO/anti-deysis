use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JoinClassRequest {
    pub giris_tipi: String,
    pub kod: String,
    pub konum: String,
}

pub async fn join_class(client: &Client, code: String) -> Result<bool, Box<dyn Error>> {
    let lat: f64 = 38.366966;
    let lon: f64 = 27.202704;
    // Add random noise to the coordinates 0.000001 to 0.000009
    let noise_lat: f64 = (rand::random::<u32>() % 9 + 1) as f64 * 0.000001;
    let noise_lon: f64 = (rand::random::<u32>() % 9 + 1) as f64 * 0.000001;
    let lat = lat + noise_lat;
    let lon = lon + noise_lon;
    let coordinates = format!("{},{}", lat, lon);
    let req_body = JoinClassRequest {
        kod: code,
        giris_tipi: "K".to_string(),
        konum: coordinates.to_string()
    };
    debug!("Join class request body: {:?}", req_body);
    let response = client
        .post("https://deysis.deu.edu.tr/api/Ogrenci/YoklamaKatil")
        .json(&req_body)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(true)
    }
    else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}