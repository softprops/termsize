#![deny(missing_docs)]

//! termsize is a tiny crate that provides a simple
//! interface for setting and retrieving the current
//! terminal interface size

extern crate libc;

use libc::{ c_ulong, c_ushort, STDOUT_FILENO };
use libc::funcs::bsd44::ioctl;

const TIOCGWINSZ: c_ulong = 0x40087468;
const TIOCSWINSZ: c_ulong = 2148037735;

/// A representation of the size of the current terminal
#[repr(C)]
#[derive(Debug)]
pub struct Size {
  height: c_ushort,
  width: c_ushort,
  x: c_ushort,
  y: c_ushort
}

impl Size {
  /// creates a new empty Size
  pub fn new() -> Size {
    Size { height: 0, width: 0, x: 0, y: 0 }
  }

  /// sets size height
  pub fn height(&mut self, h: u16) {
    self.height = h
  }

  /// sets size width
  pub fn width(&mut self, w: u16) {
    self.width = w;
  }
}

/// Sets the current terminal size
pub fn set(s: &Size) -> bool {
  let r = unsafe { ioctl(STDOUT_FILENO, TIOCSWINSZ, s) };
  r == 0
}

/// Gets the current terminal size
pub fn get() -> Option<Size> {
  let s = Size::new();
  let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &s) };
  if r == 0 { Some(s) } else { None }
}
