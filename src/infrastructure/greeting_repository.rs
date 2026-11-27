//! In-memory adapter for the greeting repository.

use crate::domain::{Greeting, GreetingRepository};

/// Default in-memory greeting adapter.
#[derive(Clone, Copy, Debug, Default)]
pub struct StaticGreetingRepository;

impl GreetingRepository for StaticGreetingRepository {
    fn greeting(&self) -> Greeting {
        // This is a static, known-valid value.
        Greeting::new("Hello from the DDD application!")
            .expect("the built-in greeting is non-empty")
    }
}
