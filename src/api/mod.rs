mod handlers;
mod routes;
mod state;

use axum::Router;
use axum::{error_handling::HandleErrorLayer, http::StatusCode};
pub use state::AppState;
use axum_server::Server;
use log::info;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub async fn serve(state: Arc<AppState>, port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .merge(routes::sessions_routes())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_| async {
                    StatusCode::INTERNAL_SERVER_ERROR
                }))
                .timeout(Duration::from_secs(30))
                .layer(TraceLayer::new_for_http()),
        );

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting server on {}", addr);

    Server::bind(addr).serve(app.into_make_service()).await?;

    Ok(())
}
