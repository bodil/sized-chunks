#![allow(clippy::unit_arg)]

use std::fmt::Debug;
use std::iter::FromIterator;
use std::panic::{catch_unwind, AssertUnwindSafe};

use proptest::{arbitrary::any, collection::vec, prelude::*, proptest};
use proptest_derive::Arbitrary;

use crate::sized_chunk::Chunk;

#[test]
fn validity_invariant() {
    assert!(Some(Chunk::<Box<()>>::new()).is_some());
}

#[derive(Debug)]
struct InputVec<A>(Vec<A>);

impl<A> InputVec<A> {
    fn unwrap(self) -> Vec<A> {
        self.0
    }
}

impl<A> Arbitrary for InputVec<A>
where
    A: Arbitrary + Debug,
    <A as Arbitrary>::Strategy: 'static,
{
    type Parameters = usize;
    type Strategy = BoxedStrategy<InputVec<A>>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        #[allow(clippy::redundant_closure)]
        proptest::collection::vec(any::<A>(), 0..Chunk::<u32>::CAPACITY)
            .prop_map(|v| InputVec(v))
            .boxed()
    }
}

#[derive(Arbitrary, Debug)]
enum Construct<A>
where
    A: Arbitrary,
    <A as Arbitrary>::Strategy: 'static,
{
    Empty,
    Single(A),
    Pair((A, A)),
    DrainFrom(InputVec<A>),
    CollectFrom(InputVec<A>, usize),
    FromFront(InputVec<A>, usize),
    FromBack(InputVec<A>, usize),
}

#[derive(Arbitrary, Debug)]
enum Action<A>
where
    A: Arbitrary,
    <A as Arbitrary>::Strategy: 'static,
{
    PushFront(A),
    PushBack(A),
    PopFront,
    PopBack,
    DropLeft(usize),
    DropRight(usize),
    SplitOff(usize),
    Append(Construct<A>),
    DrainFromFront(Construct<A>, usize),
    DrainFromBack(Construct<A>, usize),
    Set(usize, A),
    Insert(usize, A),
    Remove(usize),
    Drain,
    Clear,
}

