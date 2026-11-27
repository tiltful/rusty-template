{% if crate_type == "lib" %}
//! # {{project-name}}
//! {{description}}

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Returns a friendly greeting.
///
/// # Examples
///
/// ```
/// assert_eq!({{crate_name}}::greeting(), "Hello, world!");
/// ```
pub fn greeting() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_available() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
{% endif %}
