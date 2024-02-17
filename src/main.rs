mod about;
mod search;
mod timeline;
mod utils;
pub mod thread;

use axum::{routing::get, Router};
use megalodon::{entities::Instance, Megalodon};
use serde::{Deserialize, Serialize};
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
    let config = parse_config().await?;

    let client: Client = megalodon::generator(
        megalodon::SNS::Mastodon,
        format!("https://{}", &config.instance),
        Some(config.token.clone()),
        Some(config.user_agent.clone()),
    );
    let instance = client.get_instance().await?.json;

    let state = Arc::new(ClientState {
        client,
        instance,
        config,
    });

    let app = Router::new()
        .route("/", get(timeline::home))
        .route("/home", get(timeline::home))
        //.route("/home/:homeserver", get(root))
        .route("/federation", get(timeline::federation))
        //.route("/:user", get(root))
        .route("/object/:id", get(thread::thread))
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
    pub config: Config,
}

type Client = Box<dyn Megalodon + Sync + Send>;

pub async fn parse_config() -> anyhow::Result<Config> {
    let config_text = tokio::fs::read_to_string("config.toml").await?;
    let config = toml::from_str::<Config>(&config_text)?;

    Ok(config)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub instance: String,
    pub token: String,
    pub user_agent: String,
}
