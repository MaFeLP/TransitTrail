#![warn(missing_docs, invalid_doc_attributes, missing_debug_implementations)]
#![deny(non_ascii_idents, unused_crate_dependencies)]
//! Google Maps API Client
//!
//! This crate provides endpoints of the official Google Maps API. It requires an API-Key from the
//! API website from <https://developers.google.com/maps/documentation/directions/get-api-key>.
//!
//! # Example
//! ```
//!
//! ```

use time::{macros::datetime, PrimitiveDateTime};

pub mod endpoints;
pub mod structs;

/// Google Maps Api Client
#[derive(Debug)]
pub struct GoogleMapsClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl GoogleMapsClient {
    /// Creates a new instance of the API Client.

    pub fn new(api_key: String) -> Self {
        GoogleMapsClient {
            api_key,
            base_url: "https://maps.googleapis.com/maps/api".to_string(),
            client: reqwest::Client::default(),
        }
    }

    /// Sets the base URL of the API, when using a different endpoint.
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}

/// A [PrimitiveDateTime] that represents the UNIX Epoch (January 1st, 1970 at 12:00 am).
/// This value can then be used as a default value for PrimitiveDateTimes
pub(crate) const UNIX_EPOCH: PrimitiveDateTime = datetime!(1970-01-01 0:00);

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