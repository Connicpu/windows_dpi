#![cfg(windows)]

extern crate libloading;
extern crate winapi;
extern crate user32;
extern crate gdi32;
#[macro_use]
extern crate lazy_static;

use libloading::{Library, Symbol};
use std::ptr;
use winapi::{BOOL, HRESULT, DWORD, HWND, HMONITOR, UINT, LOGPIXELSX};

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

pub fn get_dpi_for(hwnd: HWND) -> f32 {
    // This will be Some on a system with Windows 8.1 or newer
    if let Some(get_dpi_for) = (*GET_DPI_FOR_MONITOR).as_ref() {
        unsafe {
            let hmon = user32::MonitorFromWindow(hwnd, 0 /* EFFECTIVE_DPI */);
            let mut dpix = 96;
            let mut dpiy = 96;
            get_dpi_for(hmon, 1 /* DEFAULTTOPRIMARY */, &mut dpix, &mut dpiy);
            dpix as f32 / 96.0
        }
    // On systems without ShCore, there is only a global DPI anyways
    } else {
        unsafe {
            let hdc = user32::GetDC(ptr::null_mut());
            let dpi = gdi32::GetDeviceCaps(hdc, LOGPIXELSX);
            dpi as f32 / 96.0
        }
    }
}
