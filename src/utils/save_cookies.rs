use reqwest_cookie_store::CookieStoreMutex;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

pub fn save_cookies(cookie_store: Arc<CookieStoreMutex>, cookie_path: String) {
    let file = File::create(cookie_path).expect("Failed to create cookies file");
    let mut writer = BufWriter::new(file);
    cookie_store::serde::json::save(&cookie_store.lock().unwrap(), &mut writer).expect("Failed to save cookies");
}