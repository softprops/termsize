#![deny(missing_docs)]

//! termsize is a tiny crate that provides a simple
//! interface for setting and retrieving the current
//! terminal interface size
//!
//! ```rust
//!  extern crate termsize;
//!  termsize::get().map(|size| println!("width {} height {}", size.width, size.height));
//! ```
extern crate libc;

use libc::{ c_ulong, c_ushort, STDOUT_FILENO };
use libc::funcs::bsd44::ioctl;

const TIOCGWINSZ: c_ulong = 0x40087468;

/// A representation of the size of the current terminal
#[repr(C)]
#[derive(Debug)]
pub struct Size {
  /// number of rows
  pub rows: c_ushort,
  /// number of columns
  pub cols: c_ushort,
  x: c_ushort,
  y: c_ushort
}

impl Size {
  /// creates a new empty Size
  pub fn new() -> Size {
    Size { rows: 0, cols: 0, x: 0, y: 0 }
  }
}

/// Gets the current terminal size
pub fn get() -> Option<Size> {
  let s = Size::new();
  let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &s) };
  if r == 0 { Some(s) } else { None }
}
