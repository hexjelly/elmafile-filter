[![Build Status](https://travis-ci.org/hexjelly/elma-rust.svg?branch=master)](https://travis-ci.org/hexjelly/elma-rust) [![Coverage Status](https://coveralls.io/repos/github/hexjelly/elma-rust/badge.svg?branch=master)](https://coveralls.io/github/hexjelly/elma-rust?branch=master)

# ![logo](http://i.imgur.com/4Pg7LyG.png)

[Elasto Mania](http://elmaonline.net/) file handler crate for Rust.
Very much still a work in progress.

## Installation

Add this in your Cargo.toml file:

```toml
[dependencies]
elma = "*"
```

## Documentation

[http://hexjelly.github.io/elma-rust/elma/](http://hexjelly.github.io/elma-rust/elma/)

## Progress

-   [ ] Across support

### Levels

-   [x] Parse level information
-   [x] Edit levels
-   [x] Save levels/data

### Replays

-   [x] Parse single-player replay information
-   [x] Edit single-player replays
-   [x] Parse multi-player replay information
-   [x] Edit multi-player replays
-   [x] Save replays/data

### State.dat

Might be added, maybe out of scope or redunant for this library.

## Usage examples

### Level operations

To create a new default level:

```rust
extern crate elma;
use elma::lev::*;

fn main () {
    let mut level = Level::new();
    level.save("example.lev", false).unwrap();
}
```

![Screenshot of default level](http://i.imgur.com/TGSo1h4.png)
