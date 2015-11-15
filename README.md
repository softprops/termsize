# termsize

[![Build Status](https://travis-ci.org/softprops/termsize.svg)](https://travis-ci.org/softprops/termsize)

> because terminal size matters

Termsize is a rust crate providing a multi-platform interface for resolving
you're terminal's current size in rows and columns. On most unix systems, this is similar invoking the [stty(1)](http://man7.org/linux/man-pages/man1/stty.1.html) program, requesting the terminal size.

## api docs

Find them [here](https://softprops.github.com/termsize)

## usage

Termize provides one function, `get`, which returns a `termsize::Size` struct
exposing two fields: `rows` and `cols`.

```rust
extern crate termsize;

pub fn main() {
  termsize::get().map(|size| {
    println!("rows {} cols {}", size.rows, size.cols)
  });
}
```

Doug Tangren (softprops) 2015
