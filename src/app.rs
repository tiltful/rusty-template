//! Application use cases and business workflows.

/// Returns the starter application's greeting.
#[must_use]
pub fn greeting() -> &'static str {
    "Hello from the modular application!"
}
