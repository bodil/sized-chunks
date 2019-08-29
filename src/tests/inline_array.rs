#![allow(clippy::unit_arg)]

use std::panic::{catch_unwind, AssertUnwindSafe};

use proptest::{arbitrary::any, collection::vec, prelude::*, proptest};
use proptest_derive::Arbitrary;

use crate::inline_array::InlineArray;

#[test]
fn validity_invariant() {
    assert!(Some(InlineArray::<usize, [Box<()>; 2]>::new()).is_some());

    let mut chunk = InlineArray::<usize, [Box<()>; 2]>::new();
    chunk.push(0);
    assert!(Some(chunk).is_some());
}

type TestType = [usize; 16];

#[derive(Arbitrary, Debug)]
enum Action<A>
where
    A: Arbitrary,
    <A as Arbitrary>::Strategy: 'static,
{
    Push(A),
    Pop,
    Set((usize, A)),
    Insert(usize, A),
    Remove(usize),
    SplitOff(usize),
    Drain,
    Clear,
}

proptest! {
    #[test]
    fn test_actions(actions in vec(any::<Action<u32>>(), 0..super::action_count())) {
        let capacity = InlineArray::<u32, TestType>::CAPACITY;
        let mut chunk = InlineArray::<u32, TestType>::new();
        let mut guide: Vec<_> = chunk.iter().cloned().collect();
        for action in actions {
            match action {
                 Action::Push(value) => {
                    if chunk.is_full() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk.push(value))).is_err());
                    } else {
                        chunk.push(value);
                        guide.push(value);
                    }
                }
                Action::Pop => {
                    assert_eq!(chunk.pop(), guide.pop());
                }
                Action::Set((index, value)) => {
                    if index >= chunk.len() {
                        assert!(catch_unwind(AssertUnwindSafe(|| chunk[index] = value)).is_err());
                    } else {
                        chunk[index] = value;
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
                        assert_eq!(None, chunk.remove(index));
                    } else {
                        assert_eq!(chunk.remove(index), Some(guide.remove(index)));
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
