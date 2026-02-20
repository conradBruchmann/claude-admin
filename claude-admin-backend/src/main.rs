mod app;
mod domain;
mod infra;
mod routes;
mod services;

use std::net::SocketAddr;
use tokio::net::TcpListener;

use app::create_app;
use infra::config::Config;
use infra::logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();

    let config = Config::load();
    let app = create_app(config.clone()).await?;

    let host: std::net::IpAddr = config
        .host
        .parse()
        .unwrap_or_else(|_| std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    let addr = SocketAddr::from((host, config.port));
    tracing::info!("ClaudeAdmin backend starting on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut term = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Shutdown: ctrl_c");
            }
            _ = term.recv() => {
                tracing::info!("Shutdown: SIGTERM");
            }
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    }
}
