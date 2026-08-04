use std::marker::PhantomData;

pub struct GhostId<T> {
    value:   i64,
    _entity: PhantomData<fn() -> T>,
}

impl<T> GhostId<T> {
    pub fn new(value: i64) -> Self {
        Self { value, _entity: PhantomData }
    }

    // ── Getters ──────────────────────────────────────────────────────────────
    pub fn value(&self) -> i64 { self.value }
}

impl<T> From<i64> for GhostId<T> {
    fn from(value: i64) -> Self { Self::new(value) }
}
