use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use megalodon::entities::attachment::AttachmentType;

use crate::{utils::Err, ClientState};

#[axum::debug_handler]
pub async fn media(
    Path(id): Path<String>,
    State(state): State<Arc<ClientState>>,
) -> Result<Response, StatusCode> {
    let media = state.client.get_media(id).await.to_code()?.json;
    
    if let AttachmentType::Image = media.r#type {
        match state.webclient.get(&media.url).send().await {
            Ok(response) => {
                let builder = Response::builder().status(response.status().as_u16());

                builder
                    .body(Body::from_stream(response.bytes_stream()))
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            }
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
