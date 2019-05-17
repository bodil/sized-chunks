# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic
Versioning](http://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2019-05-18

### ADDED

- A new data structure, `InlineArray`, which is a stack allocated array matching
  the size of a given type, intended for optimising for the case of very small
  vectors.
- `Chunk` has an implementation of `From<InlineArray>` which is considerably
  faster than going via iterators.

## [0.2.2] - 2019-05-10

### ADDED

- `Slice::get` methods now return references with the lifetime of the underlying
  `RingBuffer` rather than the lifetime of the slice.

## [0.2.1] - 2019-04-15

### ADDED

- A lot of documentation.
- `std::io::Read` implementations for `Chunk<u8>` and `RingBuffer<u8>` to match
  their `Write` implementations.

## [0.2.0] - 2019-04-14

### CHANGED

- The `capacity()` method has been replacied with a `CAPACITY` const on each
  type.

### ADDED

- There is now a `RingBuffer` implementation, which should be nearly a drop-in
  replacement for `SizedChunk` but is always O(1) on push and cannot be
  dereferenced to slices (but it has a set of custom slice-like implementations
  to make that less of a drawback).
- The `Drain` iterator for `SizedChunk` now implements `DoubleEndedIterator`.

### FIXED

- `SizedChunk::drain_from_front/back` will now always panic if the iterator
  underflows, instead of only doing it in debug mode.

## [0.1.3] - 2019-04-12

### ADDED

- `SparseChunk` now has a default length of `U64`.
- `Chunk` now has `PartialEq` defined for anything that can be borrowed as a
  slice.
- `SparseChunk<A>` likewise has `PartialEq` defined for `BTreeMap<usize, A>` and
  `HashMap<usize, A>`. These are intended for debugging and aren't optimally
  `efficient.
- `Chunk` and `SparseChunk` now have a new method `capacity()` which returns its
  maximum capacity (the number in the type) as a usize.
- Added an `entries()` method to `SparseChunk`.
- `SparseChunk` now has a `Debug` implementation.

### FIXED

- Extensive integration tests were added for `Chunk` and `SparseChunk`.
- `Chunk::clear` is now very slightly faster.

## [0.1.2] - 2019-03-11

### FIXED

- Fixed an alignment issue in `Chunk::drain_from_back`. (#1)

## [0.1.1] - 2019-02-19

### FIXED

- Some 2018 edition issues.

## [0.1.0] - 2019-02-19

Initial release.
