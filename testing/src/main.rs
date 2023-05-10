use transit_api_client::TransitClient;
use transit_api_client::structs::Usage;
use transit_api_client::prelude::*;

// file reader
use std::fs::File;
use std::io::prelude::*;

// json parser
use serde_json;

//use markdown;

#[tokio::main]
async fn main() {
    // const STOP: u32 = 11030;

    let mut file: File = File::open("C:\\Users\\mfcpi\\Documents\\WT_KEY").expect("Unable to open file");
    let mut api_key: String = String::new();
    file.read_to_string(&mut api_key)
        .expect("Unable to read file");

    let client: TransitClient = TransitClient::new(api_key);

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

    let service_advisories: Vec<ServiceAdvisory> = client.service_advisories(Vec::new(), Usage::Normal).await.unwrap();
    //println!("{:?}", service_advisories);
    // let service_advisories_json: String = serde_json::to_string(&service_advisories).unwrap();
    // println!("{}", service_advisories_json);
    space();

    // let routes = client.routes(Vec::new(), Usage::Normal).await.unwrap();
    // let routes_json = serde_json::to_string(&routes).unwrap();
    // println!("{}", routes_json);
    // space();
    
    // let markdown = "As of May 09, 2023 07:06, route 15 is under the effects of a detour. This detour affects the following stops:\n\n* 30317 - Eastbound Mountain at Sinclair\n* 30472 - Eastbound Mountain at Airlies\n\nReroutes are as follows:\n\n* **Routes 15 (Sargent-Mountain to Airport via Flight Road) and 15 (Sargent-Mountain to Airport via Wellington)** from Mountain Avenue will reroute following these instructions:\n** Travel along Mountain Avenue and turn north on to Airlies Street\n** Continue along Airlies Street and turn east on to Church Avenue\n** Continue along Church Avenue and turn south on to Arlington Street\n** Continue along Arlington Street and turn east on to Mountain Avenue\n** Return to regular service";
    // let pared_markdown = markdown::to_html(markdown);
    // let mut out = String::new();

    // let mut in_list = false;
    // for line in pared_markdown.lines() {
    //     if line.starts_with("** ") {
    //         if !in_list {
    //             out.push_str("<ul>");
    //             in_list = true;
    //         }
    //         out.push_str("<li>");
    //         out.push_str(&line[3..]);
    //         out.push_str("</li>");

    //     } else {
    //         if in_list {
    //             out.push_str("</ul>");
    //             in_list = false;
    //         }
    //         out.push_str(line);
    //     }
    // }

    // println!("{}", out);

    const HEADER: u32 = 3;
    const FILTERS: Vec<filters::ServiceAdvisory> = Vec::new();

    let service_advisories: Vec<ServiceAdvisory> = client.service_advisories(FILTERS, Usage::Normal).await.unwrap();
  
    let mut out: String = String::new();
    for service_advisory in service_advisories {
      let mut temp: String = String::new();
  
      let body: String = service_advisory.body;
      let title: String = service_advisory.title;
  
      let pared_markdown_body: String = markdown::to_html(&body);
  
      temp.push_str(&format!("<h{}>{}</h{}>", HEADER, title, HEADER));
      temp.push_str(&pared_markdown_body);
      temp.push_str("<hr>");
  
      out.push_str(&temp);
    }

    const PATH: &str = "C:\\Users\\mfcpi\\Documents\\service_advisories.html";

    let mut file: File = File::create(PATH).expect("Unable to create file");
    file.write_all(out.as_bytes()).expect("Unable to write data");


}

fn space() {
    println!();
    println!();
}