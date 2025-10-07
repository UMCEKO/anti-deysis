use crate::requests::refresh_token::refresh_token;
use reqwest::Client;

pub async fn check_auth(client: &Client) -> bool {
    let success = eval_auth(client).await;
    match success {
        true => true,
        false => {
            refresh_token(client).await;
            eval_auth(client).await
        }
    }
}

async fn eval_auth(client: &Client) -> bool {
    let response = client.get("https://deysis.deu.edu.tr/api/CheckAuth")
        .send()
        .await;

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AuthResponse {
        is_auth: bool,
    }

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let auth_response= match resp.json::<AuthResponse>().await {
                    Ok(data) => data,
                    Err(_e) => return false,
                };
                auth_response.is_auth
            } else {
                false
            }
        },
        Err(_e) => {
            false
        }
    }
}