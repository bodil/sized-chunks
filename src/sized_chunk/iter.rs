use core::iter::FusedIterator;

use super::Chunk;

/// A consuming iterator over the elements of a `Chunk`.
pub struct Iter<A, const N: usize> {
    pub(crate) chunk: Chunk<A, N>,
}

impl<A, const N: usize> Iterator for Iter<A, N> {
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            None
        } else {
            Some(self.chunk.pop_front())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.chunk.len(), Some(self.chunk.len()))
    }
}

impl<A, const N: usize> DoubleEndedIterator for Iter<A, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            None
        } else {
            Some(self.chunk.pop_back())
        }
    }
}

impl<A, const N: usize> ExactSizeIterator for Iter<A, N> {}

impl<A, const N: usize> FusedIterator for Iter<A, N> {}

/// A draining iterator over the elements of a `Chunk`.
///
/// "Draining" means that as the iterator yields each element, it's removed from
/// the `Chunk`. When the iterator terminates, the chunk will be empty. This is
/// different from the consuming iterator `Iter` in that `Iter` will take
/// ownership of the `Chunk` and discard it when you're done iterating, while
/// `Drain` leaves you still owning the drained `Chunk`.
pub struct Drain<'a, A, const N: usize> {
    pub(crate) chunk: &'a mut Chunk<A, N>,
}

impl<'a, A, const N: usize> Iterator for Drain<'a, A, N>
where
    A: 'a,
{
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            None
        } else {
            Some(self.chunk.pop_front())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.chunk.len(), Some(self.chunk.len()))
    }
}

impl<'a, A, const N: usize> DoubleEndedIterator for Drain<'a, A, N>
where
    A: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            None
        } else {
            Some(self.chunk.pop_back())
        }
    }
}

impl<'a, A, const N: usize> ExactSizeIterator for Drain<'a, A, N> where A: 'a {}

impl<'a, A, const N: usize> FusedIterator for Drain<'a, A, N> where A: 'a {}
