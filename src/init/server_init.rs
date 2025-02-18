use std::{net::SocketAddr, sync::Arc, time::Duration};

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use tracing::info;

use crate::{
    init::config::EmailConfig, jobs::job_funcs::init_scheduler::task_init,
    routers::main_router::build_router,
};

use super::{config::DbConfig, state::ServerState};

pub async fn server_init_proc(start: tokio::time::Instant) -> anyhow::Result<()> {
    let num_cores: u32 = num_cpus::get_physical() as u32;

    let db_config = DbConfig::from_env()
        .map_err(|e| anyhow::anyhow!("Failed to get DB config from environment: {}", e))?
        .to_url()
        .map_err(|e| anyhow::anyhow!("Failed to convert DB config to URL: {}", e))?;
    info!("Loaded DB configuration.");

    let pool_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_config);

    let pool = Pool::builder()
        .min_idle(Some(num_cores))
        .max_size(num_cores * 10u32)
        .connection_timeout(Duration::from_secs(2))
        .build(pool_config)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to build connection pool: {}", e))?;

    info!(
        "Connection pool built with {} connections. Will scale to {} connections.",
        num_cores,
        num_cores * 10u32
    );

    let app_name_version: String = std::env::var("APP_NAME_VERSION")
        .map_err(|e| anyhow::anyhow!("Failed to load APP_NAME_VAR from .env: {}", e))?;

    let email_config = EmailConfig::from_env()
        .map_err(|e| anyhow::anyhow!("Failed to load email configs from .env: {}", e))?;
    let email_creds: Credentials = email_config.to_creds();
    let email_client: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&email_config.get_url())?
            .credentials(email_creds)
            .build();

    info!(
        "Email client configured; relay at {}",
        email_config.get_url()
    );

    let state = Arc::new(
        ServerState::builder()
            .app_name_version(app_name_version)
            .pool(pool)
            .server_start_time(start)
            .email_client(email_client)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build ServerState: {}", e))?,
    );
    info!("ServerState initialized.");

    // initialize scheduled jobs manager
    task_init(state.clone()).await?;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind TCP listener: {}", e))?;
    info!("Listening to Port 3000...");

    info!(
        "Initialization complete in {:?}. Starting server now...",
        start.elapsed()
    );

    axum::serve(
        listener,
        build_router(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Failed to serve application: {}", e))?;

    Ok(())
}
