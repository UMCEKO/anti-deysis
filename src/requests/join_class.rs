use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn join_class(client: &Client, code: String) -> Result<bool, Box<dyn Error>> {
    let lat: f64 = 38.366966;
    let lon: f64 = 27.202704;
    // Add random noise to the coordinates 0.000001 to 0.000009
    let noise_lat: f64 = (rand::random::<u32>() % 9 + 1) as f64 * 0.000001;
    let noise_lon: f64 = (rand::random::<u32>() % 9 + 1) as f64 * 0.000001;
    let lat = lat + noise_lat;
    let lon = lon + noise_lon;
    let coordinates = format!("{},{}", lat, lon);
    let response = client
        .post("https://deysis.deu.edu.tr/api/Ogrenci/YoklamaKatil")
        .json(&json!({
            "GIRIS_TIPI": "K",
            "KOD": code,
            "KONUM": coordinates
        }))
        .send()
        .await?;

    #[derive(serde::Deserialize)]
    struct Response {
        pub message: String,
    }
    if response.status().is_success() {
        Ok(true)
    }
    else {
        let resp_json: Response = response.json().await?;
        match resp_json.message.as_str() {
            "Yoklama bulunamadı" => {
                Err("Yoklama bulunamadı, kodu kontrol edin.".into())
            }
            _ => {
                Err(format!("Bir hata ile karşılaşıldı: {}", resp_json.message).into())
            }
        }
    }
}