impl<A> Construct<A>
where
    A: Arbitrary + Clone + Debug + Eq,
    <A as Arbitrary>::Strategy: 'static,
{
    fn make(self) -> Chunk<A> {
        match self {
            Construct::Empty => {
                let out = Chunk::new();
                assert!(out.is_empty());
                out
            }
            Construct::Single(value) => {
                let out = Chunk::unit(value.clone());
                assert_eq!(out, vec![value]);
                out
            }
            Construct::Pair((left, right)) => {
                let out = Chunk::pair(left.clone(), right.clone());
                assert_eq!(out, vec![left, right]);
                out
            }
            Construct::DrainFrom(vec) => {
                let vec = vec.unwrap();
                let mut source = Chunk::from_iter(vec.iter().cloned());
                let out = Chunk::drain_from(&mut source);
                assert!(source.is_empty());
                assert_eq!(out, vec);
                out
            }
            Construct::CollectFrom(vec, len) => {
                let mut vec = vec.unwrap();
                if vec.is_empty() {
                    return Chunk::new();
                }
                let len = len % vec.len();
                let mut source = vec.clone().into_iter();
                let out = Chunk::collect_from(&mut source, len);
                let expected_remainder = vec.split_off(len);
                let remainder: Vec<_> = source.collect();
                assert_eq!(expected_remainder, remainder);
                assert_eq!(out, vec);
                out
            }
            Construct::FromFront(vec, len) => {
                let mut vec = vec.unwrap();
                if vec.is_empty() {
                    return Chunk::new();
                }
                let len = len % vec.len();
                let mut source = Chunk::from_iter(vec.iter().cloned());
                let out = Chunk::from_front(&mut source, len);
                let remainder = vec.split_off(len);
                assert_eq!(source, remainder);
                assert_eq!(out, vec);
                out
            }
            Construct::FromBack(vec, len) => {
                let mut vec = vec.unwrap();
                if vec.is_empty() {
                    return Chunk::new();
                }
                let len = len % vec.len();
                let mut source = Chunk::from_iter(vec.iter().cloned());
                let out = Chunk::from_back(&mut source, len);
                let remainder = vec.split_off(vec.len() - len);
                assert_eq!(out, remainder);
                assert_eq!(source, vec);
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
        let capacity = Chunk::<u32>::CAPACITY;
        let mut chunk = cons.make();
        let mut guide: Vec<_> = chunk.iter().cloned().collect();
        for action in actions {
            match action {
                Action::PushFront(value) => {
                    if chunk.is_full() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.push_front(value))).is_err());
                    } else {
                        chunk.push_front(value);
                        guide.insert(0, value);
                    }
                }
                Action::PushBack(value) => {
                    if chunk.is_full() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.push_back(value))).is_err());
                    } else {
                        chunk.push_back(value);
                        guide.push(value);
                    }
                }
                Action::PopFront => {
                    if chunk.is_empty() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.pop_front())).is_err());
                    } else {
                        assert_eq!(chunk.pop_front(), guide.remove(0));
                    }
                }
                Action::PopBack => {
                    if chunk.is_empty() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.pop_back())).is_err());
                    } else {
                        assert_eq!(chunk.pop_back(), guide.pop().unwrap());
                    }
                }
                Action::DropLeft(index) => {
                    if index >= chunk.len() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.drop_left(index))).is_err());
                    } else {
                        chunk.drop_left(index);
                        guide.drain(..index);
                    }
                }
                Action::DropRight(index) => {
                    if index >= chunk.len() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.drop_right(index))).is_err());
                    } else {
                        chunk.drop_right(index);
                        guide.drain(index..);
                    }
                }
                Action::SplitOff(index) => {
                    if index >= chunk.len() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.split_off(index))).is_err());
                    } else {
                        let chunk_off = chunk.split_off(index);
                        let guide_off = guide.split_off(index);
                        assert_eq!(chunk_off, guide_off);
                    }
                }
                Action::Append(other) => {
                    let mut other = other.make();
                    let mut other_guide: Vec<_> = other.iter().cloned().collect();
                    if other.len() + chunk.len() > capacity {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.append(&mut other))).is_err());
                    } else {
                        chunk.append(&mut other);
                        guide.append(&mut other_guide);
                    }
                }
                Action::DrainFromFront(other, count) => {
                    let mut other = other.make();
                    let mut other_guide: Vec<_> = other.iter().cloned().collect();
                    if count >= other.len() || chunk.len() + count > capacity {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.drain_from_front(&mut other, count))).is_err());
                    } else {
                        chunk.drain_from_front(&mut other, count);
                        guide.extend(other_guide.drain(..count));
                        assert_eq!(other, other_guide);
                    }
                }
                Action::DrainFromBack(other, count) => {
                    let mut other = other.make();
                    let mut other_guide: Vec<_> = other.iter().cloned().collect();
                    if count >= other.len() || chunk.len() + count > capacity {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.drain_from_back(&mut other, count))).is_err());
                    } else {
                        chunk.drain_from_back(&mut other, count);
                        let other_index = other.len() - count;
                        guide = other_guide.drain(other_index..).chain(guide.into_iter()).collect();
                        assert_eq!(other, other_guide);
                    }
                }
                Action::Set(index, value) => {
                    if index >= chunk.len() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.set(index, value))).is_err());
                    } else {
                        chunk.set(index, value);
                        guide[index] = value;
                    }
                }
                Action::Insert(index, value) => {
                    if index >= chunk.len() || chunk.is_full() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.insert(index, value))).is_err());
                    } else {
                        chunk.insert(index, value);
                        guide.insert(index, value);
                    }
                }
                Action::Remove(index) => {
                    if index >= chunk.len() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.remove(index))).is_err());
                    } else {
                        assert_eq!(chunk.remove(index), guide.remove(index));
                    }
                }
                Action::Drain => {
                    let drained: Vec<_> = chunk.drain().collect();
                    let drained_guide: Vec<_> = guide.drain(..).collect();
                    assert_eq!(drained, drained_guide);
                }
                Action::Clear => {
                    chunk.clear();
                    guide.clear();
                }
            }
            assert_eq!(chunk, guide);
            assert!(guide.len() <= capacity);
        }
    }
}
