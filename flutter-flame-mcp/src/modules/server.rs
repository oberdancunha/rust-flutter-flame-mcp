use crate::modules::features::Features;
use anyhow::Result;
use rmcp::transport::{
    StreamableHttpService, streamable_http_server::session::local::LocalSessionManager,
};
use tracing_subscriber::{
    EnvFilter,
    {fmt::layer, layer::SubscriberExt, util::SubscriberInitExt},
};

#[derive(Debug)]
pub struct Server {}

impl Server {
    pub async fn init() -> Result<()> {
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".to_string().into()))
            .with(layer())
            .init();

        let service = StreamableHttpService::new(
            || Ok(Features::new()),
            LocalSessionManager::default().into(),
            Default::default(),
        );

        let router = axum::Router::new().nest_service("/mcp", service);
        let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
        axum::serve(tcp_listener, router)
            .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
            .await?;

        Ok(())
    }
}
