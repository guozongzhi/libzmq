// SPDX-License-Identifier: MPL-2.0

/// Planned lifecycle states for a Rust-backed ZeroMQ context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextState {
    Running,
    ShuttingDown,
    Terminating,
    Terminated,
}

impl ContextState {
    pub fn shutdown(self) -> Self {
        match self {
            ContextState::Running => ContextState::ShuttingDown,
            other => other,
        }
    }

    pub fn terminate(self) -> Self {
        match self {
            ContextState::Terminated => ContextState::Terminated,
            _ => ContextState::Terminating,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContextState;

    #[test]
    fn context_shutdown_is_idempotent_after_first_transition() {
        assert_eq!(ContextState::Running.shutdown(), ContextState::ShuttingDown);
        assert_eq!(
            ContextState::ShuttingDown.shutdown(),
            ContextState::ShuttingDown
        );
    }

    #[test]
    fn context_terminate_moves_active_states_to_terminating() {
        assert_eq!(ContextState::Running.terminate(), ContextState::Terminating);
        assert_eq!(
            ContextState::ShuttingDown.terminate(),
            ContextState::Terminating
        );
    }
}
