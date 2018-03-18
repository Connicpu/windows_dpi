#![allow(unused_imports)]

#[cfg(target_os = "windows")]
extern crate libloading;
#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[macro_use]
extern crate lazy_static;

extern crate libc;

#[cfg(target_os = "windows")]
pub use windows::enable_dpi;
#[cfg(target_os = "windows")]
pub use windows::desktop_dpi;
#[cfg(target_os = "windows")]
pub use windows::get_dpi_for;

#[cfg(target_os = "linux")]
pub use linux::enable_dpi;
#[cfg(target_os = "linux")]
pub use linux::desktop_dpi;
#[cfg(target_os = "linux")]
pub use linux::get_dpi_for;

#[cfg(target_os = "macos")]
pub use macos::enable_dpi;
#[cfg(target_os = "macos")]
pub use macos::desktop_dpi;
#[cfg(target_os = "macos")]
pub use macos::get_dpi_for;

#[cfg(target_os = "windows")]
pub mod windows {
    use std::ptr;

    use libloading::{Library, Symbol};
    use winapi::shared::minwindef::*;
    use winapi::shared::winerror::*;
    use winapi::shared::windef::*;
    use winapi::um::{wingdi, winuser};

    type TSetProcDpi = unsafe extern "system" fn(DWORD) -> HRESULT;
    type TGetMonDpi = unsafe extern "system" fn(HMONITOR, DWORD, *mut UINT, *mut UINT);
    type TEnableProcDpi = unsafe extern "system" fn() -> BOOL;

    lazy_static! {
        // This library may not exist on all supported playforms
        static ref SHCORE: Option<Library> = {
            Library::new("ShCore.dll").ok()
        };

        // This library always exists on supported platforms
        static ref USER32: Library = {
            Library::new("User32.dll").unwrap()
        };

        // This function may not exist on all supported playforms
        static ref SET_PROCESS_DPI_AWARENESS: Option<Symbol<'static, TSetProcDpi>> = {
            unsafe {
                SHCORE.as_ref().and_then(|shcore| shcore.get(b"SetProcessDpiAwareness\0").ok())
            }
        };

        // This function may not exist on all supported playforms
        static ref GET_DPI_FOR_MONITOR: Option<Symbol<'static, TGetMonDpi>> = {
            unsafe {
                SHCORE.as_ref().and_then(|shcore| shcore.get(b"GetDpiForMonitor\0").ok())
            }
        };

        // This function always exists on supported platforms, it's just not defined
        // by winapi at the moment
        static ref SET_PROCESS_DPI_AWARE: Symbol<'static, TEnableProcDpi> = {
            unsafe {
                USER32.get(b"SetProcessDPIAware\0").unwrap()
            }
        };
    }

    /// Tells the OS this program is aware of DPI and windows will not be
    /// scaled by the OS. Ensure your program uses the DPI values, especially
    /// checking per-window and listening for the WM_DPI_CHANGED message. Please
    /// be a good high-DPI citizen :)
    pub fn enable_dpi() {
        if let Some(set_awareness) = (*SET_PROCESS_DPI_AWARENESS).as_ref() {
            unsafe {
                set_awareness(2);
            }
        } else {
            unsafe {
                SET_PROCESS_DPI_AWARE();
            }
        }
    }

    /// Returns the default DPI of the primary desktop
    pub fn desktop_dpi() -> f32 {
        unsafe { get_dpi_for(ptr::null_mut()) }
    }

    /// Returns the DPI of the desktop on which a window currently resides.
    /// null may be passed, in which case it will perform the same behavior
    /// as desktop_dpi().
    pub unsafe fn get_dpi_for(hwnd: HWND) -> f32 {
        // This will be Some on a system with Windows 8.1 or newer
        if let (true, Some(get_dpi_for)) =
            (hwnd != ptr::null_mut(), (*GET_DPI_FOR_MONITOR).as_ref())
        {
            let hmon = winuser::MonitorFromWindow(hwnd, 0 /* EFFECTIVE_DPI */);
            let mut dpix = 96;
            let mut dpiy = 96;
            get_dpi_for(hmon, 1 /* DEFAULTTOPRIMARY */, &mut dpix, &mut dpiy);
            dpix as f32 / 96.0
        // On systems without ShCore, there is only a global DPI anyways
        } else {
            let hdc = winuser::GetDC(ptr::null_mut());
            let dpi = wingdi::GetDeviceCaps(hdc, wingdi::LOGPIXELSX);
            dpi as f32 / 96.0
        }
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    /// Stub function, I don't believe any linux desktops do OS-overridden scaling
    pub fn enable_dpi() {}
    /// Stub function, always returns 1.0
    pub fn desktop_dpi() -> f32 {
        1.0
    }
    /// Stub function, always returns 1.0
    pub unsafe fn get_dpi_for(_window: *mut ::libc::c_void) -> f32 {
        1.0
    }
}

#[cfg(target_os = "macos")]
pub mod macos {
    use cocoa::base::{class, id, nil, BOOL, YES};
    use cocoa::appkit::NSScreen;

    /// Stub function, I don't know if this is necessary on mac of if it's even
    /// possible programatically. If it is possible, contributions would be happily
    /// accepted.
    pub fn enable_dpi() {}

    /// Gets the mainScreen DPI. Limitation: This will always be the screen
    /// which currently has keyboard focus. Use [get_dpi_for] if you want the
    /// dpi of a specific screen or window.
    pub fn desktop_dpi() -> f32 {
        unsafe {
            let screen = NSScreen::mainScreen(nil);
            let scale = screen.backingScaleFactor();
            scale as f32
        }
    }

    /// Gets the DPI for a specific surface. You can use NSScreen or NSWindow, but any
    /// object that respondes to `-(CGFloat)backingScaleFactor` may be passed. If the
    /// passed object is nil, or does not respond to `backingScaleFactor`, [desktop_dpi]
    /// is called instead.
    pub unsafe fn get_dpi_for(object: id) -> f32 {
        if window == nil {
            return desktop_dpi();
        }

        let is_window: BOOL = msg_send![window, respondsToSelector: sel!(backingScaleFactor)];
        if is_window == YES {
            let scale = window.backingScaleFactor();
            scale as f32
        } else {
            desktop_dpi()
        }
    }
}
