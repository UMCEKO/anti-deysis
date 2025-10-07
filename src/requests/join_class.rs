use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn join_class(client: &Client, code: String, coordinates: (f64, f64)) -> Result<bool, Box<dyn Error>> {
    let coord_str = format!("{},{}", coordinates.0, coordinates.1);
    let response = client
        .post("https://deysis.deu.edu.tr/api/Ogrenci/YoklamaKatil")
        .json(&json!({
            "GIRIS_TIPI": "K",
            "KOD": code,
            "KONUM": coord_str
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