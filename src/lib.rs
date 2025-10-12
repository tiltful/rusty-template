//! # Rusty Template
//!
//! This crate provides basic arithmetic operations as a template for Rust projects.
//!
//! ## Features
//! - Safe Rust (forbids unsafe code)
//! - Strict linting rules
//! - Example function: `add` for summing two `u64` numbers
//!
//! ## Usage
//! ```rust
//! use rusty_template::add;
//! let sum = add(2, 3);
//! assert_eq!(sum, 5);
//! ```

#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies
)]
#![warn(
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::similar_names,
    clippy::struct_excessive_bools
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/<USERNAME>/<REPO>/main/assets/images/logo.png"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/<USERNAME>/<REPO>/main/assets/images/favicon.png"
)]

/// Adds two `u64` numbers and returns the result.
///
/// # Arguments
///
/// * `left` - The first operand.
/// * `right` - The second operand.
///
/// # Returns
///
/// The sum of `left` and `right`.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
