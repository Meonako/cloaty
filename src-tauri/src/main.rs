// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::collections::HashMap;

// use std::sync::Mutex;
use tokio::sync::Mutex;

pub struct State {
    pg: Mutex<HashMap<String, sqlx::PgPool>>,
    mysql: Mutex<HashMap<String, sqlx::MySqlPool>>,
}

fn main() {
    let state = State {
        pg: Mutex::new(HashMap::new()),
        mysql: Mutex::new(HashMap::new()),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::connect::connect_db,
            commands::disconnect::disconnect_db,
            commands::get_table::get_tables,
            commands::get_data::get_data,
            commands::get_settings::get_settings,
            commands::save_settings::save_settings,
            commands::test_connection::test_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
