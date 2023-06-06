// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;
mod service_advisory;

use log::{info, error};
use settings::{load_settings, get_settings, save_settings, Settings, test_token, reset_settings};
use std::fmt::Debug;
use tauri::async_runtime::Mutex;
use tauri_plugin_log::LogTarget;
use transit_api_client::TransitClient;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub struct ClientState (
    pub Mutex<TransitClient>,
);

pub struct SettingsState (
    pub Mutex<Settings>
);

fn main() {
    let user_settings = load_settings().unwrap_or_default();
    info!("Loaded user settings: {user_settings:?}");

    tauri::Builder::default()
        .manage(ClientState(Mutex::new(TransitClient::new(user_settings.api_key.clone()))))
        .manage(SettingsState(Mutex::new(user_settings)))
        .invoke_handler(tauri::generate_handler![
            greet,
            service_advisory::service_advisorie_html,
            save_settings,
            get_settings,
            reset_settings,
            test_token,
        ])
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            //LogTarget::Webview,
        ]).build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn error_string<'a, E: Debug>(error: &E, message: &'a str) -> &'a str {
    error!("{message}: {error:?}");
    message
}
