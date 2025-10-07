//fetch("https://deysis.deu.edu.tr/api/refresh-token", {
//   "headers": {
//     "accept": "application/json, text/plain, */*",
//     "accept-language": "en-US,en;q=0.5",
//     "content-type": "application/json",
//     "priority": "u=1, i",
//     "sec-ch-ua": "\"Chromium\";v=\"140\", \"Not=A?Brand\";v=\"24\", \"Brave\";v=\"140\"",
//     "sec-ch-ua-mobile": "?0",
//     "sec-ch-ua-platform": "\"Linux\"",
//     "sec-fetch-dest": "empty",
//     "sec-fetch-mode": "cors",
//     "sec-fetch-site": "same-origin",
//     "sec-gpc": "1",
//     "cookie": "session_id=ced7318b-3af6-4a84-b936-8850064c9295; refresh_token=Mb05gkfMGVNI1kGzHALsvLwhGO8ehVlhO6C952kAURY%3D",
//     "Referer": "https://deysis.deu.edu.tr/ogrenci"
//   },
//   "body": "{}",
//   "method": "POST"
// });

use reqwest::Client;

pub async fn refresh_token(client: &Client) -> bool {
    let response = client.post("https://deysis.deu.edu.tr/api/refresh-token").send().await;
    match response {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}