use anyhow::Result;
use subnet_dpn_lp_backend::APP_CONFIG;

mod api;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    // Load the configuration file
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(&APP_CONFIG.log_level));


    // Initialize PostgreSQL connection
    let db_pool = db::create_pool(&APP_CONFIG.database_url).await?;

    // TODO: Initialize Redis connection

    // TODO: Create WebSocket broadcast channel
    // let (tx, _) = broadcast::channel(100);

    // Create shared application state
    let state = api::AppState::new(db_pool);

    // Start the server
    api::serve(state, APP_CONFIG.port).await?;


    Ok(())
}
