{% if architecture == "modular" %}
//! Shared application state.

use std::time::Instant;

/// Dependencies and process-wide state shared by request handlers.
#[derive(Clone, Debug)]
pub struct AppState {
    started_at: Instant,
}

impl Default for AppState {
    fn default() -> Self {
        Self { started_at: Instant::now() }
    }
}

impl AppState {
    /// Returns the process uptime in whole seconds.
    #[must_use]
    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}
{% elsif architecture == "ddd" %}
//! Shared application state.

use std::sync::Arc;

use crate::{application::Greet, infrastructure::StaticGreetingRepository};

/// Cloneable state shared by the application and its adapters.
#[derive(Clone, Debug)]
pub struct AppState {
    /// Greeting use case, shared safely between request handlers.
    pub greet: Arc<Greet<StaticGreetingRepository>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { greet: Arc::new(Greet::new(StaticGreetingRepository)) }
    }
}
{% endif %}
