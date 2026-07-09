use crate::app::App;

mod app;
mod auth;
mod error;
mod models;
mod repository;
mod routes;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
