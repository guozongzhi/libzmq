// SPDX-License-Identifier: MPL-2.0

/// Planned ownership state for a future Rust-backed `zmq_msg_t`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageStorageKind {
    Empty,
    Inline,
    Heap,
    ForeignBuffer,
}

/// Minimal message descriptor used by scaffold tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessagePlan {
    storage: MessageStorageKind,
    size: usize,
}

impl MessagePlan {
    pub fn empty() -> Self {
        Self {
            storage: MessageStorageKind::Empty,
            size: 0,
        }
    }

    pub fn sized(size: usize) -> Self {
        Self {
            storage: if size == 0 {
                MessageStorageKind::Empty
            } else {
                MessageStorageKind::Heap
            },
            size,
        }
    }

    pub fn storage(&self) -> MessageStorageKind {
        self.storage
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_message_plan_has_zero_size() {
        let plan = MessagePlan::empty();
        assert_eq!(plan.storage(), MessageStorageKind::Empty);
        assert_eq!(plan.size(), 0);
    }

    #[test]
    fn non_empty_message_plan_uses_heap_until_layout_is_ported() {
        let plan = MessagePlan::sized(8);
        assert_eq!(plan.storage(), MessageStorageKind::Heap);
        assert_eq!(plan.size(), 8);
    }
}
