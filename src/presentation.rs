//! Presentation-layer response mapping.

use crate::state::AppState;

/// Builds a response without exposing infrastructure details.
#[must_use]
pub fn greeting(state: &AppState) -> String {
    {% if architecture == "modular" %}
    format!("{} Uptime: {}s", crate::app::greeting(), state.uptime_seconds())
    {% elsif architecture == "ddd" %}
    match state.greet.execute() {
        Ok(greeting) => greeting.as_str().to_owned(),
        Err(error) => format!("Greeting failed: {error}"),
    }
    {% endif %}
}
