// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::types::ChunkLength;

pub struct RawIndex<A, N: ChunkLength<A>>(usize, PhantomData<(A, N)>);

impl<A, N: ChunkLength<A>> Clone for RawIndex<A, N> {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        self.0.into()
    }
}

impl<A, N> Copy for RawIndex<A, N> where N: ChunkLength<A> {}

impl<A, N: ChunkLength<A>> RawIndex<A, N> {
    #[inline]
    #[must_use]
    pub fn to_usize(self) -> usize {
        self.0
    }

    /// Increments the index and returns a copy of the index /before/ incrementing.
    #[inline]
    #[must_use]
    pub fn inc(&mut self) -> Self {
        let old = *self;
        self.0 = if self.0 == N::USIZE - 1 {
            0
        } else {
            self.0 + 1
        };
        old
    }

    /// Decrements the index and returns a copy of the new value.
    #[inline]
    #[must_use]
    pub fn dec(&mut self) -> Self {
        self.0 = if self.0 == 0 {
            N::USIZE - 1
        } else {
            self.0 - 1
        };
        *self
    }
}

impl<A, N: ChunkLength<A>> From<usize> for RawIndex<A, N> {
    #[inline]
    #[must_use]
    fn from(index: usize) -> Self {
        debug_assert!(index < N::USIZE);
        RawIndex(index, PhantomData)
    }
}

impl<A, N: ChunkLength<A>> PartialEq for RawIndex<A, N> {
    #[inline]
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<A, N: ChunkLength<A>> Eq for RawIndex<A, N> {}

impl<A, N: ChunkLength<A>> Add for RawIndex<A, N> {
    type Output = RawIndex<A, N>;
    #[inline]
    #[must_use]
    fn add(self, other: Self) -> Self::Output {
        self + other.0
    }
}

impl<A, N: ChunkLength<A>> Add<usize> for RawIndex<A, N> {
    type Output = RawIndex<A, N>;
    #[inline]
    #[must_use]
    fn add(self, other: usize) -> Self::Output {
        let mut result = self.0 + other;
        while result >= N::USIZE {
            result -= N::USIZE;
        }
        result.into()
    }
}

impl<A, N: ChunkLength<A>> AddAssign<usize> for RawIndex<A, N> {
    #[inline]
    fn add_assign(&mut self, other: usize) {
        self.0 += other;
        while self.0 >= N::USIZE {
            self.0 -= N::USIZE;
        }
    }
}

impl<A, N: ChunkLength<A>> Sub for RawIndex<A, N> {
    type Output = RawIndex<A, N>;
    #[inline]
    #[must_use]
    fn sub(self, other: Self) -> Self::Output {
        self - other.0
    }
}

impl<A, N: ChunkLength<A>> Sub<usize> for RawIndex<A, N> {
    type Output = RawIndex<A, N>;
    #[inline]
    #[must_use]
    fn sub(self, other: usize) -> Self::Output {
        let mut start = self.0;
        while other > start {
            start += N::USIZE;
        }
        (start - other).into()
    }
}

impl<A, N: ChunkLength<A>> SubAssign<usize> for RawIndex<A, N> {
    #[inline]
    fn sub_assign(&mut self, other: usize) {
        while other > self.0 {
            self.0 += N::USIZE;
        }
        self.0 -= other;
    }
}

pub struct IndexIter<A, N: ChunkLength<A>> {
    pub remaining: usize,
    pub left_index: RawIndex<A, N>,
    pub right_index: RawIndex<A, N>,
}

impl<A, N: ChunkLength<A>> Iterator for IndexIter<A, N> {
    type Item = RawIndex<A, N>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;
            Some(self.left_index.inc())
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<A, N: ChunkLength<A>> DoubleEndedIterator for IndexIter<A, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;
            Some(self.right_index.dec())
        } else {
            None
        }
    }
}

impl<A, N: ChunkLength<A>> ExactSizeIterator for IndexIter<A, N> {}

impl<A, N: ChunkLength<A>> FusedIterator for IndexIter<A, N> {}
