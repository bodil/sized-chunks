// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A fixed capacity sparse array.
//!
//! See [`SparseChunk`](struct.SparseChunk.html)

use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Error, Formatter};
use std::mem::{self, MaybeUninit};
use std::ops::Index;
use std::ops::IndexMut;
use std::ptr;
use std::slice::{from_raw_parts, from_raw_parts_mut};

use typenum::U64;

use bitmaps::{Bitmap, Bits, Iter as BitmapIter};

use crate::types::ChunkLength;

/// A fixed capacity sparse array.
///
/// An inline sparse array of up to `N` items of type `A`, where `N` is an
/// [`Unsigned`][Unsigned] type level numeral. You can think of it as an array
/// of `Option<A>`, where the discriminant (whether the value is `Some<A>` or
/// `None`) is kept in a bitmap instead of adjacent to the value.
///
/// Because the bitmap is kept in a primitive type, the maximum value of `N` is
/// currently 128, corresponding to a type of `u128`. The type of the bitmap
/// will be the minimum unsigned integer type required to fit the number of bits
/// required. Thus, disregarding memory alignment rules, the allocated size of a
/// `SparseChunk` will be `uX` + `A` * `N` where `uX` is the type of the
/// discriminant bitmap, either `u8`, `u16`, `u32`, `u64` or `u128`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate sized_chunks;
/// # extern crate typenum;
/// # use sized_chunks::SparseChunk;
/// # use typenum::U20;
/// # fn main() {
/// // Construct a chunk with a 20 item capacity
/// let mut chunk = SparseChunk::<i32, U20>::new();
/// // Set the 18th index to the value 5.
/// chunk.insert(18, 5);
/// // Set the 5th index to the value 23.
/// chunk.insert(5, 23);
///
/// assert_eq!(chunk.len(), 2);
/// assert_eq!(chunk.get(5), Some(&23));
/// assert_eq!(chunk.get(6), None);
/// assert_eq!(chunk.get(18), Some(&5));
/// # }
/// ```
///
/// [Unsigned]: https://docs.rs/typenum/1.10.0/typenum/marker_traits/trait.Unsigned.html
pub struct SparseChunk<A, N: Bits + ChunkLength<A> = U64> {
    map: Bitmap<N>,
    data: MaybeUninit<N::SizedType>,
}

impl<A, N: Bits + ChunkLength<A>> Drop for SparseChunk<A, N> {
    fn drop(&mut self) {
        if mem::needs_drop::<A>() {
            for index in &self.map.clone() {
                unsafe { SparseChunk::force_drop(index, self) }
            }
        }
    }
}

impl<A: Clone, N: Bits + ChunkLength<A>> Clone for SparseChunk<A, N> {
    fn clone(&self) -> Self {
        let mut out = Self::new();
        for index in &self.map {
            out.insert(index, self[index].clone());
        }
        out
    }
}

