//! Greeting use case.

use crate::domain::{Greeting, GreetingError, GreetingRepository};

/// Application use case that retrieves a greeting through a domain port.
#[derive(Clone, Debug)]
pub struct Greet<R> {
    repository: R,
}

impl<R> Greet<R>
where
    R: GreetingRepository,
{
    /// Creates the use case with its repository dependency.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Executes the use case.
    #[must_use]
    pub fn execute(&self) -> Result<Greeting, GreetingError> {
        self.repository.greeting()
    }
}
