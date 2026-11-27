{% if crate_type == "bin" %}
//! # {{project-name}}
//! {{description}}

#![forbid(unsafe_code)]

{% if use_tracing %}
use tracing_subscriber::EnvFilter;
{% endif %}
{% if use_axum %}
use axum::{routing::get, Router};
use tower_http::compression::CompressionLayer;
{% if use_tracing %}
use tower_http::trace::TraceLayer;
{% endif %}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {% if use_tracing %}
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();
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
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
{% elsif use_tokio %}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {% if use_tracing %}
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();
    tracing::info!("Application started");
    {% endif %}
    Ok(())
}
{% else %}
fn main() {
    {% if use_tracing %}
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();
    tracing::info!("Application started");
    {% else %}
    println!("Hello from {{project-name}}!");
    {% endif %}
}
{% endif %}
{% endif %}
