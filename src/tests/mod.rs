mod inline_array;
mod ring_buffer;
mod sized_chunk;
mod sparse_chunk;

pub fn action_count() -> usize {
    std::env::var("ACTION_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100)
}
