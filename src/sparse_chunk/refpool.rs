use core::mem::MaybeUninit;

use bitmaps::{Bitmap, Bits, BitsImpl};

use ::refpool::{PoolClone, PoolDefault};

use crate::SparseChunk;

impl<A, const N: usize> PoolDefault for SparseChunk<A, N>
where
    BitsImpl<N>: Bits,
{
    unsafe fn default_uninit(target: &mut MaybeUninit<Self>) {
        let ptr = target.as_mut_ptr();
        let map_ptr: *mut Bitmap<N> = &mut (*ptr).map;
        map_ptr.write(Bitmap::new());
    }
}

impl<A, const N: usize> PoolClone for SparseChunk<A, N>
where
    A: Clone,
    BitsImpl<N>: Bits,
{
    unsafe fn clone_uninit(&self, target: &mut MaybeUninit<Self>) {
        let ptr = target.as_mut_ptr();
        let map_ptr: *mut Bitmap<N> = &mut (*ptr).map;
        let data_ptr: *mut _ = &mut (*ptr).data;
        let data_ptr: *mut A = (*data_ptr).as_mut_ptr().cast();
        map_ptr.write(self.map);
        for index in &self.map {
            data_ptr.add(index).write(self[index].clone());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::refpool::{Pool, PoolRef};

    #[test]
    fn default_and_clone() {
        let pool: Pool<SparseChunk<usize, 64>> = Pool::new(16);
        let mut ref1 = PoolRef::default(&pool);
        {
            let chunk = PoolRef::make_mut(&pool, &mut ref1);
            chunk.insert(5, 13);
            chunk.insert(10, 37);
            chunk.insert(31, 337);
        }
        let ref2 = PoolRef::cloned(&pool, &ref1);
        assert_eq!(ref1, ref2);
        assert!(!PoolRef::ptr_eq(&ref1, &ref2));
    }
}