impl<A, N> SparseChunk<A, N>
where
    N: Bits + ChunkLength<A>,
{
    pub const CAPACITY: usize = N::USIZE;

    #[inline]
    fn values(&self) -> &[A] {
        unsafe { from_raw_parts(&self.data as *const _ as *const A, N::USIZE) }
    }

    #[inline]
    fn values_mut(&mut self) -> &mut [A] {
        unsafe { from_raw_parts_mut(&mut self.data as *mut _ as *mut A, N::USIZE) }
    }

    /// Copy the value at an index, discarding ownership of the copied value
    #[inline]
    unsafe fn force_read(index: usize, chunk: &Self) -> A {
        ptr::read(&chunk.values()[index as usize])
    }

    /// Write a value at an index without trying to drop what's already there
    #[inline]
    unsafe fn force_write(index: usize, value: A, chunk: &mut Self) {
        ptr::write(&mut chunk.values_mut()[index as usize], value)
    }

    /// Drop the value at an index
    #[inline]
    unsafe fn force_drop(index: usize, chunk: &mut Self) {
        ptr::drop_in_place(&mut chunk.values_mut()[index])
    }

    /// Construct a new empty chunk.
    pub fn new() -> Self {
        Self {
            map: Bitmap::default(),
            data: MaybeUninit::uninit(),
        }
    }

    /// Construct a new chunk with one item.
    pub fn unit(index: usize, value: A) -> Self {
        let mut chunk = Self::new();
        chunk.insert(index, value);
        chunk
    }

    /// Construct a new chunk with two items.
    pub fn pair(index1: usize, value1: A, index2: usize, value2: A) -> Self {
        let mut chunk = Self::new();
        chunk.insert(index1, value1);
        chunk.insert(index2, value2);
        chunk
    }

    /// Get the length of the chunk.
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Test if the chunk is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.len() == 0
    }

    /// Test if the chunk is at capacity.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == N::USIZE
    }

    /// Insert a new value at a given index.
    ///
    /// Returns the previous value at that index, if any.
    pub fn insert(&mut self, index: usize, value: A) -> Option<A> {
        if index >= N::USIZE {
            panic!("SparseChunk::insert: index out of bounds");
        }
        if self.map.set(index, true) {
            Some(mem::replace(&mut self.values_mut()[index], value))
        } else {
            unsafe { SparseChunk::force_write(index, value, self) };
            None
        }
    }

    /// Remove the value at a given index.
    ///
    /// Returns the value, or `None` if the index had no value.
    pub fn remove(&mut self, index: usize) -> Option<A> {
        if index >= N::USIZE {
            panic!("SparseChunk::remove: index out of bounds");
        }
        if self.map.set(index, false) {
            Some(unsafe { SparseChunk::force_read(index, self) })
        } else {
            None
        }
    }

    /// Remove the first value present in the array.
    ///
    /// Returns the value that was removed, or `None` if the array was empty.
    pub fn pop(&mut self) -> Option<A> {
        self.first_index().and_then(|index| self.remove(index))
    }

    /// Get the value at a given index.
    pub fn get(&self, index: usize) -> Option<&A> {
        if index >= N::USIZE {
            return None;
        }
        if self.map.get(index) {
            Some(&self.values()[index])
        } else {
            None
        }
    }

    /// Get a mutable reference to the value at a given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut A> {
        if index >= N::USIZE {
            return None;
        }
        if self.map.get(index) {
            Some(&mut self.values_mut()[index])
        } else {
            None
        }
    }

    /// Make an iterator over the indices which contain values.
    pub fn indices(&self) -> BitmapIter<N> {
        self.map.into_iter()
    }

    /// Find the first index which contains a value.
    pub fn first_index(&self) -> Option<usize> {
        self.map.first_index()
    }

    /// Make an iterator of references to the values contained in the array.
    pub fn iter(&self) -> Iter<'_, A, N> {
        Iter {
            indices: self.indices(),
            chunk: self,
        }
    }

    /// Make an iterator of mutable references to the values contained in the
    /// array.
    pub fn iter_mut(&mut self) -> IterMut<'_, A, N> {
        IterMut {
            bitmap: self.map.clone(),
            chunk: self,
        }
    }

    /// Turn the chunk into an iterator over the values contained within it.
    pub fn drain(self) -> Drain<A, N> {
        Drain { chunk: self }
    }

    /// Make an iterator of pairs of indices and references to the values
    /// contained in the array.
    pub fn entries(&self) -> impl Iterator<Item = (usize, &A)> {
        self.indices().zip(self.iter())
    }
}

impl<A, N: Bits + ChunkLength<A>> Index<usize> for SparseChunk<A, N> {
    type Output = A;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<A, N: Bits + ChunkLength<A>> IndexMut<usize> for SparseChunk<A, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<A, N: Bits + ChunkLength<A>> IntoIterator for SparseChunk<A, N> {
    type Item = A;
    type IntoIter = Drain<A, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.drain()
    }
}

