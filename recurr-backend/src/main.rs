#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use recurr::AppBuilder;

pub fn main() {
    AppBuilder::new().run();
}
