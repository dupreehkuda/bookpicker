mod bot;
mod err;
mod insights;
mod models;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    bot::run().await;
}
