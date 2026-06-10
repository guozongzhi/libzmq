// SPDX-License-Identifier: MPL-2.0

/// Readiness classes the Rust poller must preserve at the C ABI boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PollTargetKind {
    Socket,
    FileDescriptor,
}

#[cfg(test)]
mod tests {
    use super::PollTargetKind;

    #[test]
    fn poller_plan_keeps_socket_and_fd_targets() {
        assert_ne!(PollTargetKind::Socket, PollTargetKind::FileDescriptor);
    }
}
