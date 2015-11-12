extern crate libc;
extern crate atty;

use self::super::Size;
use self::libc::{c_ulong, c_ushort, STDOUT_FILENO};
use self::libc::ioctl;

#[cfg(not(target_os = "macos"))]
const TIOCGWINSZ: c_ulong = 0x00005413;

#[cfg(target_os = "macos")]
const TIOCGWINSZ: c_ulong = 0x40087468;

/// A representation of the size of the current terminal
#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
  /// number of rows
  pub rows: c_ushort,
  /// number of columns
  pub cols: c_ushort,
  x: c_ushort,
  y: c_ushort
}


/// Gets the current terminal size
pub fn get() -> Option<Size> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    if atty::isnt() {
        return None;
    }
    let us = UnixSize { rows: 0, cols: 0, x: 0, y: 0 };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &us) };
    if r == 0 { Some(Size { rows: us.rows, cols: us.cols }) } else { None }
}
