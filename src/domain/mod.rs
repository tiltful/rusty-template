//! Domain types and ports.

mod greeting;

pub use greeting::{Greeting, GreetingError};

/// A domain port for retrieving the application's greeting.
pub trait GreetingRepository: Send + Sync {
    /// Loads the greeting value.
    fn greeting(&self) -> Result<Greeting, GreetingError>;
}
