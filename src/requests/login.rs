use crate::utils::config::{Config, UserCredentials};
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use serde_json::json;
use std::sync::{Arc, Mutex};

pub async fn login(http_client: &Client, mut user_creds: UserCredentials, config: Arc<Mutex<Config>>, cookies: Arc<CookieStoreMutex>) -> Result<(), Box<dyn std::error::Error>> {
    let response = http_client.post("https://deysis.deu.edu.tr/api/Login")
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
            {
                let mut config = config.lock().unwrap();
                match config.users.get_mut(&user_creds.email) {
                    Some(user) => {
                        user.password = user_creds.password;
                        user.cookies = cookies.lock().unwrap().clone();
                    },
                    None => {
                        user_creds.cookies = cookies.lock().unwrap().clone();
                        config.users.insert(user_creds.email.clone(), user_creds);
                    }
                }

                config.save_to_file();
            }
            Ok(())
        },
        _status => {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Login failed")))
        }
    }
}
