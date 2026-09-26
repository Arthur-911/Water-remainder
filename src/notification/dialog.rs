use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Shows a native Windows topmost pop-up dialog box that appears over all windows.
pub fn show_dialog(title: &str, message: &str) {
    let wide_title: Vec<u16> = OsStr::new(title).encode_wide().chain(Some(0)).collect();
    let wide_msg: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();

    std::thread::spawn(move || unsafe {
        #[link(name = "user32")]
        unsafe extern "system" {
            fn MessageBoxW(
                hwnd: *mut std::ffi::c_void,
                lpText: *const u16,
                lpCaption: *const u16,
                uType: u32,
            ) -> i32;
        }
        const MB_OK: u32 = 0x00000000;
        const MB_ICONINFORMATION: u32 = 0x00000040;
        const MB_TOPMOST: u32 = 0x00004000;
        const MB_SETFOREGROUND: u32 = 0x00010000;

        MessageBoxW(
            std::ptr::null_mut(),
            wide_msg.as_ptr(),
            wide_title.as_ptr(),
            MB_OK | MB_ICONINFORMATION | MB_TOPMOST | MB_SETFOREGROUND,
        );
    });
}
