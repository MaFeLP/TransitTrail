use crate::{error_string, ClientState};
use tauri::State;
use transit_api_client::prelude::*;

#[tauri::command]
pub async fn stop_info(id: u32, client: State<'_, ClientState>) -> Result<Stop, &'static str> {
    client
        .0
        .lock()
        .await
        .stop_info(id, Usage::Normal)
        .await
        .map_err(|why| error_string(&why, "Could not get stop info"))
}
