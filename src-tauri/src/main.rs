// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod service_advisory;
mod settings;
mod stop_info;
mod stop_schedules;
mod trip_planner;

use google_maps_api_client::GoogleMapsClient;
use log::error;
use settings::{
    get_settings, load_settings, reset_settings, save_settings, test_google_token, test_token,
    Settings,
};
use std::fmt::Debug;
use tauri::async_runtime::Mutex;
use tauri_plugin_log::LogTarget;
use transit_api_client::TransitClient;

/// Get access to the global TransitClient of the application.
///
/// Just add `client: State<'_, ClientState>` as a parameter to the function
/// and tauri will automatically give you access to the managed client.
///
/// Any function that uses the client **has to return a `Result` and be async!**
///
/// ```
/// #[tauri::command]
/// pub async fn client_usage(client: State<'_, ClientState>) -> Result<(), &'static str> {
///     let client = client.0.lock().await;
///     // --snip--
/// }
/// ```
pub struct ClientState(
    /// The actual client
    pub Mutex<TransitClient>,
);

/// Get access to the global settings of the application.
///
/// Just add `settings: State<'_, SettingsState>` as a parameter to the function
/// and tauri will automatically give you access to the managed state.
///
/// Any function that uses the settings **has to return a `Result` and be async!**
///
/// ```no_run
/// use tauri::State;
/// use crate::SettingsState;
///
/// #[tauri::command]
/// pub async fn settings_usage(settings: State<'_, SettingsState>) -> Result<(), ()> {
///     let settings = settings.0.lock().await;
///     // --snip--
/// }
/// ```
pub struct SettingsState(pub Mutex<Settings>);

/// Get access to the global settings of the application.
///
/// Just add `maps_client: State<'_, GoogleMapsState>` as a parameter to the function
/// and tauri will automatically give you access to the managed state.
///
/// Any function that uses the settings **has to return a `Result` and be async!**
///
/// ```no_run
/// use tauri::State;
/// use crate::GoogleMapsState;
///
/// #[tauri::command]
/// pub async fn google_maps_usage(maps_client: State<'_, GoogleMapsState>) -> Result<(), ()> {
///     let maps_client = maps_client.0.lock().await;
///     // --snip--
/// }
/// ```
pub struct GoogleMapsState(pub Mutex<GoogleMapsClient>);

fn main() {
    let user_settings = load_settings().unwrap_or_default();
    println!("[Before Init]: Loaded user settings: {user_settings:?}");

    tauri::Builder::default()
        .manage(ClientState(Mutex::new(TransitClient::new(String::from(
            &user_settings.api_key,
        )))))
        .manage(GoogleMapsState(Mutex::new(GoogleMapsClient::new(
            String::from(&user_settings.google_api_key),
        ))))
        .manage(SettingsState(Mutex::new(user_settings)))
        .invoke_handler(tauri::generate_handler![
            // Settings
            save_settings,
            get_settings,
            reset_settings,
            test_token,
            test_google_token,
            service_advisory::service_advisorie_html,
            stop_schedules::stop_schedule,
            stop_info::stop_info,
            // Trip Planner
            trip_planner::trip_planner,
            trip_planner::search_locations,
            trip_planner::google_trip_planner,
        ])
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::LogDir,
                    LogTarget::Stdout,
                    //LogTarget::Webview,
                ])
                .build(),
        )
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                use tauri::Manager;
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Format an error nicely and print it out to the console.
///
/// # Arguments
///
/// * error: The error that will be printed with debug format
/// * message: The message that will be printed before the error. This is also the return value
pub fn error_string<'a, E: Debug>(error: &E, message: &'a str) -> &'a str {
    error!("{message}: {error:?}");
    message
}
