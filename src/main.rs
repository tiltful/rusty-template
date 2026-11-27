{% if crate_type == "bin" %}
//! # {{project-name}}
//! {{description}}

#![forbid(unsafe_code)]

{% if use_tracing %}
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_error::ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}
{% endif %}
{% if use_axum %}
use axum::{routing::get, Router};
use tower_http::compression::CompressionLayer;
{% if use_tracing %}
use tower_http::trace::TraceLayer;
{% endif %}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut terminate = tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )
        .expect("failed to install SIGTERM handler");

        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                if let Err(error) = result {
                    eprintln!("failed to listen for Ctrl+C: {error}");
                }
            }
            _ = terminate.recv() => {}
        }
    }

    #[cfg(not(unix))]
    {
        if let Err(error) = tokio::signal::ctrl_c().await {
            eprintln!("failed to listen for Ctrl+C: {error}");
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {% if use_tracing %}
    init_tracing();
    {% endif %}

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(CompressionLayer::new());
    {% if use_tracing %}
    let app = app.layer(TraceLayer::new_for_http());
    {% endif %}
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_owned())
        .parse::<u16>()?;
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    {% if use_tracing %}
    tracing::info!("HTTP server listening on {}", listener.local_addr()?);
    {% endif %}
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
{% elsif use_tokio or use_sqlx or use_redis or use_reqwest %}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {% if use_tracing %}
    init_tracing();
    tracing::info!("Application started");
    {% endif %}
    Ok(())
}
{% else %}
fn main() {
    {% if use_tracing %}
    init_tracing();
    tracing::info!("Application started");
    {% else %}
    println!("Hello from {{project-name}}!");
    {% endif %}
}
{% endif %}
{% endif %}
