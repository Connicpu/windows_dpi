# windows_dpi [![windows_dpi on crates.io](https://img.shields.io/crates/v/windows_dpi.svg)](https://crates.io/crates/windows_dpi) [![Build Status](https://travis-ci.org/Connicpu/windows_dpi.svg?branch=master)](https://travis-ci.org/Connicpu/windows_dpi)

```toml
[dependencies]
windows_dpi = "0.1"
```

```rust
extern crate windows_dpi;

fn main() {
    // Make sure the OS knows you're paying attention to DPI so it won't scale your window
    windows_dpi::enable_dpi();

    // Get the basic scaling factor so you can know how big to make your windows
    let mut scaling_factor = windows_dpi::desktop_dpi();

    let window = create_window(scaling_factor);
    // Now that you have your window, check what its real scaling factor is
    scaling_factor = unsafe { windows_dpi::get_dpi_for(window.platform_handle()) };

    // Do some drawing stuff
    draw_graphics(&window, scaling_factor);
}
```
