// SPDX-License-Identifier: MPL-2.0

/// Compile-time libzmq version mirrored from `include/zmq.h`.
pub const VERSION_MAJOR: i32 = 4;
pub const VERSION_MINOR: i32 = 3;
pub const VERSION_PATCH: i32 = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

pub fn version() -> Version {
    Version {
        major: VERSION_MAJOR,
        minor: VERSION_MINOR,
        patch: VERSION_PATCH,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_current_header_constants() {
        assert_eq!(version().major, 4);
        assert_eq!(version().minor, 3);
        assert_eq!(version().patch, 6);
    }
}
