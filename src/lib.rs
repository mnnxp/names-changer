// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! # Overview
//! Taking data as str.
//! This crate `#[names_changer]` provides trait method `.camel_to_snake()` that convert a names from camel case to snake case.
//! The trait searches for words matching the pattern and converts them to snake case.
//!
//! # Getting Started
//!
//! First of all you have to add this dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! names-changer = "0.2.1"
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
//!         let change_content = content.camel_to_snake();
//!
//!         assert_eq!("TABLE client_tokens_ref IS 'text';", change_content)
//!     }
//! }
//! ```

mod names_changer;

pub use self::names_changer::NamesChanger;
