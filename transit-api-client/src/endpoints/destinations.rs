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
//! Holds functions for getting destinations
//!

use serde::Deserialize;

use crate::structs::{destinations::Destination, Error, UrlParameter, Usage};

impl crate::TransitClient {
    /// Returns destinations for the requested variant. These destinations are important landmarks
    /// which buses on the variant will pass.
    ///
    /// # Arguments
    ///
    /// * `route`: The route for which to request Destinations. Is a [Variant]'s key
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// [Variant]: crate::structs::routes::Variant
    ///
    /// returns: Result<Vec\<Destination\>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// # tokio_test::block_on(async {
    /// let destinations = client.destinations("16-1-K", Usage::Normal).await.unwrap();
    /// # })
    /// ```
    pub async fn destinations(&self, route: &str, usage: Usage) -> Result<Vec<Destination>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            destinations: Vec<Destination>,
        }

        let response = self
            .client
            .get(format!(
                "{base}/variants/{route}/destinations.json?api-key={key}{usage}",
                base = self.base_url,
                key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for destinations: {response:?}");
        let text = response.text().await?;
        log::info!("Response body for destinations: {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.destinations)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[tokio::test]
    async fn destinations() {
        let client = crate::testing_client();
        let actual = client.destinations("16-1-K", Usage::Normal).await.unwrap();
        let expected = vec![
            Destination {
                key: 5,
                name: "Downtown".to_string(),
            },
            Destination {
                key: 10,
                name: "City Hall".to_string(),
            },
            Destination {
                key: 164,
                name: "Kingston Row".to_string(),
            },
        ];

        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
