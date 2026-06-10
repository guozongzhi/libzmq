// SPDX-License-Identifier: MPL-2.0

/// First socket families targeted by the inproc migration slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SocketMigrationFamily {
    Pair,
    ReqRep,
    PubSub,
    DealerRouter,
    PushPull,
}

pub const INPROC_FIRST_FAMILIES: &[SocketMigrationFamily] = &[
    SocketMigrationFamily::Pair,
    SocketMigrationFamily::ReqRep,
    SocketMigrationFamily::PubSub,
    SocketMigrationFamily::DealerRouter,
    SocketMigrationFamily::PushPull,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inproc_slice_starts_with_core_socket_families() {
        assert!(INPROC_FIRST_FAMILIES.contains(&SocketMigrationFamily::Pair));
        assert!(INPROC_FIRST_FAMILIES.contains(&SocketMigrationFamily::ReqRep));
    }
}
