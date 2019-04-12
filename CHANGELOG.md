# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic
Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### ADDED

- `Chunk` now has `PartialEq` defined for anything that can be borrowed as a
  slice.
- `Chunk` has a new method `capacity()` which returns its maximum capacity (the
  number in the type) as a usize.

### FIXED

- Extensive integration tests were added for `Chunk`.
- `Chunk::clear` is now very slightly faster.

## [0.1.2] - 2019-03-11

### FIXED

- Fixed an alignment issue in `Chunk::drain_from_back`. (#1)

## [0.1.1] - 2019-02-19

### FIXED

- Some 2018 edition issues.

## [0.1.0] - 2019-02-19

Initial release.
