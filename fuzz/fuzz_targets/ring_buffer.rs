#![no_main]
#![feature(is_sorted)]

use std::fmt::Debug;
use std::iter::FromIterator;

use arbitrary::Arbitrary;
use array_ops::{ArrayMut, HasLength};
use libfuzzer_sys::fuzz_target;

use imbl_sized_chunks::RingBuffer;

mod assert;
use assert::assert_panic;

#[derive(Arbitrary, Debug)]
enum Construct<A> {
    Empty,
    Single(A),
    Pair((A, A)),
    DrainFrom(RingBuffer<A, 64>),
    CollectFrom(RingBuffer<A, 64>, usize),
    FromFront(RingBuffer<A, 64>, usize),
    FromBack(RingBuffer<A, 64>, usize),
    FromIter(RingBuffer<A, 64>),
}

#[derive(Arbitrary, Debug)]
enum Action<A> {
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
    InsertFrom(Vec<A>, usize),
    InsertOrdered(A),
    Remove(usize),
    Drain,
    Clear,
}

impl<A> Construct<A>
where
    A: Arbitrary<'static> + Clone + Debug + Eq,
{
    fn make(self) -> RingBuffer<A, 64> {
        match self {
            Construct::Empty => {
                let out = RingBuffer::new();
                assert!(out.is_empty());
                out
            }
            Construct::Single(value) => {
                let out = RingBuffer::unit(value.clone());
                assert_eq!(out, vec![value]);
                out
            }
            Construct::Pair((left, right)) => {
                let out = RingBuffer::pair(left.clone(), right.clone());
                assert_eq!(out, vec![left, right]);
                out
            }
            Construct::DrainFrom(vec) => {
                let mut source = RingBuffer::from_iter(vec.iter().cloned());
                let out = RingBuffer::drain_from(&mut source);
                assert!(source.is_empty());
                assert_eq!(out, vec);
                out
            }
            Construct::CollectFrom(mut vec, len) => {
                if vec.is_empty() {
                    return RingBuffer::new();
                }
                let len = len % vec.len();
                let mut source = vec.clone().into_iter();
                let out = RingBuffer::collect_from(&mut source, len);
                let expected_remainder = vec.split_off(len);
                let remainder: Vec<_> = source.collect();
                assert_eq!(expected_remainder, remainder);
                assert_eq!(out, vec);
                out
            }
            Construct::FromFront(mut vec, len) => {
                if vec.is_empty() {
                    return RingBuffer::new();
                }
                let len = len % vec.len();
                let mut source = RingBuffer::from_iter(vec.iter().cloned());
                let out = RingBuffer::from_front(&mut source, len);
                let remainder = vec.split_off(len);
                assert_eq!(source, remainder);
                assert_eq!(out, vec);
                out
            }
            Construct::FromBack(mut vec, len) => {
                if vec.is_empty() {
                    return RingBuffer::new();
                }
                let len = len % vec.len();
                let mut source = RingBuffer::from_iter(vec.iter().cloned());
                let out = RingBuffer::from_back(&mut source, len);
                let remainder = vec.split_off(vec.len() - len);
                assert_eq!(out, remainder);
                assert_eq!(source, vec);
                out
            }
            Construct::FromIter(vec) => {
                let out = vec.clone().into_iter().collect();
                assert_eq!(out, vec);
                out
            }
        }
    }
}

fuzz_target!(|input: (Construct<u32>, Vec<Action<u32>>)| {
    let (cons, actions) = input;
    let capacity = RingBuffer::<u32, 64>::CAPACITY;
    let mut chunk = cons.make();
    let mut guide: Vec<_> = chunk.iter().cloned().collect();
    for action in actions {
        match action {
            Action::PushFront(value) => {
                if chunk.is_full() {
                    assert_panic(|| chunk.push_front(value));
                } else {
                    chunk.push_front(value);
                    guide.insert(0, value);
                }
            }
            Action::PushBack(value) => {
                if chunk.is_full() {
                    assert_panic(|| chunk.push_back(value));
                } else {
                    chunk.push_back(value);
                    guide.push(value);
                }
            }
            Action::PopFront => {
                assert_eq!(
                    chunk.pop_front(),
                    if guide.is_empty() {
                        None
                    } else {
                        Some(guide.remove(0))
                    }
                );
            }
            Action::PopBack => {
                assert_eq!(chunk.pop_back(), guide.pop());
            }
            Action::DropLeft(index) => {
                if index > chunk.len() {
                    assert_panic(|| chunk.drop_left(index));
                } else {
                    chunk.drop_left(index);
                    guide.drain(..index);
                }
            }
            Action::DropRight(index) => {
                if index > chunk.len() {
                    assert_panic(|| chunk.drop_right(index));
                } else {
                    chunk.drop_right(index);
                    guide.drain(index..);
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
            Action::Append(other) => {
                let mut other = other.make();
                let mut other_guide: Vec<_> = other.iter().cloned().collect();
                if other.len() + chunk.len() > capacity {
                    assert_panic(|| chunk.append(&mut other));
                } else {
                    chunk.append(&mut other);
                    guide.append(&mut other_guide);
                }
            }
            Action::DrainFromFront(other, count) => {
                let mut other = other.make();
                let mut other_guide: Vec<_> = other.iter().cloned().collect();
                if count > other.len() || chunk.len() + count > capacity {
                    assert_panic(|| chunk.drain_from_front(&mut other, count));
                } else {
                    chunk.drain_from_front(&mut other, count);
                    guide.extend(other_guide.drain(..count));
                    assert_eq!(other, other_guide);
                }
            }
            Action::DrainFromBack(other, count) => {
                let mut other = other.make();
                let mut other_guide: Vec<_> = other.iter().cloned().collect();
                if count > other.len() || chunk.len() + count > capacity {
                    assert_panic(|| chunk.drain_from_back(&mut other, count));
                } else {
                    let other_index = other.len() - count;
                    chunk.drain_from_back(&mut other, count);
                    guide = other_guide
                        .drain(other_index..)
                        .chain(guide.into_iter())
                        .collect();
                    assert_eq!(other, other_guide);
                }
            }
            Action::Set(index, value) => {
                if index >= chunk.len() {
                    assert_eq!(None, chunk.set(index, value));
                } else {
                    chunk.set(index, value);
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
            Action::InsertFrom(values, index) => {
                if index > chunk.len() || chunk.len() + values.len() > capacity {
                    assert_panic(|| chunk.insert_from(index, values));
                } else {
                    chunk.insert_from(index, values.clone());
                    for value in values.into_iter().rev() {
                        guide.insert(index, value);
                    }
                }
            }
            Action::InsertOrdered(value) => {
                if chunk.iter().is_sorted() {
                    if chunk.is_full() {
                        assert_panic(|| chunk.insert_ordered(value));
                    } else {
                        chunk.insert_ordered(value);
                        match guide.binary_search(&value) {
                            Ok(index) => guide.insert(index, value),
                            Err(index) => guide.insert(index, value),
                        }
                    }
                }
            }
            Action::Remove(index) => {
                if index >= chunk.len() {
                    assert_panic(|| chunk.remove(index));
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
});
