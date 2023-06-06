// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;
mod service_advisory;

use settings::{load_settings, save_settings, test_token};
use std::fmt::Debug;
use tauri::async_runtime::Mutex;
use transit_api_client::TransitClient;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub struct ClientState (
    pub Mutex<TransitClient>
);

fn main() {
    let settings = load_settings().expect("Could not load the settings!");

    tauri::Builder::default()
        .manage(ClientState(Mutex::new(TransitClient::new(settings.api_key))))
        .invoke_handler(tauri::generate_handler![
            greet,
            service_advisory::service_advisorie_html,
            save_settings,
            load_settings,
            test_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn error_string<'a, E: Debug>(error: &E, message: &'a str) -> &'a str {
    eprintln!("{message}: {error:?}");
    message
}