//!
//! Holds function to get information about physical streets in the city
//!

use reqwest::Error;
use serde::Deserialize;

use crate::structs::{common::Street, UrlParameter, Usage};

// TODO add streets_by_key
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
        let out: Response = response.json().await?;
        Ok(out.streets)
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
        //dbg!("{:?},{:?}", &actual, &expected);
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
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
