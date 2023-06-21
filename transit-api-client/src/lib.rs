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

use time::{macros::datetime, PrimitiveDateTime};

pub mod endpoints;
pub mod filters;
pub mod prelude;
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
            base_url: "https://api.winnipegtransit.com/v3".to_string(),
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

/// A [PrimitiveDateTime] that represents the UNIX Epoch (January 1st, 1970 at 12:00 am).
/// This value can then be used as a default value for PrimitiveDateTimes
pub(crate) const UNIX_EPOCH: PrimitiveDateTime = datetime!(1970-01-01 0:00);

/// Creates a Transit Client from environment variables
#[cfg(test)]
pub fn testing_client() -> TransitClient {
    // Load testing_stops environment from `.env` file
    dotenv::dotenv().unwrap();

    // Create a logging instance
    // See https://docs.rs/env_logger/latest/env_logger/#capturing-logs-in-tests for more info
    let _ = env_logger::builder().is_test(true).try_init();

    // Create and return a default instance of the TransitClient
    TransitClient::new(
        std::env::var("WPG_TRANSIT_API_KEY")
            .expect("No environment variable `WPG_TRANSIT_API_KEY` found!"),
    )
}
