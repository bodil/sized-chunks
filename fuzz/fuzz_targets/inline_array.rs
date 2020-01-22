#![no_main]

use std::fmt::Debug;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use sized_chunks::InlineArray;

mod assert;
use assert::assert_panic;

type TestType = [usize; 16];

#[derive(Arbitrary, Debug)]
enum Action<A>
where
    A: Arbitrary,
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

fuzz_target!(|actions: Vec<Action<u32>>| {
    let capacity = InlineArray::<u32, TestType>::CAPACITY;
    let mut chunk = InlineArray::<u32, TestType>::new();
    let mut guide: Vec<_> = chunk.iter().cloned().collect();
    for action in actions {
        match action {
            Action::Push(value) => {
                if chunk.is_full() {
                    assert_panic(|| chunk.push(value));
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
                    assert_panic(|| chunk[index] = value);
                } else {
                    chunk[index] = value;
                    guide[index] = value;
                }
            }
            Action::Insert(index, value) => {
                if index > chunk.len() || chunk.is_full() {
                    assert_panic(|| chunk.insert(index, value));
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
                if index > chunk.len() {
                    assert_panic(|| chunk.split_off(index));
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
});
