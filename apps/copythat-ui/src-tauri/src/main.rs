// Hide the console window on Windows release builds; debug builds keep it
// open so log output is visible.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    copythat_ui_lib::run()
}
