use self::super::Size;

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    termion::terminal_size().ok().map(|(cols, rows)| {
        Size { rows, cols }
    })
}
