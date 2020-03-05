use std::sync::atomic::{AtomicUsize, Ordering};

mod inline_array;
mod ring_buffer;
mod sized_chunk;
mod sparse_chunk;

pub(crate) fn action_count() -> usize {
    std::env::var("ACTION_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100)
}

pub(crate) struct DropTest<'a> {
    counter: &'a AtomicUsize,
}

impl<'a> DropTest<'a> {
    pub(crate) fn new(counter: &'a AtomicUsize) -> Self {
        counter.fetch_add(1, Ordering::Relaxed);
        DropTest { counter }
    }
}

impl<'a> Drop for DropTest<'a> {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::Relaxed);
    }
}
