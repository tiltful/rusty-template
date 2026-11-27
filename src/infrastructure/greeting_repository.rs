//! In-memory adapter for the greeting repository.

use crate::domain::{Greeting, GreetingError, GreetingRepository};

/// Default in-memory greeting adapter.
#[derive(Clone, Copy, Debug, Default)]
pub struct StaticGreetingRepository;

impl GreetingRepository for StaticGreetingRepository {
    fn greeting(&self) -> Result<Greeting, GreetingError> {
        // This is a static, known-valid value.
        Greeting::new("Hello from the DDD application!")
    }
}
