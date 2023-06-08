use reqwest;
use serde::{Deserialize, Serialize};
use crate::prelude::{GeocodeResponse, GeocodeResult};

impl crate::GoogleMapsClient {
    /// Geocode an address
    ///
    /// # Arguments
    ///
    /// * `address`: The address to geocode
    ///
    /// returns: Result<Option<GeocodeResult>, Error>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub async fn geocode(&self, address: &str) -> Result<Option<GeocodeResult>, reqwest::Error> {
        let url = format!(
            "{}geocode/json?address={}&key={}",
            self.base_url,
            address,
            self.api_key
        );

        let response = self.client.get(&url).send().await?;
        let json: GeocodeResponse = response.json().await?;
        if let Some(result) = json.results.first().cloned() {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}