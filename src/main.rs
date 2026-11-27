{% if crate_type == "bin" %}
//! # {{project-name}}
//! {{description}}

#![forbid(unsafe_code)]

{% if architecture == "modular" %}
mod app;
mod presentation;
mod state;
{% elsif architecture == "ddd" %}
mod application;
mod domain;
mod infrastructure;
mod presentation;
mod state;
{% endif %}

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
{% if use_config or use_sqlx or use_redis or use_reqwest %}
fn init_integrations() -> anyhow::Result<()> {
    {% if use_config %}
    dotenvy::dotenv().ok();
    let _settings = config::Config::builder()
        .add_source(config::Environment::default())
        .build()?;
    {% endif %}
    {% if use_sqlx %}
    // Configure the pool here and connect once DATABASE_URL is available.
    let _database_pool_options = sqlx::postgres::PgPoolOptions::new();
    let _now = time::OffsetDateTime::now_utc();
    let _request_id = uuid::Uuid::now_v7();
    {% endif %}
    {% if use_redis %}
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1/".to_owned());
    let _redis_client = redis::Client::open(redis_url)?;
    {% endif %}
    {% if use_reqwest %}
    let _http_client = reqwest::Client::builder().build()?;
    {% endif %}
    Ok(())
}
{% endif %}
{% if use_axum %}
{% if architecture == "minimal" %}
use axum::{routing::get, Router};
{% else %}
use axum::{extract::State, routing::get, Router};
{% endif %}
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
    {% if use_config or use_sqlx or use_redis or use_reqwest %}
    init_integrations()?;
    {% endif %}

{% if architecture == "minimal" %}
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(CompressionLayer::new());
{% else %}
    let state = state::AppState::default();
    let app = Router::new()
        .route(
            "/",
            get(|State(state): State<state::AppState>| async move {
                presentation::greeting(&state)
            }),
        )
        .with_state(state)
        .layer(CompressionLayer::new());
{% endif %}
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
    {% if use_config or use_sqlx or use_redis or use_reqwest %}
    init_integrations()?;
    {% endif %}
    {% if architecture != "minimal" %}
    run_architecture();
    {% endif %}
    Ok(())
}
{% else %}
{% if use_config %}
fn main() -> anyhow::Result<()> {
    {% if use_tracing %}
    init_tracing();
    {% endif %}
    init_integrations()?;
    {% if architecture != "minimal" %}
    run_architecture();
    {% elsif use_tracing %}
    tracing::info!("Application started");
    {% else %}
    println!("Hello from {{project-name}}!");
    {% endif %}
    Ok(())
}
{% else %}
fn main() {
    {% if architecture != "minimal" %}
    run_architecture();
    {% elsif use_tracing %}
    init_tracing();
    tracing::info!("Application started");
    {% else %}
    println!("Hello from {{project-name}}!");
    {% endif %}
}
{% endif %}
{% endif %}
{% endif %}

{% if architecture == "modular" or architecture == "ddd" %}
fn run_architecture() {
    let state = state::AppState::default();
    println!("{}", presentation::greeting(&state));
}
{% endif %}
