// SPDX-License-Identifier: MPL-2.0
//! C ABI probe for the staged Rust rewrite.
//!
//! This crate deliberately exports only an internal probe symbol. It does not
//! replace any existing `zmq_*` public ABI entry point yet.

/// Internal probe used by CMake/CI to verify that Rust code can be compiled and
/// linked without colliding with the existing public C ABI.
#[no_mangle]
pub extern "C" fn zmq_rs_version_probe() -> u32 {
    libzmq_core::RUST_REWRITE_PROBE_VERSION
}

#[cfg(test)]
mod tests {
    use super::zmq_rs_version_probe;

    #[test]
    fn probe_reports_scaffold_version() {
        assert_eq!(zmq_rs_version_probe(), 1);
    }
}
