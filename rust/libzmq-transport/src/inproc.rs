// SPDX-License-Identifier: MPL-2.0

/// The inproc transport is the first full Rust send/receive path target.
pub const INPROC_SCHEME: &str = "inproc";

#[cfg(test)]
mod tests {
    use super::INPROC_SCHEME;

    #[test]
    fn inproc_scheme_is_first_transport_anchor() {
        assert_eq!(INPROC_SCHEME, "inproc");
    }
}
