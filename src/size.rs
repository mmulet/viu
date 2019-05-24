extern crate libc;

#[cfg(not(target_os = "wasi"))]
use libc::ioctl;

#[cfg(not(target_os = "wasi"))]
use libc::{c_ushort, STDOUT_FILENO, TIOCGWINSZ};

#[cfg(not(target_os = "wasi"))]
#[repr(C)]
struct winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}

pub fn get_size() -> Result<(u32, u32), &'static str> {
    #[cfg(not(target_os = "wasi"))]
    {
        let w = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };
        match r {
            //1 less row because the prompt after executing the command the prompt will take a line
            0 => Ok((u32::from(w.ws_col), u32::from(w.ws_row) - 1)),
            _ => Err("Could not get terminal size, using default width..."),
        }
    }

    #[cfg(target_os = "wasi")]
    {
        Err("Could not get terminal size, using default width...")
    }
}
