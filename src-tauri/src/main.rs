// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;
mod stop_schedules;

use settings::{load_settings, save_settings, Settings, test_token};
use std::fmt::Debug;
use transit_api_client::TransitClient;
use stop_schedules::stop_schedule;
use tauri::async_runtime::Mutex;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub struct ClientState(pub Mutex<TransitClient>);
pub struct SettingsState(pub Mutex<Settings>);

fn main() {
    let user_settings = load_settings().unwrap_or_default();

    tauri::Builder::default()
        .manage(ClientState(Mutex::new(TransitClient::new(String::from(&user_settings.api_key)))))
        .manage(SettingsState(Mutex::new(user_settings)))
        .invoke_handler(tauri::generate_handler![
            greet,
            save_settings,
            load_settings,
            test_token,
            stop_schedule
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn error_string<'a, E: Debug>(error: &E, message: &'a str) -> &'a str {
    eprintln!("{message}: {error:?}");
    message
}
