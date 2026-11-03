#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Some(code) = aniwall_lib::cli::maybe_run() {
        std::process::exit(code);
    }
    aniwall_lib::run();
}
