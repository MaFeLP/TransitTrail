//!
//! Holds methods to get information about service advisories
//!

use serde::Deserialize;

use crate::filters;
use crate::structs::{service_advisories::ServiceAdvisory, Error, UrlParameter, Usage};

impl crate::TransitClient {
    /// Get information about a specified service advisory
    ///
    /// # Arguments
    ///
    /// * `key`: The unique key of the service advisory to get information about.
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<ServiceAdvisory, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let advisory = client.service_advisory(96, Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn service_advisory(&self, key: u32, usage: Usage) -> Result<ServiceAdvisory, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "service-advisory")]
            service_advisory: ServiceAdvisory,
        }

        let response = self
            .client
            .get(format!(
                "{base}/service-advisories/{key}.json?api-key={api_key}{usage}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!(
            "Got response for service_advisory (#{key}): {:?}",
            &response
        );
        let text = response.text().await?;
        log::debug!("Response body for service_advisory (#{key}): {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.service_advisory)
    }

    /// Get recent service advisories
    ///
    /// # Arguments
    ///
    /// * `priority`: Only return service advisories of this priority or higher.
    ///   (default: [Priority::VeryLow])
    /// * `category`: Only return service advisories of this category (default: [Category::All])
    /// * `max_age`: Only returns advisories created or updated in the last N days.
    /// * `limit`: Only show the top N service advisories -- no more than the given limit.
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec\<ServiceAdvisory\>, Error>
    ///
    /// [Priority::VeryLow]: crate::structs::service_advisories::Priority::VeryLow
    /// [Category::All]: crate::structs::service_advisories::Category::All
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let advisories = client.service_advisories(Vec::new(), Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn service_advisories(
        &self,
        filters: Vec<filters::ServiceAdvisory>,
        usage: Usage,
    ) -> Result<Vec<ServiceAdvisory>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "service-advisories")]
            service_advisory: Vec<ServiceAdvisory>,
        }

        let mut filter_parameters = String::new();
        for filter in &filters {
            filter_parameters.push_str(&UrlParameter::from(filter.clone()).0);
        }

        let response = self
            .client
            .get(format!(
                "{base}/service-advisories.json?api-key={api_key}{usage}{filter_parameters}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!(
            "Got response for service advisories (filters: {filters:?}: {:?}",
            &response
        );
        let text = response.text().await?;
        log::debug!("Response body for service_advisory (filter: {filters:?}): {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.service_advisory)
    }
}

#[cfg(test)]
mod test {
    use time::macros::datetime;

    use crate::prelude::*;

    #[tokio::test]
    async fn service_advisory() {
        let client = crate::testing_client();
        let actual = client.service_advisory(96, Usage::Normal).await.unwrap();
        let expected = ServiceAdvisory {
            key: 96,
            priority: Priority::VeryHigh,
            title: "Blue Priority Service".to_string(),
            body: "Winnipeg Transit is operating a Blue Priority Service. Please check the website or call 311 for information on service delays and route cancellations. ".to_string(),
            category: Category::Transit,
            updated_at: datetime!(2009-02-10 15:41:30),
        };
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn service_advisories() {
        let client = crate::testing_client();
        let actual = client
            .service_advisories(vec![filters::ServiceAdvisory::Limit(3)], Usage::Normal)
            .await
            .unwrap();
        // Can only test serialization, as advisories from this query change often. Unit tests
        // would therefore fail automatically after a while.
        log::info!("actual={:?}", &actual);
        assert_eq!(actual.len(), 3);
    }
}
