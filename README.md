[![Build Status](https://travis-ci.org/elmadev/elma-rust.svg?branch=master)](https://travis-ci.org/elmadev/elma-rust) [![Build status](https://ci.appveyor.com/api/projects/status/0mqnblvhjwlyltkn/branch/master?svg=true)](https://ci.appveyor.com/project/hexjelly/elma-rust/branch/master) [![Coverage Status](https://coveralls.io/repos/github/elmadev/elma-rust/badge.svg?branch=master)](https://coveralls.io/github/elmadev/elma-rust?branch=master) [![Docs](https://docs.rs/elma/badge.svg)](https://docs.rs/elma/)

# ![logo](http://i.imgur.com/4Pg7LyG.png)

[Elasto Mania](http://elmaonline.net/) file handler crate for Rust.
Very much still a work in progress.

## Requirements

Until a 1.0 release this will only target the latest stable version of Rust.

## Installation

Add this in your Cargo.toml file:

```toml
[dependencies]
elma = "*"
```

## Documentation

[https://docs.rs/elma/](https://docs.rs/elma/)

## Progress

-   [ ] Across support
-   [x] Elma support
    -   [x] Level
    -   [x] Replay
    -   [x] LGR
    -   [x] state.dat best times write/read support
    -   [ ] full state.dat support

## Usage examples

### Level operations

To create a new default level:

```rust
extern crate elma;
use elma::lev::*;

fn main () {
    let mut level = Level::new();
    level.save("example.lev", Top10Save::No).unwrap();
}
```

![Screenshot of default level](http://i.imgur.com/TGSo1h4.png)
