//! # Overview
//! Taking data as str.
//! This crate provides `#[names_changer]` trait that convert a names from camel case to snake case.
//! The searches for words matching the pattern, to converts them to snake case.
//!
//! # Getting Started
//!
//! First of all you have to add this dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! names-changer = "0.1.0"
//! ```
//!
//! Additionally, you have to import the procedural macro with `use` statement:
//!
//! ```rust
//! use names_changer::NamesChanger;
//! ```
//!
//! # Example usage:
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use names_changer::NamesChanger;
//!
//!     // Not needed for this example, but useful in general
//!     use super::*;
//!
//!     #[test]
//!     fn test_name_change() {
//!         let content = "TABLE ClientTokensRef IS 'text';";
//!         let change_content = content.camel_to_snack();
//!
//!         assert_eq!("TABLE client_tokens_ref IS 'text';", change_content)
//!     }
//! }
//! ```

mod names_changer;

pub use self::names_changer::NamesChanger;
