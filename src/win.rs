extern crate winapi;
extern crate kernel32;

use self::super::Size;
use std::ptr;

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    //http://rosettacode.org/wiki/Terminal_control/Dimensions#Windows
    use self::winapi::HANDLE;
    use self::kernel32::{self, GetStdHandle, GetConsoleScreenBufferInfo, GetLastError};
    use self::winapi::{
        self,
        CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
        STD_OUTPUT_HANDLE, INVALID_HANDLE_VALUE
    };
    let name = b"CONOUT$\0";
    let handle: HANDLE = unsafe {
        kernel32::CreateFileA(
            name.as_ptr() as *const i8,
            winapi::GENERIC_READ | winapi::GENERIC_WRITE,
            winapi::FILE_SHARE_WRITE,
            ptr::null_mut(),
            winapi::OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };
    println!("handle {:?}", handle);
    if handle == INVALID_HANDLE_VALUE {
        println!("handle was invalid");
        return None
    }
    let zc = COORD { X: 0, Y: 0 };
    // screen buffer info is a container of console display information
    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms682093(v=vs.85).aspx
    /*CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: zc.clone(),
        dwCursorPosition: zc.clone(),
        wAttributes: 0,
        srWindow: SMALL_RECT { Left:0, Top: 0, Right: 0, Bottom: 0 },
        dwMaximumWindowSize: zc
};*/
    let info = unsafe {
        // https://msdn.microsoft.com/en-us/library/windows/desktop/ms683171(v=vs.85).aspx
        let mut info = ::std::mem::uninitialized();
        let result = GetConsoleScreenBufferInfo(handle, &mut info);
        println!("result was {}", result);
        if result == 0 {
            println!("last error was {:?}", GetLastError());
            None
        } else {
            Some(info)
        }
    };
    println!("info {:?}", info);
    info.map(|inf| {
        Size {
            rows: (inf.srWindow.Bottom - inf.srWindow.Top + 1) as u16,
            cols: (inf.srWindow.Right - inf.srWindow.Left + 1) as u16
        }
    })
}
