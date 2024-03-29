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

use crate::ClientState;
use tauri::State;
use transit_api_client::prelude::*;

#[tauri::command]
/// Returns a string of HTML containing all service advisories.
///
/// # Arguments
/// * `filters` - A vector of filters to apply to the service advisories.
/// * `header` - The header level to use for the title of each service advisory.
///
/// # Example
/// ```rust
/// let service_advisories_html = client.service_advisories_html(Vec::new(), Usage::Normal).await.unwrap();
/// ```
pub async fn service_advisorie_html(
    client: State<'_, ClientState>,
    filters: Vec<filters::ServiceAdvisory>,
) -> Result<String, String> {
    let client = client.0.lock().await;
    let service_advisories = client
        .service_advisories(filters, Usage::Normal)
        .await
        .unwrap();

    let mut out = String::new();
    for service_advisory in service_advisories {
        let mut temp = r#"<details class="advisory">"#.to_string();

        let body = service_advisory.body;
        let title = service_advisory.title;
        temp.push_str(&format!(
            "<summary class=\"advisory-summary\">{title}</summary>"
        ));

        let pared_markdown_body = markdown::to_html(&body);

        let mut in_list = false;
        for line in pared_markdown_body.lines() {
            if let Some(stripped) = line.strip_prefix("** ") {
                if !in_list {
                    temp.push_str("<ul>");
                    in_list = true;
                }
                temp.push_str("<li>");
                temp.push_str(stripped);
                temp.push_str("</li>");
            } else {
                if in_list {
                    temp.push_str("</ul>");
                    in_list = false;
                }
                temp.push_str(line);
            }
        }

        temp.push_str(&pared_markdown_body);
        temp.push_str(r#"</details>"#);
        temp.push_str("<hr>");

        out.push_str(&temp);
    }

    Ok(out.to_string())
}
