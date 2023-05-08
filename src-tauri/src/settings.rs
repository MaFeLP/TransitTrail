use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

use crate::{ClientState, error_string};
use serde::{Deserialize, Serialize};
use tauri::api::path::config_dir;
use tauri::{Runtime, State};
use transit_api_client::prelude::{TransitClient, Usage};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub api_key: String,
    pub walking_distance: i32,
    pub waiting_time: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            walking_distance: 1000,
            waiting_time: 15,
        }
    }
}

#[tauri::command]
pub fn save_settings(
    api_key: &str,
    walking_distance: &str,
    waiting_time: &str,
    client: State<ClientState>,
) -> Result<(), &'static str> {
    let dir = config_dir().ok_or("failed to get config directory")?;
    let file_path = dir.join("wpg-transit-client").join("settings.toml");

    let settings = Settings {
        api_key: api_key.to_string(),
        walking_distance: {
            if walking_distance == "" {
                Settings::default().walking_distance
            } else {
                walking_distance
                    .parse()
                    .map_err(|why|
                        error_string(
                            &why,
                            "Could not parse field `walking_distance` in settings.toml (not of type `i32`"
                        ))?
            }
        },
        waiting_time: {
            if waiting_time == "" {
                Settings::default().waiting_time
            } else {
                waiting_time.parse().map_err(|why| {
                    error_string(
                        &why,
                        "Could not parse field `waiting_time` in settings.toml (not of type `i32`",
                    )
                })?
            }
        },
    };

    let toml_config = toml::to_string(&settings).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)
        .map_err(|why| error_string(&why, "Could not find settings.toml file"))?;

    file.write_all(&toml_config.as_bytes())
        .map_err(|why| error_string(&why, "Could not write to settings.toml file"))?;

    *client.0.lock().unwrap() = TransitClient::new(String::from(api_key));

    Ok(())
}

#[tauri::command]
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
    toml::from_str(&buf)
        .map_err(|why| error_string(&why, "Could not parse settings.toml file"))
}

#[tauri::command]
pub async fn test_token<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    token: String,
) -> Result<(), &'static str> {
    let client = TransitClient::new(token);
    match client.stop_info(10064, Usage::Normal).await {
        Ok(_) => Ok(()),
        Err(why) => Err(error_string(&why, "Error while testing connection")),
    }
}

#[allow(dead_code)]
#[tauri::command]
pub fn login_to_api<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> tauri::Result<()> {
    let window = tauri::WindowBuilder::new(
        &app,
        "winnipeg-transit-api-login",
        tauri::WindowUrl::App("https://api.winnipegtransit.com/".into()),
    )
    .title("Log In with Winnipeg Transit API")
    .enable_clipboard_access()
    //        .initialization_script(r#"console.log('Hello from the login window!'); window.addEventListener('load', (event) => console.log(event));"#)
    .build()?;
    window.open_devtools();
    window.center()?;

    window.on_window_event(|event| {
        match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                println!("Window is closing");
                //tauri::event::emit(&window, "login", Some("Hello from the login window!"));
            }
            _ => {}
        }
    });

    println!("{:?}", window.eval("console.log('This is a second test'); window.addEventListener('load', (event) => console.log(event));"));

    Ok(())
}
