extern crate windows_dpi;

#[test]
fn desktop_dpi() {
    windows_dpi::desktop_dpi();
}

#[test]
fn enable_dpi() {
    windows_dpi::enable_dpi();
}

#[test]
fn get_dpi_for_null() {
    unsafe {
        windows_dpi::get_dpi_for(0 as *mut _);
    }
}

