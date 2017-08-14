#![deny(missing_docs)]

//! termsize is a tiny crate that provides a simple
//! interface for retrieving the current
//! [terminal interface](http://www.manpagez.com/man/4/tty/) size
//!
//! ```rust
//! extern crate termsize;
//! termsize::get().map(|size| {
//!   println!("rows {} cols {}", size.rows, size.cols)
//! });
//! ```

/// Size
#[derive(Debug)]
pub struct Size {
    /// number of rows
    pub rows: u16,
    /// number of columns
    pub cols: u16,

    /// width in pixels
    pub width: Option<u16>,

    /// height in pixels
    pub height: Option<u16>,
}

#[cfg(unix)]
mod nix;
#[cfg(unix)]
pub use self::nix::get;

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use self::win::get;

#[cfg(target_os = "redox")]
mod redox;
#[cfg(target_os = "redox")]
pub use self::redox::get;

#[cfg(test)]
mod tests {
    use super::get;
    #[test]
    fn test_get() {
        assert!(get().is_some())
    }
}
