//!
//! Contains functions for the Geocode API
//!

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;

impl crate::GoogleMapsClient {
    pub async fn geocode(&self, address: &str) -> Option<GeocodeResult> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
            address, self.api_key
        );

        let response = self.client.get(&url).send().await;
        if let Ok(response) = response {
            let json: JsonResult<GeocodeResponse> = response.json().await;
            if let Ok(data) = json {
                if let Some(result) = data.results.first() {
                    return Some(result.clone());
                }
            }
        }

        None
    }
}