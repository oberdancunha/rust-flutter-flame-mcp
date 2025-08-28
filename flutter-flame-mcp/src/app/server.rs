use anyhow::Result;
use rmcp::transport::{
    StreamableHttpService, streamable_http_server::session::local::LocalSessionManager,
};
use tracing_subscriber::{
    EnvFilter,
    {fmt::layer, layer::SubscriberExt, util::SubscriberInitExt},
};

use crate::{MCP_ADDRESS, MCP_ENTRY_POINT, MCP_PORT, presentation::routes::Routes};

#[derive(Debug)]
pub struct Server {}

impl Server {
    pub async fn init() -> Result<()> {
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".to_string().into()))
            .with(layer())
            .init();

        let service = StreamableHttpService::new(
            || Ok(Routes::new()),
            LocalSessionManager::default().into(),
            Default::default(),
        );

        let router = axum::Router::new().nest_service(*MCP_ENTRY_POINT, service);
        let tcp_listener =
            tokio::net::TcpListener::bind(format!("{}:{}", *MCP_ADDRESS, *MCP_PORT)).await?;
        axum::serve(tcp_listener, router)
            .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
            .await?;

        Ok(())
    }
}
