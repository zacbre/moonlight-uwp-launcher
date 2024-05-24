#![no_main]

#[cfg(windows)]
use crate::app::launch;

#[cfg(windows)]
mod app;

#[cfg(not(windows))]
fn main() {
    println!("This app is not supported on other platforms besides windows!");
}

#[cfg(windows)]
#[no_mangle]
fn main(_argc: i32, _argv: *const *const u8) {
    launch();
}
