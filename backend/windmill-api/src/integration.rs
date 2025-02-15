use crate::{db::DB, HTTP_CLIENT};
use axum::{
    body::StreamBody, extract::Query, response::IntoResponse, routing::get, Extension, Router,
};
use windmill_common::{error::Error, utils::query_elems_from_hub};

pub fn global_service() -> Router {
    Router::new().route("/hub/list", get(list_hub_integrations))
}

#[derive(serde::Deserialize)]
struct ListHubIntegrationsQuery {
    kind: Option<String>,
}
async fn list_hub_integrations(
    Query(query): Query<ListHubIntegrationsQuery>,
    Extension(db): Extension<DB>,
) -> impl IntoResponse {
    let mut query_params = vec![];

    if let Some(kind) = query.kind {
        query_params.push(("kind", kind));
    }

    let (status_code, headers, response) = query_elems_from_hub(
        &HTTP_CLIENT,
        "https://hub.windmill.dev/integrations/list",
        Some(query_params),
        &db,
    )
    .await?;
    Ok::<_, Error>((
        status_code,
        headers,
        StreamBody::new(response.bytes_stream()),
    ))
}
