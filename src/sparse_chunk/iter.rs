use bitmaps::{Bitmap, Bits, Iter as BitmapIter};

use super::SparseChunk;
use crate::types::ChunkLength;

/// An iterator over references to the elements of a `SparseChunk`.
pub struct Iter<'a, A, N: Bits + ChunkLength<A>> {
    pub(crate) indices: BitmapIter<'a, N>,
    pub(crate) chunk: &'a SparseChunk<A, N>,
}

impl<'a, A, N: Bits + ChunkLength<A>> Iterator for Iter<'a, A, N> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|index| &self.chunk.values()[index])
    }
}

/// An iterator over mutable references to the elements of a `SparseChunk`.
pub struct IterMut<'a, A, N: Bits + ChunkLength<A>> {
    pub(crate) bitmap: Bitmap<N>,
    pub(crate) chunk: &'a mut SparseChunk<A, N>,
}

impl<'a, A, N: Bits + ChunkLength<A>> Iterator for IterMut<'a, A, N> {
    type Item = &'a mut A;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.bitmap.first_index() {
            self.bitmap.set(index, false);
            unsafe {
                let p: *mut A = &mut self.chunk.values_mut()[index];
                Some(&mut *p)
            }
        } else {
            None
        }
    }
}

/// A draining iterator over the elements of a `SparseChunk`.
///
/// "Draining" means that as the iterator yields each element, it's removed from
/// the `SparseChunk`. When the iterator terminates, the chunk will be empty.
pub struct Drain<A, N: Bits + ChunkLength<A>> {
    pub(crate) chunk: SparseChunk<A, N>,
}

impl<'a, A, N: Bits + ChunkLength<A>> Iterator for Drain<A, N> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunk.pop()
    }
}
