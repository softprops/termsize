extern crate libc;
extern crate atty;

use self::super::Size;
use self::libc::{STDOUT_FILENO, winsize, TIOCGWINSZ};
use self::libc::ioctl;

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    if atty::isnt() {
        return None;
    }
    let mut us = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut us) };
    if r == 0 {
        Some(Size {
            rows: us.ws_row,
            cols: us.ws_col,

            width: if us.ws_xpixel == 0 { None } else { Some(us.ws_xpixel) },
            height: if us.ws_ypixel == 0 { None } else { Some(us.ws_ypixel) },
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::process::{Command, Output, Stdio};
    use super::super::Size;
    use super::get;

    #[cfg(target_os = "macos")]
    fn stty_size() -> Output {
        Command::new("stty")
            .arg("-f")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
    }

    #[cfg(not(target_os = "macos"))]
    fn stty_size() -> Output {
        Command::new("stty")
            .arg("-F")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
    }

    #[test]
    fn test_shell() {
        let output = stty_size();
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        let mut data = stdout.split_whitespace();
        let rs = data.next().unwrap().parse::<u16>().unwrap();
        let cs = data.next().unwrap().parse::<u16>().unwrap();
        if let Some(Size { rows, cols, .. }) = get() {
            assert_eq!(rows, rs);
            assert_eq!(cols, cs);
        }
    }
}
