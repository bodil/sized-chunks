# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) and this project
adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.5.3] - 2020-03-11

### FIXED

-   Debug only assertions made it into the previous release by accident, and this change has been
    reverted. (#7)

## [0.5.2] - 2020-03-10

### ADDED

-   `Chunk` now has an `insert_from` method for inserting multiple values at an index in one go.
-   `Chunk` now also has an `insert_ordered` method for inserting values into a sorted chunk.
-   `SparseChunk` now has the methods `option_iter()`, `option_iter_mut()` and `option_drain()` with
    their corresponding iterators to iterate over a chunk as if it were an array of `Option`s.
-   [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) implementations
    for all data types have been added behind the `arbitrary` feature flag.

### FIXED

-   Internal consistency assertions are now only performed in debug mode (like with
    `debug_assert!`). This means `sized_chunks` will no longer cause panics in release mode when you
    do things like pushing to a full chunk, but do bad and undefined things instead. It also means a
    very slight performance gain.

## [0.5.1] - 2019-12-12

### ADDED

-   `PoolDefault` and `PoolClone` implementations, from the
    [`refpool`](https://crates.io/crates/refpool) crate, are available for `Chunk`, `SparseChunk`
    and `RingBuffer`, behind the `refpool` feature flag.

## [0.5.0] - 2019-09-09

### CHANGED

-   The `Bitmap` type (and its helper type, `Bits`) has been split off into a separate crate, named
    `bitmaps`. If you need it, it's in that crate now. `sized-chunks` does not re-export it. Of
    course, this means `sized-chunks` has gained `bitmaps` as its second hard dependency.

## [0.4.0] - 2019-09-02

### CHANGED

-   The 0.3.2 release increased the minimum rustc version required, which should have been a major
    version bump, so 0.3.2 is being yanked and re-tagged as 0.4.0.

## [0.3.2] - 2019-08-29

### ADDED

-   Chunk/bitmap sizes up to 1024 are now supported.

### FIXED

-   Replaced `ManuallyDrop` in implementations with `MaybeUninit`, along with a general unsafe code
    cleanup. (#3)

## [0.3.1] - 2019-08-03

### ADDED

-   Chunk sizes up to 256 are now supported.

## [0.3.0] - 2019-05-18

### ADDED

-   A new data structure, `InlineArray`, which is a stack allocated array matching the size of a
    given type, intended for optimising for the case of very small vectors.
-   `Chunk` has an implementation of `From<InlineArray>` which is considerably faster than going via
    iterators.

## [0.2.2] - 2019-05-10

### ADDED

-   `Slice::get` methods now return references with the lifetime of the underlying `RingBuffer`
    rather than the lifetime of the slice.

## [0.2.1] - 2019-04-15

### ADDED

-   A lot of documentation.
-   `std::io::Read` implementations for `Chunk<u8>` and `RingBuffer<u8>` to match their `Write`
    implementations.

## [0.2.0] - 2019-04-14

### CHANGED

-   The `capacity()` method has been replacied with a `CAPACITY` const on each type.

### ADDED

-   There is now a `RingBuffer` implementation, which should be nearly a drop-in replacement for
    `SizedChunk` but is always O(1) on push and cannot be dereferenced to slices (but it has a set
    of custom slice-like implementations to make that less of a drawback).
-   The `Drain` iterator for `SizedChunk` now implements `DoubleEndedIterator`.

### FIXED

-   `SizedChunk::drain_from_front/back` will now always panic if the iterator underflows, instead of
    only doing it in debug mode.

## [0.1.3] - 2019-04-12

### ADDED

-   `SparseChunk` now has a default length of `U64`.
-   `Chunk` now has `PartialEq` defined for anything that can be borrowed as a slice.
-   `SparseChunk<A>` likewise has `PartialEq` defined for `BTreeMap<usize, A>` and
    `HashMap<usize, A>`. These are intended for debugging and aren't optimally `efficient.
-   `Chunk` and `SparseChunk` now have a new method `capacity()` which returns its maximum capacity
    (the number in the type) as a usize.
-   Added an `entries()` method to `SparseChunk`.
-   `SparseChunk` now has a `Debug` implementation.

### FIXED

-   Extensive integration tests were added for `Chunk` and `SparseChunk`.
-   `Chunk::clear` is now very slightly faster.

## [0.1.2] - 2019-03-11

### FIXED

-   Fixed an alignment issue in `Chunk::drain_from_back`. (#1)

## [0.1.1] - 2019-02-19

### FIXED

-   Some 2018 edition issues.

## [0.1.0] - 2019-02-19

Initial release.
