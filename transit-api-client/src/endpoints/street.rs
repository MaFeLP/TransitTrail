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

//!
//! Holds function to get information about physical streets in the city
//!

use serde::Deserialize;
use std::fmt::Display;

use crate::filters;
use crate::structs::{common::Street, Error, UrlParameter, Usage};

impl crate::TransitClient {
    /// Returns information about physical streets in the city
    ///
    /// # Arguments
    ///
    /// * `filters`: The filters to apply to the street search.
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec<Street>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// client
    ///     .street(
    ///         vec![
    ///             filters::Street::Name("Portage Ave"),
    ///             filters::Street::Type("Avenue"),
    ///             filters::Street::Leg(StreetLeg::East),
    ///         ],
    ///         Usage::Normal,
    ///     )
    ///     .await
    ///     .unwrap();
    /// # });
    /// ```
    pub async fn street(
        &self,
        filters: Vec<filters::Street<'_>>,
        usage: Usage,
    ) -> Result<Vec<Street>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            streets: Vec<Street>,
        }

        let mut filter_parameters = String::new();
        for filter in filters {
            filter_parameters.push_str(UrlParameter::from(filter).0.as_str())
        }

        let response = self
            .client
            .get(format!(
                "{base}/streets.json?api-key={key}{usage}{filter_parameters}",
                base = self.base_url,
                key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for street: {:?}", &response);
        let text = response.text().await?;
        log::debug!("Response body for street: {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.streets)
    }

    /// Returns a street by the given key.
    ///
    /// # Arguments
    ///
    /// * `key`: The key of the street, to get more information about
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec<Street>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let street = client.street_by_key(2094, Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn street_by_key<T>(&self, key: T, usage: Usage) -> Result<Street, Error>
    where
        T: Display,
    {
        #[derive(Debug, Deserialize)]
        struct Response {
            street: Street,
        }

        let response = self
            .client
            .get(format!(
                "{base}/streets/{key}.json?api-key={api_key}{usage}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for street (`key={key}`): {:?}", &response);
        let text = response.text().await?;
        log::debug!("Response body is: {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.street)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[tokio::test]
    async fn main_street() {
        let client = crate::testing_client();
        let actual = client
            .street(vec![filters::Street::Name("Main Street")], Usage::Normal)
            .await
            .unwrap();
        let expected = vec![
            Street {
                key: 2265,
                name: "Main Street".to_string(),
                street_type: Some("Street".to_string()),
                leg: None,
            },
            Street {
                key: 3442,
                name: "St Germain Street".to_string(),
                street_type: Some("Street".to_string()),
                leg: None,
            },
        ];
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn portage() {
        let client = crate::testing_client();
        let actual = client
            .street(vec![filters::Street::Name("Portage Ave")], Usage::Normal)
            .await
            .unwrap();
        let expected = vec![
            Street {
                key: 2903,
                name: "Portage Avenue".to_string(),
                street_type: Some("Avenue".to_string()),
                leg: None,
            },
            Street {
                key: 2904,
                name: "Portage Avenue".to_string(),
                street_type: Some("Avenue".to_string()),
                leg: Some(StreetLeg::East),
            },
        ];
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn portage_east_filter() {
        let client = crate::testing_client();
        let actual = client
            .street(
                vec![
                    filters::Street::Name("Portage Ave"),
                    filters::Street::Type("Avenue"),
                    filters::Street::Leg(StreetLeg::East),
                ],
                Usage::Normal,
            )
            .await
            .unwrap();
        let expected = vec![Street {
            key: 2904,
            name: "Portage Avenue".to_string(),
            street_type: Some("Avenue".to_string()),
            leg: Some(StreetLeg::East),
        }];
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn street_by_key() {
        let client = crate::testing_client();
        let actual = client.street_by_key(2904, Usage::Normal).await.unwrap();
        let expected = Street {
            key: 2904,
            name: "Portage Avenue".to_string(),
            street_type: Some("Avenue".to_string()),
            leg: Some(StreetLeg::East),
        };
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
