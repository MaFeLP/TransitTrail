// use transit_api_client::TransitClient;
// use transit_api_client::structs::Usage;
// use transit_api_client::prelude::*;

// // file reader
// use std::fs::File;
// use std::io::prelude::*;

// // json parser
// use serde_json;

use markdown;

#[tokio::main]
async fn main() {
    // const STOP: u32 = 11030;

    // let mut file = File::open("C:\\Users\\mfcpi\\Documents\\WT_KEY").expect("Unable to open file");
    // let mut api_key = String::new();
    // file.read_to_string(&mut api_key)
    //     .expect("Unable to read file");

    // let client = TransitClient::new(api_key);

    // let stop = client.stop_info(STOP, Usage::Long).await.unwrap();
    // let stop_json = serde_json::to_string(&stop).unwrap();
    // println!("{}", stop_json);
    // space();

    // let street = client.street(
    //     vec![
    //         filters::Street::Name("Portage"),
    //     ],
    //     Usage::Long,
    // ).await.unwrap();

    // let street_json = serde_json::to_string(&street).unwrap();
    // println!("{}", street_json);
    // space();   

    // let features = client.stop_features(STOP, Usage::Long).await.unwrap();
    // let features_json = serde_json::to_string(&features).unwrap();
    // println!("{}", features_json);
    // space();

    // let service_advisories = client.service_advisories(Vec::new(), Usage::Normal).await.unwrap();
    // let service_advisories_json = serde_json::to_string(&service_advisories).unwrap();
    // println!("{}", service_advisories_json);
    // space();

    // let routes = client.routes(Vec::new(), Usage::Normal).await.unwrap();
    // let routes_json = serde_json::to_string(&routes).unwrap();
    // println!("{}", routes_json);
    // space();
    
    let markdown = "As of May 09, 2023 07:06, route 15 is under the effects of a detour. This detour affects the following stops:\n\n* 30317 - Eastbound Mountain at Sinclair\n* 30472 - Eastbound Mountain at Airlies\n\nReroutes are as follows:\n\n* **Routes 15 (Sargent-Mountain to Airport via Flight Road) and 15 (Sargent-Mountain to Airport via Wellington)** from Mountain Avenue will reroute following these instructions:\n** Travel along Mountain Avenue and turn north on to Airlies Street\n** Continue along Airlies Street and turn east on to Church Avenue\n** Continue along Church Avenue and turn south on to Arlington Street\n** Continue along Arlington Street and turn east on to Mountain Avenue\n** Return to regular service";
    let paredMarkdown = markdown::to_html(markdown);
    let mut out = String::new();

    let mut inList = false;
    for line in paredMarkdown.lines() {
        if line.starts_with("** ") {
            if !inList {
                out.push_str("<ul>");
                inList = true;
            }
            out.push_str("<li>");
            out.push_str(&line[3..]);
            out.push_str("</li>");

        } else {
            if inList {
                out.push_str("</ul>");
                inList = false;
            }
            out.push_str(line);
        }
    }

    println!("{}", out);

    

}

fn space() {
    println!();
    println!();
}