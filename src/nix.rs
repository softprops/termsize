extern crate libc;

use std::io::IsTerminal;

use self::{
    super::Size,
    libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ},
};

/// A representation of the size of the current terminal
#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
    /// number of rows
    pub rows: c_ushort,
    /// number of columns
    pub cols: c_ushort,
    x: c_ushort,
    y: c_ushort,
}

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    if !std::io::stdout().is_terminal() {
        return None;
    }
    let mut us = UnixSize {
        rows: 0,
        cols: 0,
        x: 0,
        y: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut us) };
    if r == 0 {
        Some(Size {
            rows: us.rows,
            cols: us.cols,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Size, get};
    use std::process::{Command, Output, Stdio};

    #[cfg(target_os = "macos")]
    fn stty_size() -> Output {
        Command::new("stty")
            .arg("-f")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .expect("expected stty output")
    }

    #[cfg(not(target_os = "macos"))]
    fn stty_size() -> Output {
        Command::new("stty")
            .arg("-F")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .expect("expected stty output")
    }

    #[test]
    fn test_shell() {
        let output = stty_size();
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).expect("expected utf8");
        let mut data = stdout.split_whitespace();
        let rs = data
            .next()
            .expect("expected row")
            .parse::<u16>()
            .expect("expected u16 col");
        let cs = data
            .next()
            .expect("expected col")
            .parse::<u16>()
            .expect("expected u16 col");
        if let Some(Size { rows, cols }) = get() {
            assert_eq!(rows, rs);
            assert_eq!(cols, cs);
        }
    }
}
