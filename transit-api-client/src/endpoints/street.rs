//!
//! Holds function to get information about physical streets in the city
//!

use reqwest::Error;
use serde::Deserialize;
use std::fmt::Display;

use crate::structs::{common::Street, UrlParameter, Usage};

// TODO add type and leg filters?
impl crate::TransitClient {
    /// Returns information about physical streets in the city
    ///
    /// # Arguments
    ///
    /// * `name`: The name of the street to match.
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec<Street, Global>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::structs::Usage;
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let street = client.street("Portage Ave", Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn street(&self, name: &str, usage: Usage) -> Result<Vec<Street>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            streets: Vec<Street>,
        }

        let response = self
            .client
            .get(format!(
                "{base}/streets.json?api-key={key}{usage}&name={name}",
                base = self.base_url,
                key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for street (`{name}`): {:?}", &response);
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

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
    /// use transit_api_client::structs::Usage;
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
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
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

        Ok(out.street)
    }
}

#[cfg(test)]
mod test {
    use crate::structs::{
        common::{Street, StreetLeg, StreetType},
        Usage,
    };

    #[tokio::test]
    async fn main_street() {
        let client = crate::testing_client();
        let actual = client.street("Main Street", Usage::Normal).await.unwrap();
        let expected = vec![
            Street {
                key: 2265,
                name: "Main Street".to_string(),
                street_type: Some(StreetType::Street),
                leg: None,
            },
            Street {
                key: 3442,
                name: "St Germain Street".to_string(),
                street_type: Some(StreetType::Street),
                leg: None,
            },
        ];
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn portage() {
        let client = crate::testing_client();
        let actual = client.street("Portage Ave", Usage::Normal).await.unwrap();
        let expected = vec![
            Street {
                key: 2903,
                name: "Portage Avenue".to_string(),
                street_type: Some(StreetType::Avenue),
                leg: None,
            },
            Street {
                key: 2904,
                name: "Portage Avenue".to_string(),
                street_type: Some(StreetType::Avenue),
                leg: Some(StreetLeg::East),
            },
        ];
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
            street_type: Some(StreetType::Avenue),
            leg: Some(StreetLeg::East),
        };
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
