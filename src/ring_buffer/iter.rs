// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::iter::FusedIterator;

use crate::types::ChunkLength;

use super::{index::RawIndex, RingBuffer};

/// A reference iterator over a `RingBuffer`.
pub struct Iter<'a, A, N>
where
    N: ChunkLength<A>,
{
    pub(crate) buffer: &'a RingBuffer<A, N>,
    pub(crate) left_index: RawIndex<A, N>,
    pub(crate) right_index: RawIndex<A, N>,
    pub(crate) remaining: usize,
}

impl<'a, A, N> Iterator for Iter<'a, A, N>
where
    N: ChunkLength<A>,
{
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(unsafe { &*self.buffer.ptr(self.left_index.inc()) })
        }
    }

    #[inline]
    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, A, N> DoubleEndedIterator for Iter<'a, A, N>
where
    N: ChunkLength<A>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(unsafe { &*self.buffer.ptr(self.right_index.dec()) })
        }
    }
}

impl<'a, A, N> ExactSizeIterator for Iter<'a, A, N> where N: ChunkLength<A> {}

impl<'a, A, N> FusedIterator for Iter<'a, A, N> where N: ChunkLength<A> {}

/// A mutable reference iterator over a `RingBuffer`.
pub struct IterMut<'a, A, N>
where
    N: ChunkLength<A>,
{
    pub(crate) buffer: &'a mut RingBuffer<A, N>,
    pub(crate) left_index: RawIndex<A, N>,
    pub(crate) right_index: RawIndex<A, N>,
    pub(crate) remaining: usize,
}

impl<'a, A, N> Iterator for IterMut<'a, A, N>
where
    N: ChunkLength<A>,
{
    type Item = &'a mut A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(unsafe { &mut *self.buffer.mut_ptr(self.left_index.inc()) })
        }
    }

    #[inline]
    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, A, N> DoubleEndedIterator for IterMut<'a, A, N>
where
    N: ChunkLength<A>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(unsafe { &mut *self.buffer.mut_ptr(self.right_index.dec()) })
        }
    }
}

impl<'a, A, N> ExactSizeIterator for IterMut<'a, A, N> where N: ChunkLength<A> {}

impl<'a, A, N> FusedIterator for IterMut<'a, A, N> where N: ChunkLength<A> {}

/// A draining iterator over a `RingBuffer`.
pub struct Drain<'a, A: 'a, N: ChunkLength<A> + 'a> {
    pub(crate) buffer: &'a mut RingBuffer<A, N>,
}

impl<'a, A: 'a, N: ChunkLength<A> + 'a> Iterator for Drain<'a, A, N> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop_front()
    }

    #[inline]
    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.buffer.len(), Some(self.buffer.len()))
    }
}

impl<'a, A: 'a, N: ChunkLength<A> + 'a> DoubleEndedIterator for Drain<'a, A, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.buffer.pop_back()
    }
}

impl<'a, A: 'a, N: ChunkLength<A> + 'a> ExactSizeIterator for Drain<'a, A, N> {}

impl<'a, A: 'a, N: ChunkLength<A> + 'a> FusedIterator for Drain<'a, A, N> {}

/// A consuming iterator over a `RingBuffer`.
pub struct OwnedIter<A, N: ChunkLength<A>> {
    pub(crate) buffer: RingBuffer<A, N>,
}

impl<A, N: ChunkLength<A>> Iterator for OwnedIter<A, N> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop_front()
    }

    #[inline]
    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.buffer.len(), Some(self.buffer.len()))
    }
}

impl<A, N: ChunkLength<A>> DoubleEndedIterator for OwnedIter<A, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.buffer.pop_back()
    }
}

impl<A, N: ChunkLength<A>> ExactSizeIterator for OwnedIter<A, N> {}

impl<A, N: ChunkLength<A>> FusedIterator for OwnedIter<A, N> {}
