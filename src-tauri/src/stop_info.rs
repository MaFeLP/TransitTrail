// TransitTrail - Navigate Winnipeg Transit with a different style
// Copyright (C) - 2023 Foxx Azalea Pinkerton, Max Fehlinger
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

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
