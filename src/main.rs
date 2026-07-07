use crate::app::App;

mod app;
mod models;
mod routes;
mod auth;
mod error;
mod repository;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}

