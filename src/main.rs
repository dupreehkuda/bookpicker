mod bot;
mod models;
mod repository;
mod service;
mod err;

#[tokio::main]
async fn main() {
    bot::run().await;
}
