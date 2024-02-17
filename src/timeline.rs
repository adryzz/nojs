use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use megalodon::{
    entities::Status,
    megalodon::{GetHomeTimelineInputOptions, GetPublicTimelineInputOptions},
};
use serde::Deserialize;

use crate::{
    utils::{self, Err},
    ClientState,
};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    instance: String,
    instance_name: String,
    statuses: Vec<Status>,
    first_id: Option<String>,
    last_id: Option<String>,
}

#[axum::debug_handler]
pub async fn home(
    Query(params): Query<TimelineParams>,
    State(state): State<Arc<ClientState>>,
) -> Result<HomeTemplate, StatusCode> {
    let opts = GetHomeTimelineInputOptions {
        limit: Some(20),
        since_id: params.after,
        max_id: params.before,
        ..Default::default()
    };
    let statuses = state
        .client
        .get_home_timeline(Some(&opts))
        .await
        .to_code()?
        .json;
    let first_id = statuses.first().map(|f| f.id.clone());
    let last_id = statuses.last().map(|f| f.id.clone());
    Ok(HomeTemplate {
        instance: utils::remove_protocol(state.config.instance.clone()),
        instance_name: state.instance.title.clone(),
        statuses,
        first_id,
        last_id,
    })
}

#[axum::debug_handler]
pub async fn federation(
    Query(params): Query<TimelineParams>,
    State(state): State<Arc<ClientState>>,
) -> Result<HomeTemplate, StatusCode> {
    let opts = GetPublicTimelineInputOptions {
        limit: Some(20),
        since_id: params.after,
        max_id: params.before,
        ..Default::default()
    };

    let statuses = state
        .client
        .get_public_timeline(Some(&opts))
        .await
        .to_code()?
        .json;
    let first_id = statuses.first().map(|f| f.id.clone());
    let last_id = statuses.last().map(|f| f.id.clone());
    Ok(HomeTemplate {
        instance: utils::remove_protocol(state.config.instance.clone()),
        instance_name: state.instance.title.clone(),
        statuses,
        first_id,
        last_id,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct TimelineParams {
    after: Option<String>,
    before: Option<String>,
}
