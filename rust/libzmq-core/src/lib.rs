// SPDX-License-Identifier: MPL-2.0
//! libzmq Rust core scaffolding.
//!
//! This crate intentionally starts with small, non-exported state machines so
//! the build and test pipeline can be validated before any public C ABI symbol
//! is replaced.

pub mod atomic_counter;
pub mod context;
pub mod message;
pub mod poller;
pub mod socket;

/// Current Rust rewrite scaffold version.
pub const RUST_REWRITE_PROBE_VERSION: u32 = 1;
