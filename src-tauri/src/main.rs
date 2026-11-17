// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .init();
    visca_camera_simulator_lib::run()
}
