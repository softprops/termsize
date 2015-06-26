//#[cfg(feature= "nightly")] extern crate libc;

extern crate libc;

use libc::{ c_ulong, c_ushort, STDOUT_FILENO };
use libc::funcs::bsd44::ioctl;

const TIOCGWINSZ: c_ulong = 0x40087468;
const TIOCSWINSZ: c_ulong = 2148037735;

#[repr(C)]
#[derive(Debug)]
pub struct Size {
  height: c_ushort,
  width: c_ushort,
  x: c_ushort,
  y: c_ushort
}

impl Size {
  pub fn new() -> Size {
    Size { height: 0, width: 0, x: 0, y: 0 }
  }
  pub fn height(&mut self, h: u16) {
    self.height = h
  }
  pub fn width(&mut self, w: u16) {
    self.width = w;
  }
}

pub fn set(s: &Size) -> bool {
  let r = unsafe { ioctl(STDOUT_FILENO, TIOCSWINSZ, s)};
  r == 0
}

pub fn get() -> Option<Size> {
  let mut s = Size::new();
  let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &s) };
  match r {
    0 => {
      Some(s)
    },
    _ => {
      None
    }
  }
}

#[test]
fn it_works() {
}
