// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::iter::FusedIterator;
use core::ops::{Add, AddAssign, Sub, SubAssign};

pub(crate) struct RawIndex<const N: usize>(usize);

impl<const N: usize> Clone for RawIndex<N> {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        self.0.into()
    }
}

impl<const N: usize> Copy for RawIndex<N> {}

impl<const N: usize> RawIndex<N> {
    #[inline]
    #[must_use]
    pub(crate) fn to_usize(self) -> usize {
        self.0
    }

    /// Increments the index and returns a copy of the index /before/ incrementing.
    #[inline]
    #[must_use]
    pub(crate) fn inc(&mut self) -> Self {
        let old = *self;
        self.0 = if self.0 == N - 1 { 0 } else { self.0 + 1 };
        old
    }

    /// Decrements the index and returns a copy of the new value.
    #[inline]
    #[must_use]
    pub(crate) fn dec(&mut self) -> Self {
        self.0 = if self.0 == 0 { N - 1 } else { self.0 - 1 };
        *self
    }
}

impl<const N: usize> From<usize> for RawIndex<N> {
    #[inline]
    #[must_use]
    fn from(index: usize) -> Self {
        debug_assert!(index < N);
        RawIndex(index)
    }
}

impl<const N: usize> PartialEq for RawIndex<N> {
    #[inline]
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize> Eq for RawIndex<N> {}

impl<const N: usize> Add for RawIndex<N> {
    type Output = RawIndex<N>;
    #[inline]
    #[must_use]
    fn add(self, other: Self) -> Self::Output {
        self + other.0
    }
}

impl<const N: usize> Add<usize> for RawIndex<N> {
    type Output = RawIndex<N>;
    #[inline]
    #[must_use]
    fn add(self, other: usize) -> Self::Output {
        let mut result = self.0 + other;
        while result >= N {
            result -= N;
        }
        result.into()
    }
}

impl<const N: usize> AddAssign<usize> for RawIndex<N> {
    #[inline]
    fn add_assign(&mut self, other: usize) {
        self.0 += other;
        while self.0 >= N {
            self.0 -= N;
        }
    }
}

impl<const N: usize> Sub for RawIndex<N> {
    type Output = RawIndex<N>;
    #[inline]
    #[must_use]
    fn sub(self, other: Self) -> Self::Output {
        self - other.0
    }
}

impl<const N: usize> Sub<usize> for RawIndex<N> {
    type Output = RawIndex<N>;
    #[inline]
    #[must_use]
    fn sub(self, other: usize) -> Self::Output {
        let mut start = self.0;
        while other > start {
            start += N;
        }
        (start - other).into()
    }
}

impl<const N: usize> SubAssign<usize> for RawIndex<N> {
    #[inline]
    fn sub_assign(&mut self, other: usize) {
        while other > self.0 {
            self.0 += N;
        }
        self.0 -= other;
    }
}

pub(crate) struct IndexIter<const N: usize> {
    pub(crate) remaining: usize,
    pub(crate) left_index: RawIndex<N>,
    pub(crate) right_index: RawIndex<N>,
}

impl<const N: usize> Iterator for IndexIter<N> {
    type Item = RawIndex<N>;
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

impl<const N: usize> DoubleEndedIterator for IndexIter<N> {
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

impl<const N: usize> ExactSizeIterator for IndexIter<N> {}

impl<const N: usize> FusedIterator for IndexIter<N> {}
