// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # Sized Chunks
//!
//! This crate contains three fixed size low level array like data structures,
//! primarily intended for use in [immutable.rs], but fully supported as a
//! standalone crate.
//!
//! Their sizing information is encoded in the type using the
//! [`typenum`][typenum] crate, which you may want to take a look at before
//! reading on, but usually all you need to know about it is that it provides
//! types `U1` to `U128` to represent numbers, which the data types take as type
//! parameters, eg. `SparseChunk<A, U32>` would give you a sparse array with
//! room for 32 elements of type `A`. You can also omit the size, as they all
//! default to a size of 64, so `SparseChunk<A>` would be a sparse array with a
//! capacity of 64.
//!
//! All data structures always allocate the same amount of space, as determined
//! by their capacity, regardless of how many elements they contain, and when
//! they run out of space, they will panic.
//!
//! ## Data Structures
//!
//! | Type | Description | Push | Pop | Deref to `&[A]` |
//! | --- | --- | --- | --- | --- |
//! | [`Chunk`][Chunk] | Contiguous array | O(1)/O(n) | O(1) | Yes |
//! | [`RingBuffer`][RingBuffer] | Non-contiguous array | O(1) | O(1) | No |
//! | [`SparseChunk`][SparseChunk] | Sparse array | N/A | N/A | No |
//!
//! The [`Chunk`][Chunk] and [`RingBuffer`][RingBuffer] are very similar in
//! practice, in that they both work like a plain array, except that you can
//! push to either end with some expectation of performance. The difference is
//! that [`RingBuffer`][RingBuffer] always allows you to do this in constant
//! time, but in order to give that guarantee, it doesn't lay out its elements
//! contiguously in memory, which means that you can't dereference it to a slice
//! `&[A]`.
//!
//! [`Chunk`][Chunk], on the other hand, will shift its contents around when
//! necessary to accommodate a push to a full side, but is able to guarantee a
//! contiguous memory layout in this way, so it can always be dereferenced into
//! a slice. Performance wise, repeated pushes to the same side will always run
//! in constant time, but a push to one side followed by a push to the other
//! side will cause the latter to run in linear time if there's no room (which
//! there would only be if you've popped from that side).
//!
//!To choose between them, you can use the following rules:
//! - I only ever want to push to the back: you don't need this crate, try
//!   [`ArrayVec`][ArrayVec].
//! - I need to push to either side but probably not both on the same array: use
//!   [`Chunk`][Chunk].
//! - I need to push to both sides and I don't need slices: use
//!   [`RingBuffer`][RingBuffer].
//! - I need to push to both sides but I do need slices: use [`Chunk`][Chunk].
//!
//! Finally, [`SparseChunk`][SparseChunk] is a more efficient version of
//! `Vec<Option<A>>`: each index is either inhabited or not, but instead of
//! using the `Option` discriminant to decide which is which, it uses a compact
//! bitmap. You can also think of `SparseChunk<A, N>` as a `BTreeMap<usize, A>`
//! where the `usize` must be less than `N`, but without the performance
//! overhead. Its API is also more consistent with a map than an array - there's
//! no push, pop, append, etc, just insert, remove and lookup.
//!
//! [immutable.rs]: https://immutable.rs/
//! [typenum]: https://docs.rs/typenum/
//! [Chunk]: struct.Chunk.html
//! [RingBuffer]: struct.RingBuffer.html
//! [SparseChunk]: struct.SparseChunk.html
//! [ArrayVec]: https://docs.rs/arrayvec/
pub mod bitmap;
pub mod inline_array;
pub mod ring_buffer;
pub mod sized_chunk;
pub mod sparse_chunk;
pub mod types;

#[cfg(test)]
mod tests;

pub use crate::bitmap::Bitmap;
pub use crate::inline_array::InlineArray;
pub use crate::ring_buffer::RingBuffer;
pub use crate::sized_chunk::Chunk;
pub use crate::sparse_chunk::SparseChunk;
