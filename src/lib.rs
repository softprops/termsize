#![deny(missing_docs)]

//! termsize is a tiny crate that provides a simple
//! interface for retrieving the current
//! [terminal interface](http://www.manpagez.com/man/4/tty/) size
//!
//! ```rust
//!  extern crate termsize;
//!  termsize::get().map(|size| println!("width {} height {}", size.width, size.height));
//! ```

/// Size
#[derive(Debug)]
pub struct Size {
    /// number of rows
    pub rows: u16,
    /// number of columns
    pub cols: u16
}

#[cfg(unix)]
mod nix;
#[cfg(unix)]
pub use self::nix::get;

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use self::win::get;
