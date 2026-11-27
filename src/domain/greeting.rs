//! Domain model for a greeting.

use std::fmt;

/// A validated, non-empty greeting value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Greeting(String);

/// Validation error returned when a greeting is empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GreetingError;

impl fmt::Display for GreetingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a greeting must not be empty")
    }
}

impl std::error::Error for GreetingError {}

impl Greeting {
    /// Creates a greeting after validating its invariant.
    pub fn new(value: impl Into<String>) -> Result<Self, GreetingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(GreetingError);
        }
        Ok(Self(value))
    }

    /// Returns the greeting text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Greeting;

    #[test]
    fn rejects_empty_values() {
        assert!(Greeting::new("  ").is_err());
    }

    #[test]
    fn preserves_valid_values() {
        assert_eq!(Greeting::new("Hello").unwrap().as_str(), "Hello");
    }
}
