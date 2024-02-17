use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};

use crate::ClientState;

#[axum::debug_handler]
pub async fn media(
    Path(id): Path<String>,
    State(state): State<Arc<ClientState>>,
) -> Result<Response, StatusCode> {
    match state.webclient.get(id).send().await {
        Ok(response) => {
            let builder = Response::builder().status(response.status().as_u16());

            builder
                .body(Body::from_stream(response.bytes_stream()))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
