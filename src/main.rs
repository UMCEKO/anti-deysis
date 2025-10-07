use crate::app::App;
use log::logger;

pub mod utils;
mod app;
pub mod requests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    logger();
    let mut app = App::init().await;
    app.start().await?;
    Ok(())
}