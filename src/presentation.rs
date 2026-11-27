//! Presentation-layer response mapping.

use crate::state::AppState;

/// Builds a response without exposing infrastructure details.
#[must_use]
pub fn greeting(state: &AppState) -> String {
    {% if architecture == "modular" %}
    format!("{} Uptime: {}s", crate::application::greeting(), state.uptime_seconds())
    {% elsif architecture == "ddd" %}
    state.greet.execute().as_str().to_owned()
    {% endif %}
}
