#![no_main]

use std::collections::BTreeMap;
use std::fmt::Debug;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use sized_chunks::SparseChunk;

mod assert;
use assert::assert_panic;

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

fuzz_target!(|input: (Construct<u32>, Vec<Action<u32>>)| {
    let (cons, actions) = input;
    let capacity = SparseChunk::<u32>::CAPACITY;
    let mut chunk = cons.make();
    let mut guide: BTreeMap<_, _> = chunk.entries().map(|(i, v)| (i, *v)).collect();
    for action in actions {
        match action {
            Action::Insert(index, value) => {
                if index >= capacity {
                    assert_panic(|| chunk.insert(index, value));
                } else {
                    assert_eq!(chunk.insert(index, value), guide.insert(index, value));
                }
            }
            Action::Remove(index) => {
                if index >= capacity {
                    assert_panic(|| chunk.remove(index));
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
});
