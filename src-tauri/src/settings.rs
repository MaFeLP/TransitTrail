use google_maps_api_client::GoogleMapsClient;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

use crate::{error_string, ClientState, GoogleMapsState, SettingsState};
use serde::{Deserialize, Serialize};
use tauri::{api::path::config_dir, State};
use transit_api_client::prelude::{TransitClient, Usage};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Settings {
    pub api_key: String,
    pub google_api_key: String,
    pub min_waiting_time: u32,
    pub max_waiting_time: u32,
    pub max_transfers: u32,
    pub max_walking_time: u32,
    pub walking_speed: f32,
    pub search_interval: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            google_api_key: String::new(),
            min_waiting_time: 0,
            max_waiting_time: 60,
            max_transfers: 10,
            walking_speed: 4.0,
            max_walking_time: 30,

            search_interval: 5000,
        }
    }
}

#[tauri::command]
pub async fn save_settings(
    new_settings: Settings,
    client: State<'_, ClientState>,
    maps_client: State<'_, GoogleMapsState>,
    user_settings: State<'_, SettingsState>,
) -> Result<(), &'static str> {
    let dir = config_dir().ok_or("failed to get config directory")?;
    let file_path = dir.join("wpg-transit-client").join("settings.toml");

    let toml_config = toml::to_string(&new_settings)
        .map_err(|why| error_string(&why, "Could not serialize settings to toml"))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)
        .map_err(|why| error_string(&why, "Could not find settings.toml file"))?;

    file.write_all(toml_config.as_bytes())
        .map_err(|why| error_string(&why, "Could not write to settings.toml file"))?;

    *client.0.lock().await = TransitClient::new(new_settings.api_key.clone());
    *maps_client.0.lock().await = GoogleMapsClient::new(new_settings.google_api_key.clone());
    *user_settings.0.lock().await = new_settings;

    Ok(())
}

pub fn load_settings() -> Result<Settings, &'static str> {
    let dir = config_dir()
        .ok_or("Failed to get config directory")?
        .join("wpg-transit-client");
    fs::create_dir_all(&dir)
        .map_err(|why| error_string(&why, "Failed to create config directory"))?;
    let file_path = dir.join("settings.toml");

    if !file_path.exists() {
        return Ok(Settings::default());
    }

    let mut file = File::open(&file_path)
        .map_err(|why| error_string(&why, "Could not find settings.toml file"))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|why| error_string(&why, "Could not read settings.toml file"))?;
    toml::from_str(&buf).map_err(|why| error_string(&why, "Could not parse settings.toml file"))
}

#[tauri::command]
pub async fn get_settings(settings: State<'_, SettingsState>) -> Result<Settings, ()> {
    let settings = settings.0.lock().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn reset_settings(settings: State<'_, SettingsState>) -> Result<(), ()> {
    let old_key = settings.0.lock().await.api_key.clone();
    *settings.0.lock().await = Settings::default();
    settings.0.lock().await.api_key = old_key;
    Ok(())
}

#[tauri::command]
pub async fn test_token(token: String) -> Result<(), &'static str> {
    let client = TransitClient::new(token);
    match client.stop_info(10064, Usage::Normal).await {
        Ok(_) => Ok(()),
        Err(why) => Err(error_string(&why, "Error while testing connection")),
    }
}

#[tauri::command]
pub async fn test_google_token(token: String) -> Result<(), &'static str> {
    let client = GoogleMapsClient::new(token);
    match client.geocode("300 Portage Ave, Winnipeg, MB").await {
        Ok(_) => Ok(()),
        Err(why) => Err(error_string(&why, "Error while testing connection")),
    }
}
