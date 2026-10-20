//! tool_core: every rule of the assignment page.
//!
//! Builder requirement: this crate performs no input or output. It takes
//! values and returns values, so `cargo test` reaches every rule without a
//! browser. `tests/boundary.rs` enforces that, along with the dependency
//! allowlist and the reference ban on floats, hash maps and clocks. The page
//! passes the current date and hour in as text; the core reads no clock.

pub mod api;
pub mod dates;
pub mod glossary;
pub mod schedule;
pub mod text;
pub mod view;
