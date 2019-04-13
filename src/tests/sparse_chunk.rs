#![allow(clippy::unit_arg)]

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::panic::{catch_unwind, AssertUnwindSafe};

use proptest::{arbitrary::any, collection::vec, prelude::*, proptest};
use proptest_derive::Arbitrary;

use crate::sparse_chunk::SparseChunk;

#[derive(Arbitrary, Debug)]
enum Construct<A> {
    Empty,
    Single((usize, A)),
    Pair((usize, A, usize, A)),
}

#[derive(Arbitrary, Debug)]
enum Action<A> {
    Insert(usize, A),
    Remove(usize),
    Pop,
}

impl<A> Construct<A>
where
    A: Arbitrary + Clone + Debug + Eq,
    <A as Arbitrary>::Strategy: 'static,
{
    fn make(self) -> SparseChunk<A> {
        match self {
            Construct::Empty => {
                let out = SparseChunk::new();
                assert!(out.is_empty());
                out
            }
            Construct::Single((index, value)) => {
                let index = index % SparseChunk::<A>::CAPACITY;
                let out = SparseChunk::unit(index, value.clone());
                let mut guide = BTreeMap::new();
                guide.insert(index, value);
                assert_eq!(out, guide);
                out
            }
            Construct::Pair((left_index, left, right_index, right)) => {
                let left_index = left_index % SparseChunk::<A>::CAPACITY;
                let right_index = right_index % SparseChunk::<A>::CAPACITY;
                let out = SparseChunk::pair(left_index, left.clone(), right_index, right.clone());
                let mut guide = BTreeMap::new();
                guide.insert(left_index, left);
                guide.insert(right_index, right);
                assert_eq!(out, guide);
                out
            }
        }
    }
}

proptest! {
    #[test]
    fn test_constructors(cons: Construct<u32>) {
        cons.make();
    }

    #[test]
    fn test_actions(cons: Construct<u32>, actions in vec(any::<Action<u32>>(), 0..super::action_count())) {
        let capacity = SparseChunk::<u32>::CAPACITY;
        let mut chunk = cons.make();
        let mut guide: BTreeMap<_, _> = chunk.entries().map(|(i, v)| (i, *v)).collect();
        for action in actions {
            match action {
                Action::Insert(index, value) => {
                    if index >= capacity {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.insert(index, value))).is_err());
                    } else {
                        assert_eq!(chunk.insert(index, value), guide.insert(index, value));
                    }
                }
                Action::Remove(index) => {
                    if index >= capacity {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.remove(index))).is_err());
                    } else {
                        assert_eq!(chunk.remove(index), guide.remove(&index));
                    }
                }
                Action::Pop => {
                    if let Some(index) = chunk.first_index() {
                        assert_eq!(chunk.pop(), guide.remove(&index));
                    } else {
                        assert_eq!(chunk.pop(), None);
                    }
                }
            }
            assert_eq!(chunk, guide);
            assert!(guide.len() <= SparseChunk::<u32>::CAPACITY);
        }
    }
}
