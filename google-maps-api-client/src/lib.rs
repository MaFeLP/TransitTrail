#![warn(missing_docs, invalid_doc_attributes, missing_debug_implementations)]
#![deny(non_ascii_idents)]
//! Google Maps API Client
//!
//! This crate provides endpoints of the official Google Maps API. It requires an API-Key from the
//! API website from <https://developers.google.com/maps/documentation/directions/get-api-key>.
//!
//! # Example
//! ```
//!
//! ```

pub mod endpoints;
pub mod structs;
pub mod prelude;

use time::{macros::datetime, PrimitiveDateTime};

/// Google Maps Api Client
#[derive(Debug)]
pub struct GoogleMapsClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl GoogleMapsClient {
    /// Creates a new instance of the API Client.
    ///
    /// # Arguments
    /// * `api_key`: The API Key used to connect to the Official API
    ///
    /// returns: GoogleMapsClient
    pub fn new(api_key: String) -> Self {
        GoogleMapsClient {
            api_key,
            base_url: "https://maps.googleapis.com/maps/api/".to_string(),
            client: reqwest::Client::default(),
        }
    }

    /// Sets the base url for the API, which is used to a different API endpoint.
    ///
    /// Usually dosnt have to be set and defalts to `https://maps.googleapis.com/maps/api/`
    ///
    /// # Arguments
    ///
    /// * `base_url`: The base url to use for the API
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use google_maps_api_client::GoogleMapsClient;
    ///
    /// let key = "YOUR KEY GOES HERE".to_string();
    ///
    /// let mut client = GoogleMapsClient::new(key);
    /// client.set_base_url("https://maps.googleapis.com/maps/api/".to_string());
    /// ```
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}

/// A [PrimitiveDateTime] that represents the UNIX Epoch (January 1st, 1970 at 12:00 am).
/// This value can then be used as a default value for PrimitiveDateTimes
pub(crate) const UNIX_EPOCH: PrimitiveDateTime = datetime!(1970-01-01 0:00);

/// Creates a GoogleMapsClient instance from the GOOGLE_MAPS_API_KEY environment variable.
#[cfg(test)]
pub fn testing_client() -> GoogleMapsClient {
    // Load testing_stops environment from `.env` file
    dotenv::dotenv().unwrap();

    // Create a logging instance
    // See https://docs.rs/env_logger/latest/env_logger/#capturing-logs-in-tests for more info
    let _ = env_logger::builder().is_test(true).try_init();

    return GoogleMapsClient::new(
        std::env::var("GOOGLE_MAPS_API_KEY")
            .expect("GOOGLE_MAPS_API_KEY environment variable not set"),
    )
}