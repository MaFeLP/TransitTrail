use transit_api_client::prelude::*;
use transit_api_client::TransitClient;

use markdown;

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
async fn service_advisorie_html(
    client: tauri::State<crate::ClientState>,
    filters: Vec<filters::ServiceAdvisory>,
    header: u32,
) -> Result<String, String> {
    /* TODO: FIX, NOT CATTING ALL `**` */
    let service_advisories = client
        .client
        .service_advisories(filters, Usage::Normal)
        .await
        .unwrap();

    let mut out = String::new();
    for service_advisory in service_advisories {
        let mut temp = String::new();

        let body = service_advisory.body;
        let title = service_advisory.title;

        let pared_markdown_body = markdown::to_html(&body);

        let mut in_list = false;
        for line in pared_markdown_body.lines() {
            if line.starts_with("** ") {
                if !in_list {
                    temp.push_str("<ul>");
                    in_list = true;
                }
                temp.push_str("<li>");
                temp.push_str(&line[3..]);
                temp.push_str("</li>");
            } else {
                if in_list {
                    temp.push_str("</ul>");
                    in_list = false;
                }
                temp.push_str(line);
            }
        }

        temp.push_str(&format!("<h{}>{}</h{}>", header, title, header));
        temp.push_str(&pared_markdown_body);
        temp.push_str("<hr>");

        out.push_str(&temp);
    }

    Ok(out)
}
