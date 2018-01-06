# num-alias
[![Documentation](https://docs.rs/num_alias/badge.svg)](https://docs.rs/num_alias)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Provides simple and useful macros to declare 'type checked' aliases for integers and floats.

``` rust
#[macro_use]
extern crate num_alias;
fn main() {
    float_alias!(Fval, f64);
    let a = Fval(5.0);
    let b = Fval(4.0);
    let c = (a * b).sqrt();
}
```

