use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use megalodon::{
    entities::{Context, Status},
    megalodon::GetStatusContextInputOptions,
};
use serde::Deserialize;

use crate::{
    utils::{self, Err},
    ClientState,
};

#[derive(Template)]
#[template(path = "thread.html")]
pub struct ThreadTemplate {
    instance: String,
    instance_name: String,
    context: Option<Context>,
    status: Status,
    first_id: Option<String>,
    last_id: Option<String>,
}

#[axum::debug_handler]
pub async fn thread(
    Query(params): Query<ThreadParams>,
    State(state): State<Arc<ClientState>>,
    Path(id): Path<String>,
) -> Result<ThreadTemplate, StatusCode> {
    let status = state.client.get_status(id.clone()).await.to_code()?.json;

    if status.replies_count > 0 {
        let opts = GetStatusContextInputOptions {
            limit: Some(20),
            since_id: params.after,
            max_id: params.before,
        };

        let context = state
            .client
            .get_status_context(id, Some(&opts))
            .await
            .to_code()?
            .json;

        let first_id = context.ancestors.first().map(|f| f.id.clone());
        let last_id = context.descendants.last().map(|f| f.id.clone());

        Ok(ThreadTemplate {
            instance: utils::remove_protocol(state.config.instance.clone()),
            instance_name: state.instance.title.clone(),
            context: Some(context),
            first_id,
            last_id,
            status,
        })
    } else {
        Ok(ThreadTemplate {
            instance: utils::remove_protocol(state.config.instance.clone()),
            instance_name: state.instance.title.clone(),
            context: None,
            first_id: None,
            last_id: None,
            status,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThreadParams {
    after: Option<String>,
    before: Option<String>,
}
