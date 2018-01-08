# num_alias
[![Crates.io: num_alias](http://meritbadge.herokuapp.com/num_alias)](https://crates.io/crates/num_alias)
[![Documentation](https://docs.rs/num_alias/badge.svg)](https://docs.rs/num_alias)
[![Build Status](https://travis-ci.org/kngwyu/num-alias.svg?branch=master)](https://travis-ci.org/kngwyu/num-alias)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
Provides simple and useful macros to declare 'type and range checked' aliases for integers and floats.

``` rust
#[macro_use]
extern crate num_alias;
fn main() {
    // declare alias with range[3, 6)
    int_alias!(Val, i32, 3 => 6);
    let a = Val(5);
    let b = Val(4);
    // this code panics
    let c = a * b;
}
```