impl<A, N> PartialEq for SparseChunk<A, N>
where
    A: PartialEq,
    N: Bits + ChunkLength<A>,
{
    fn eq(&self, other: &Self) -> bool {
        if self.map != other.map {
            return false;
        }
        for index in self.indices() {
            if self.get(index) != other.get(index) {
                return false;
            }
        }
        true
    }
}

impl<A, N> PartialEq<BTreeMap<usize, A>> for SparseChunk<A, N>
where
    A: PartialEq,
    N: Bits + ChunkLength<A>,
{
    fn eq(&self, other: &BTreeMap<usize, A>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for index in 0..N::USIZE {
            if self.get(index) != other.get(&index) {
                return false;
            }
        }
        true
    }
}

impl<A, N> PartialEq<HashMap<usize, A>> for SparseChunk<A, N>
where
    A: PartialEq,
    N: Bits + ChunkLength<A>,
{
    fn eq(&self, other: &HashMap<usize, A>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for index in 0..N::USIZE {
            if self.get(index) != other.get(&index) {
                return false;
            }
        }
        true
    }
}

impl<A, N> Eq for SparseChunk<A, N>
where
    A: Eq,
    N: Bits + ChunkLength<A>,
{
}

impl<A, N> Debug for SparseChunk<A, N>
where
    A: Debug,
    N: Bits + ChunkLength<A>,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("SparseChunk")?;
        f.debug_map().entries(self.entries()).finish()
    }
}

pub struct Iter<'a, A: 'a, N: 'a + Bits + ChunkLength<A>> {
    indices: BitmapIter<'a, N>,
    chunk: &'a SparseChunk<A, N>,
}

impl<'a, A, N: Bits + ChunkLength<A>> Iterator for Iter<'a, A, N> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|index| &self.chunk.values()[index])
    }
}

pub struct IterMut<'a, A: 'a, N: 'a + Bits + ChunkLength<A>> {
    bitmap: Bitmap<N>,
    chunk: &'a mut SparseChunk<A, N>,
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

pub struct Drain<A, N: Bits + ChunkLength<A>> {
    chunk: SparseChunk<A, N>,
}

impl<'a, A, N: Bits + ChunkLength<A>> Iterator for Drain<A, N> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunk.pop()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use typenum::U32;

    #[test]
    fn insert_remove_iterate() {
        let mut chunk: SparseChunk<_, U32> = SparseChunk::new();
        assert_eq!(None, chunk.insert(5, 5));
        assert_eq!(None, chunk.insert(1, 1));
        assert_eq!(None, chunk.insert(24, 42));
        assert_eq!(None, chunk.insert(22, 22));
        assert_eq!(Some(42), chunk.insert(24, 24));
        assert_eq!(None, chunk.insert(31, 31));
        assert_eq!(Some(24), chunk.remove(24));
        assert_eq!(4, chunk.len());
        let indices: Vec<_> = chunk.indices().collect();
        assert_eq!(vec![1, 5, 22, 31], indices);
        let values: Vec<_> = chunk.into_iter().collect();
        assert_eq!(vec![1, 5, 22, 31], values);
    }

    #[test]
    fn clone_chunk() {
        let mut chunk: SparseChunk<_, U32> = SparseChunk::new();
        assert_eq!(None, chunk.insert(5, 5));
        assert_eq!(None, chunk.insert(1, 1));
        assert_eq!(None, chunk.insert(24, 42));
        assert_eq!(None, chunk.insert(22, 22));
        let cloned = chunk.clone();
        let right_indices: Vec<_> = chunk.indices().collect();
        let left_indices: Vec<_> = cloned.indices().collect();
        let right: Vec<_> = chunk.into_iter().collect();
        let left: Vec<_> = cloned.into_iter().collect();
        assert_eq!(left, right);
        assert_eq!(left_indices, right_indices);
        assert_eq!(vec![1, 5, 22, 24], left_indices);
        assert_eq!(vec![1, 5, 22, 24], right_indices);
    }
}
