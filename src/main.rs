mod bot;
mod repository;

#[tokio::main]
async fn main() {
    bot::run().await;
}