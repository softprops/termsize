# termsize

[![Build Status](https://travis-ci.org/softprops/termsize.svg)](https://travis-ci.org/softprops/termsize)

> because terminal size matters

```rust
extern crate termize;

pub fn main() {
  let size = termsize::get().unwrap();
  println!("{} rows {} columns", size.rows, size.colums);
}
```

Doug Tangren (softprops) 2015
