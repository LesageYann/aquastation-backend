mod usecases;
mod data_sources;
mod model;
mod templates;

mod api;

use std::net::SocketAddr;
use dotenv::dotenv;
use crate::api::app_state::AppState;
use crate::api::server::Server;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").map(|str| str.parse::<u16>().unwrap_or(3000)).unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let state = AppState::new().await;

    Server::new(addr,state)
        .start()
        .await;
}

