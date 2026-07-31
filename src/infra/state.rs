//! Translation between `BatchState` and its persisted representation.
//! Kept in `infra` (not `domain`) since it's a storage-adapter concern.

use crate::domain::BatchState;

pub fn state_to_str(state: BatchState) -> &'static str {
    match state {
        BatchState::Purchased => "Purchased",
        BatchState::Magnetized => "Magnetized",
        BatchState::Cut => "Cut",
        BatchState::Ready => "Ready",
        BatchState::Sold => "Sold",
    }
}
