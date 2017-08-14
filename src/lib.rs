#[cfg(windows)]
extern crate libloading;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate gdi32;
#[macro_use]
extern crate lazy_static;
extern crate libc;

#[cfg(windows)]
pub use windows::enable_dpi;
#[cfg(windows)]
pub use windows::desktop_dpi;
#[cfg(windows)]
pub use windows::get_dpi_for;

#[cfg(linux)]
pub use linux::enable_dpi;
#[cfg(linux)]
pub use linux::desktop_dpi;
#[cfg(linux)]
pub use linux::get_dpi_for;

#[cfg(macos)]
pub use macos::enable_dpi;
#[cfg(macos)]
pub use macos::desktop_dpi;
#[cfg(macos)]
pub use macos::get_dpi_for;

#[cfg(windows)]
pub mod windows {
    use libloading::{Library, Symbol};
    use std::ptr;
    use winapi::{BOOL, HRESULT, DWORD, HWND, HMONITOR, UINT, LOGPIXELSX};
    use {user32, gdi32};

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

    pub fn desktop_dpi() -> f32 { 
        unsafe { get_dpi_for(ptr::null_mut()) }
    }

    pub unsafe fn get_dpi_for(hwnd: HWND) -> f32 {
        // This will be Some on a system with Windows 8.1 or newer
        if let (true, Some(get_dpi_for)) = (hwnd != ptr::null_mut(), (*GET_DPI_FOR_MONITOR).as_ref()) {
            let hmon = user32::MonitorFromWindow(hwnd, 0 /* EFFECTIVE_DPI */);
            let mut dpix = 96;
            let mut dpiy = 96;
            get_dpi_for(hmon, 1 /* DEFAULTTOPRIMARY */, &mut dpix, &mut dpiy);
            dpix as f32 / 96.0
        // On systems without ShCore, there is only a global DPI anyways
        } else {
            let hdc = user32::GetDC(ptr::null_mut());
            let dpi = gdi32::GetDeviceCaps(hdc, LOGPIXELSX);
            dpi as f32 / 96.0
        }
    }
}

#[cfg(linux)]
pub mod linux {
    pub fn enable_dpi() {}
    pub fn desktop_dpi() -> f32 { 1.0 }
    pub unsafe fn get_dpi_for(window: *mut ::libc::c_void) -> f32 {}
}

#[cfg(macos)]
pub mod macos {
    pub fn enable_dpi() {}
    pub fn desktop_dpi() -> f32 { 1.0 }
    pub unsafe fn get_dpi_for(window: *mut ::libc::c_void) -> f32 {}
}
