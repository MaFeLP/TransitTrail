#![warn(missing_docs, invalid_doc_attributes, missing_debug_implementations)]
#![deny(non_ascii_idents, unused_crate_dependencies)]
//! Winnipeg Api Client
//!
//! This crate provides endpoints for version 3 of the official Winnipeg Transit API. It requires
//! an API-Key from the API website from <https://api.winnipegtransit.com>.
//!
//! # Example
//! ```
//! use transit_api_client::{structs::Usage, TransitClient};
//! use std::env;
//! # tokio_test::block_on(async {
//! # dotenv::dotenv().unwrap();
//!
//! let client = TransitClient::new(env::var("WPG_TRANSIT_API_KEY").expect("Expected API key"));
//! let stop = client.stop_info(10167, Usage::Normal).await.unwrap();
//! # });
//! ```

// Need to use this, so cargo doesn't complain, if we don't use it in test, but it's needed for
// doctests.
#[cfg(test)]
use tokio_test as _;

pub mod endpoints;
pub mod structs;

/// The client that houses all the methods for the API and handles connections to the API.
#[derive(Debug)]
pub struct TransitClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl TransitClient {
    /// Creates a new instance of the API Client.
    ///
    /// # Arguments
    ///
    /// * `api_key`: The API Key used to connect to the Official API
    ///
    /// returns: TransitClient
    ///
    /// # Examples
    ///
    /// ```
    /// use transit_api_client::TransitClient;
    ///
    /// let token = "YOUR_TOKEN_GOES_HERE".to_string();
    /// let client = TransitClient::new(token);
    /// ```
    pub fn new(api_key: String) -> Self {
        TransitClient {
            api_key,
            base_url: "https://api.winnipegtransit.com/v3/".to_string(),
            client: reqwest::Client::default(),
        }
    }

    /// Sets the base URL of the API, when using a different endpoint.
    ///
    /// Usually doesnt have to be set and defaults to <https://api.winnipegtransit.com/v3/>.
    ///
    /// # Arguments
    ///
    /// * `base_url`: The new URL that will be the base for all requests.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use transit_api_client::TransitClient;
    ///
    /// let token = "YOUR_TOKEN_GOES_HERE".to_string();
    ///
    /// let mut client = TransitClient::new(token);
    /// client.set_base_url("https://api.winnipegtransit.com/v2/".to_string());
    /// ```
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}

/// Creates a Transit Client from environment variables
#[cfg(test)]
pub fn testing_client() -> TransitClient {
    TransitClient::new(
        dotenv::var("WPG_TRANSIT_API_KEY")
            .expect("No environment variable `WPG_TRANSIT_API_KEY` found!"),
    )
}
