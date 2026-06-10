// SPDX-License-Identifier: MPL-2.0

/// ZMTP versions that must remain wire-compatible during the rewrite.
pub const SUPPORTED_ZMTP_VERSIONS: &[(u8, u8)] = &[(1, 0), (2, 0), (3, 0), (3, 1)];

#[cfg(test)]
mod tests {
    use super::SUPPORTED_ZMTP_VERSIONS;

    #[test]
    fn zmtp_31_remains_in_scope() {
        assert!(SUPPORTED_ZMTP_VERSIONS.contains(&(3, 1)));
    }
}
