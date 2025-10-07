use reqwest::header::HeaderMap;
use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;

pub async fn initialize_client(cookie_store: Arc<CookieStoreMutex>) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let mut default_headers = HeaderMap::new();
    default_headers.append("accept", "application/json, text/plain, */*".parse()? );
    default_headers.append("accept-language", "en-US,en;q=0.5".parse()? );
    default_headers.append("cache-control", "no-cache".parse()? );
    default_headers.append("pragma", "no-cache".parse()? );
    default_headers.append("priority", "u=1, i".parse()? );
    default_headers.append("sec-ch-ua", "\"Chromium\";v=\"140\", \"Not=A?Brand\";v=\"24\", \"Brave\";v=\"140\"".parse()? );
    default_headers.append("sec-ch-ua-mobile", "?0".parse()? );
    default_headers.append("sec-ch-ua-platform", "\"Linux\"".parse()? );
    default_headers.append("sec-fetch-dest", "empty".parse()? );
    default_headers.append("sec-fetch-mode", "cors".parse()? );
    default_headers.append("sec-fetch-site", "same-origin".parse()? );
    default_headers.append("sec-gpc", "1".parse()?);
    Ok(
        reqwest::Client::builder()
            .cookie_provider(cookie_store)
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36")
            .default_headers(default_headers)
            .build()?
    )
}