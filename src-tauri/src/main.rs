// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;

use settings::{load_settings, save_settings, test_token};
use std::fmt::Debug;
use std::sync::Mutex;
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