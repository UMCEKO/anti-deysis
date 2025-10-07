use crate::app::screens::login::UserCreds;
use reqwest::Client;
use serde_json::json;

pub async fn login(http_client: &Client, user_creds: UserCreds) -> Result<(), Box<dyn std::error::Error>> {
    let response = http_client.post("https://deysis.deu.edu.tr/api/Login")
        .header("accept", "application/json, text/plain, */*")
        .header("Referer", "https://deysis.deu.edu.tr/")
        .json(
            &json!({
                "email": user_creds.email,
                "password": user_creds.password,
                "rememberMe": true
            })
        )
        .send()
        .await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            Ok(())
        },
        _status => {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Login failed")))
        }
    }
}
