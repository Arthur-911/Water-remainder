#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};

static DIALOG_ACTIVE: AtomicBool = AtomicBool::new(false);

#[cfg(windows)]
#[link(name = "user32")]
unsafe extern "system" {
    fn MessageBoxW(
        hwnd: *mut std::ffi::c_void,
        lpText: *const u16,
        lpCaption: *const u16,
        uType: u32,
    ) -> i32;
}

#[cfg(windows)]
const MB_OK: u32 = 0x00000000;
#[cfg(windows)]
const MB_ICONINFORMATION: u32 = 0x00000040;
#[cfg(windows)]
const MB_TOPMOST: u32 = 0x00004000;
#[cfg(windows)]
const MB_SETFOREGROUND: u32 = 0x00010000;

struct DialogGuard;

impl Drop for DialogGuard {
    fn drop(&mut self) {
        DIALOG_ACTIVE.store(false, Ordering::SeqCst);
    }
}

/// Shows a native Windows topmost pop-up dialog box that appears over all windows.
/// If a dialog is already currently displayed on screen, ignores duplicate invocations to avoid piling windows.
pub fn show_dialog(title: &str, message: &str) {
    if DIALOG_ACTIVE.swap(true, Ordering::SeqCst) {
        // A modal dialog is already open and awaiting user response
        return;
    }

    #[cfg(windows)]
    {
        let wide_title: Vec<u16> = OsStr::new(title).encode_wide().chain(Some(0)).collect();
        let wide_msg: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();

        std::thread::spawn(move || {
            let _guard = DialogGuard;
            unsafe {
                MessageBoxW(
                    std::ptr::null_mut(),
                    wide_msg.as_ptr(),
                    wide_title.as_ptr(),
                    MB_OK | MB_ICONINFORMATION | MB_TOPMOST | MB_SETFOREGROUND,
                );
            }
        });
    }

    #[cfg(not(windows))]
    {
        let _guard = DialogGuard;
        println!("[Dialog Stub] {} - {}", title, message);
    }
}
