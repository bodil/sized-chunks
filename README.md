# imbl-sized-chunks

Various fixed length array data types.

## Overview

This crate provides the core building blocks for the immutable data structures
in [imbl]: a sized array with O(1) amortised double ended push/pop and
smarter insert/remove performance (used by `imbl::Vector` and `imbl::OrdMap`), and a
fixed size sparse array (used by `imbl::HashMap`).

In a nutshell, this crate contains the unsafe bits from [imbl], which
may or may not be useful to anyone else, and have been split out for ease of
auditing.

## Documentation

* [API docs](https://docs.rs/imbl-sized-chunks)

## Licence

Copyright 2019 Bodil Stokke
Copyright 2022 Joe Neeman

This software is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.

## Code of Conduct

Please note that this project is released with a [Contributor Code of
Conduct][coc]. By participating in this project you agree to abide by its
terms.

## Acknowledgement

This crate was forked from [`sized-chunks`], which is where basicaly all of the work was done.

[imbl]: https://crates.io/crates/imbl
[coc]: https://github.com/jneem/imbl-sized-chunks/blob/master/CODE_OF_CONDUCT.md
[`sized_chunks`]: https://crates.io/crates/sized-chunks
