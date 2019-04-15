// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A fixed capacity sparse array.
//!
//! See [`Bitmap`](struct.Bitmap.html)

use std::fmt::{Debug, Error, Formatter};

use crate::types::Bits;

/// A compact array of bits.
///
/// The bitmap is stored as a primitive type, so the maximum value of `Size` is
/// currently 128, corresponding to a type of `u128`. The type used to store the
/// bitmap will be the minimum unsigned integer type required to fit the number
/// of bits required, from `u8` to `u128`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate sized_chunks;
/// # extern crate typenum;
/// # use sized_chunks::bitmap::Bitmap;
/// # use typenum::U10;
/// # fn main() {
/// let mut bitmap = Bitmap::<U10>::new();
/// assert_eq!(bitmap.set(5, true), false);
/// assert_eq!(bitmap.set(5, true), true);
/// assert_eq!(bitmap.get(5), true);
/// assert_eq!(bitmap.get(6), false);
/// assert_eq!(bitmap.len(), 1);
/// assert_eq!(bitmap.set(3, true), false);
/// assert_eq!(bitmap.len(), 2);
/// assert_eq!(bitmap.first_index(), Some(3));
/// # }
/// ```
pub struct Bitmap<Size: Bits> {
    data: Size::Store,
}

impl<Size: Bits> Clone for Bitmap<Size> {
    fn clone(&self) -> Self {
        Bitmap { data: self.data }
    }
}

impl<Size: Bits> Copy for Bitmap<Size> {}

impl<Size: Bits> Default for Bitmap<Size> {
    fn default() -> Self {
        Bitmap {
            data: Size::Store::default(),
        }
    }
}

impl<Size: Bits> PartialEq for Bitmap<Size> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<Size: Bits> Debug for Bitmap<Size> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.data.fmt(f)
    }
}

impl<Size: Bits> Bitmap<Size> {
    /// Construct an empty bitmap.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Count the number of `true` bits in the bitmap.
    #[inline]
    pub fn len(self) -> usize {
        Size::len(&self.data)
    }

    /// Test if the bitmap contains only `false` bits.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.first_index().is_none()
    }

    /// Get the value of the bit at a given index.
    #[inline]
    pub fn get(self, index: usize) -> bool {
        Size::get(&self.data, index)
    }

    /// Set the value of the bit at a given index.
    ///
    /// Returns the previous value of the bit.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) -> bool {
        Size::set(&mut self.data, index, value)
    }

    /// Find the index of the first `true` bit in the bitmap.
    #[inline]
    pub fn first_index(self) -> Option<usize> {
        Size::first_index(&self.data)
    }
}

impl<Size: Bits> IntoIterator for Bitmap<Size> {
    type Item = usize;
    type IntoIter = Iter<Size>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            index: 0,
            data: self.data,
        }
    }
}

/// An iterator over the indices in a bitmap which are `true`.
pub struct Iter<Size: Bits> {
    index: usize,
    data: Size::Store,
}

impl<Size: Bits> Iterator for Iter<Size> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= Size::USIZE {
            return None;
        }
        if Size::get(&self.data, self.index) {
            self.index += 1;
            Some(self.index - 1)
        } else {
            self.index += 1;
            self.next()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::collection::btree_set;
    use proptest::proptest;
    use typenum::U64;

    proptest! {
        #[test]
        fn get_set_and_iter(bits in btree_set(0..64usize, 0..64)) {
            let mut bitmap = Bitmap::<U64>::new();
            for i in &bits {
                bitmap.set(*i, true);
            }
            for i in 0..64 {
                assert_eq!(bitmap.get(i), bits.contains(&i));
            }
            assert!(bitmap.into_iter().eq(bits.into_iter()));
        }
    }
}
