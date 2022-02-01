// Copyright (c) The diem-devtools Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This crate contains deserializers for machine-readable output generated by
//! [cargo-nextest](https://docs.rs/cargo-nextest).
//!
//! Implemented so far:
//! * ✅ Listing tests
//! * ✅ Semantic exit codes with [`NextestExitCode`]
//!
//! # Examples
//!
//! Get the list of tests in the repository.
//!
//! ```rust,no_run
//! // This example requires `cargo nextest` to be installed.
//!
//! use nextest_metadata::ListCommand;
//!
//! let command = ListCommand::new();
//! let test_list = command.exec().unwrap();
//!
//! // The result is a TestListSummary.
//! println!("{:?}", test_list);
//! ```
#![warn(missing_docs)]

mod errors;
mod exit_codes;
mod test_list;

pub use errors::*;
pub use exit_codes::*;
pub use test_list::*;
