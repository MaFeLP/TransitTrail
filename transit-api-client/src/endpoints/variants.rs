//!
//! Holds functions to get information about a variant
//!

use reqwest::Error;
use serde::Deserialize;

use crate::structs::{routes::Variant, UrlParameter, Usage};

impl crate::TransitClient {
    /// Get information about a variant
    ///
    /// # Arguments
    ///
    /// * `key`: The variant's unique key, to get information about
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Variant, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let variant = client.variant_by_key("17-1-G", Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn variant_by_key(&self, key: &str, usage: Usage) -> Result<Variant, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variant: Variant,
        }

        let response = self
            .client
            .get(format!(
                "{base}/variants/{key}.json?api-key={api_key}{usage}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for variant (key: {key}): {:?}", &response);
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

        Ok(out.variant)
    }

    /// Get all variants that service a stop
    ///
    /// # Arguments
    ///
    /// * `stop`: The stop number to get the variants about
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec<Variant, Global>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let variants = client.variants_by_stop(50254, Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn variants_by_stop(&self, stop: u32, usage: Usage) -> Result<Vec<Variant>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variants: Vec<Variant>,
        }

        let response = self
            .client
            .get(format!(
                "{base}/variants.json?api-key={api_key}{usage}&stop={stop}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for variants (stop #{stop}): {:?}", &response);
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

        Ok(out.variants)
    }

    /// Get all the variants that service **all** of the stops
    ///
    /// # Arguments
    ///
    /// * `stops`: A vector of stops that filter the variants
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec<Variant, Global>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let variants = client.variants_by_stops(vec![10652, 10907], Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn variants_by_stops(
        &self,
        stops: Vec<u32>,
        usage: Usage,
    ) -> Result<Vec<Variant>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variants: Vec<Variant>,
        }

        let stops_formatted = {
            let mut s = String::new();
            for stop in stops {
                s.push_str(&stop.to_string());
                s.push(',');
            }
            s.pop();
            s
        };
        let response = self
            .client
            .get(format!(
                "{base}/variants.json?api-key={api_key}{usage}&stops={stops_formatted}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!(
            "Got response for variants (stop #s: {stops_formatted}): {:?}",
            &response
        );
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

        Ok(out.variants)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[tokio::test]
    async fn variant_by_key() {
        let client = crate::testing_client();
        let actual = client
            .variant_by_key("17-1-G", Usage::Normal)
            .await
            .unwrap();
        let expected = Variant {
            key: "17-1-G".to_string(),
            name: Some("McGregor to Garden City Centre".to_string()),
        };

        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn variants_by_stop() {
        let client = crate::testing_client();
        let actual = client.variants_by_stop(50254, Usage::Normal).await.unwrap();
        log::info!("actual={:?}", &actual);
    }

    #[tokio::test]
    async fn variants_by_stops() {
        let client = crate::testing_client();
        let actual = client
            .variants_by_stops(vec![10652, 10907], Usage::Normal)
            .await
            .unwrap();
        log::info!("actual={:?}", &actual);
    }
}
