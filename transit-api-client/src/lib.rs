pub mod endpoints;
pub mod structs;

pub struct TransitClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl TransitClient {
    pub fn new(api_key: String) -> Self {
        TransitClient {
            api_key,
            base_url: "https://api.winnipegtransit.com/v3/".to_string(),
            client: reqwest::Client::default(),
        }
    }
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}
