mod bot;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    bot::run().await;
}