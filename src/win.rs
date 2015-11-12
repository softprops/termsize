extern crate winapi;
extern crate kernel32;

use self::super::Size;

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    //http://rosettacode.org/wiki/Terminal_control/Dimensions#Windows
    use self::winapi::{DWORD, HANDLE};
    use self::kernel32::{GetStdHandle, GetConsoleScreenBufferInfo};
    use self::winapi::{
        CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
        STD_INPUT_HANDLE, STD_OUTPUT_HANDLE
    };
    let stdout: HANDLE = unsafe {
        GetStdHandle(STD_OUTPUT_HANDLE)
    };
    let zc = COORD { X: 0, Y: 0 };
    let mut info = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: zc.clone(),
        dwCursorPosition: zc.clone(),
        wAttributes: 0,
        srWindow: SMALL_RECT { Left:0, Top: 0, Right: 0, Bottom: 0 },
        dwMaximumWindowSize: zc
    };
    let success: bool = unsafe {
        GetConsoleScreenBufferInfo(stdout, &mut info) != 0
    };
    if success {
        Some(
            Size {
                rows: (info.srWindow.Bottom - info.srWindow.Top + 1) as u16,
                cols: (info.srWindow.Right - info.srWindow.Left + 1) as u16
            }
        )
    } else {
        None
    }
}
