#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use snowflake_app::app;

pub fn main() {
    app::launch();
}
