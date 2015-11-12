# termsize

[![Build Status](https://travis-ci.org/softprops/termsize.svg)](https://travis-ci.org/softprops/termsize)

> because terminal size matters

## api docs

Find them [here](https://softprops.github.com/termsize)

## usage

```rust
extern crate termsize;

pub fn main() {
  termsize::get().map(|size| {
    println!("rows {} cols {}", size.rows, size.cols)
  });
}
```

Doug Tangren (softprops) 2015
