#![cfg(target_os = "macos")]

extern crate windows_dpi;

#[macro_use]
extern crate objc;
extern crate cocoa;

use cocoa::base::nil;
use cocoa::appkit::NSScreen;
use cocoa::foundation::NSUInteger;

#[test]
fn refcount_looks_right() {
    unsafe {
        let main_screen = NSScreen::mainScreen(nil);
        let refcount: NSUInteger = msg_send![main_screen, retainCount];

        for _ in 0..100 {
            windows_dpi::desktop_dpi();

            assert_eq!(refcount, msg_send![main_screen, retainCount]);
        }
    }
}
