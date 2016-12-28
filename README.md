[![Build Status](https://travis-ci.org/hexjelly/elma-rust.svg?branch=master)](https://travis-ci.org/hexjelly/elma-rust) [![Build status](https://ci.appveyor.com/api/projects/status/0mqnblvhjwlyltkn/branch/master?svg=true)](https://ci.appveyor.com/project/hexjelly/elma-rust/branch/master) [![Coverage Status](https://coveralls.io/repos/github/hexjelly/elma-rust/badge.svg?branch=master)](https://coveralls.io/github/hexjelly/elma-rust?branch=master) [![Docs](https://docs.rs/elma/badge.svg)](https://docs.rs/elma/)

# ![logo](http://i.imgur.com/4Pg7LyG.png)

[Elasto Mania](http://elmaonline.net/) file handler crate for Rust.
Very much still a work in progress.

## Requirements

Rust >1.13

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
-   [ ] state.dat support

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
