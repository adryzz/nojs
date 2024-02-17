mod about;
mod search;
mod timeline;
mod utils;

use axum::{routing::get, Router};
use megalodon::{entities::Instance, Megalodon};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server...");

    match run().await {
        Ok(_) => tracing::info!("Program exited successfully."),
        Err(e) => tracing::error!("Error: {}", e),
    }
}

async fn run() -> anyhow::Result<()> {
    let client: Client = megalodon::generator(
        megalodon::SNS::Mastodon,
        String::from(""),
        Some("".into()),
        Some("nojs fedi frontend;".into()),
    );
    let instance = client.get_instance().await?.json;

    let state = Arc::new(ClientState { client, instance });

    let app = Router::new()
        .route("/", get(timeline::home))
        .route("/home", get(timeline::home))
        //.route("/home/:homeserver", get(root))
        .route("/federation", get(timeline::federation))
        //.route("/:user", get(root))
        //.route("/object/:id", get(root))
        //.route("/search", get(search))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Listening on http://{}...", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

pub struct ClientState {
    pub client: Client,
    pub instance: Instance,
}

type Client = Box<dyn Megalodon + Sync + Send>;
