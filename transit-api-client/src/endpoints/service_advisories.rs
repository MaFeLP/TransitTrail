//!
//! Holds methods to get information about service advisories
//!

use reqwest::Error;
use serde::Deserialize;

use crate::structs::{
    service_advisories::{Category, Priority, ServiceAdvisory},
    UrlParameter, Usage,
};

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
    /// use transit_api_client::structs::Usage;
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
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
        let out: Response = response.json().await?;
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
    /// returns: Result<Vec<ServiceAdvisory>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::structs::Usage;
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let advisories = client.service_advisories(None, None, None, None, Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn service_advisories(
        &self,
        priority: Option<Priority>,
        category: Option<Category>,
        max_age: Option<u32>,
        limit: Option<u32>,
        usage: Usage,
    ) -> Result<Vec<ServiceAdvisory>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "service-advisories")]
            service_adviory: Vec<ServiceAdvisory>,
        }

        // Format only given parameters, create empty strings for None variants
        let priority: UrlParameter = priority.into();
        let category: UrlParameter = category.into();
        let max_age = max_age.map(|a| format!("&max_age={a}")).unwrap_or_default();
        let limit = limit.map(|l| format!("&limit={l}")).unwrap_or_default();

        let response = self
            .client
            .get(format!(
                "{base}/service-advisories.json?api-key={api_key}{usage}{priority}{category}{max_age}{limit}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        dbg!(&response);
        let text = response.text().await?;
        dbg!(&text);
        let out: Response = serde_json::from_str(text.as_str()).unwrap();
        Ok(out.service_adviory)
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::structs::{
        service_advisories::{Category, Priority, ServiceAdvisory},
        Usage,
    };

    #[tokio::test]
    async fn service_adviory() {
        let client = crate::testing_client();
        let actual = client.service_advisory(96, Usage::Normal).await.unwrap();
        let expected = ServiceAdvisory {
            key: 96,
            priority: Priority::VeryHigh,
            title: "Blue Priority Service".to_string(),
            body: "Winnipeg Transit is operating a Blue Priority Service. Please check the website or call 311 for information on service delays and route cancellations. ".to_string(),
            category: Category::Transit,
            updated_at: NaiveDateTime::new(NaiveDate::from_ymd_opt(2009, 2, 10).unwrap(), NaiveTime::from_hms_opt(15, 41, 30).unwrap()),
        };
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn service_advisories() {
        let client = crate::testing_client();
        let actual = client
            .service_advisories(None, None, None, Some(3), Usage::Normal)
            .await
            .unwrap();
        // Can only test serialization, as advisories from this query change often. Unit tests
        // would therefore fail automatically after a while.
        assert_eq!(actual.len(), 3);
    }
}
