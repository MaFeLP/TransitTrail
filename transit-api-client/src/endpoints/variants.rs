use crate::structs::{RouteVariante, UrlParameter, Usage};
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
    pub async fn variant_by_key(&self, key: &str, usage: Usage) -> Result<RouteVariante, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variant: RouteVariante,
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
        let out: Response = response.json().await?;
        Ok(out.variant)
    }

    pub async fn variants_by_stop(
        &self,
        stop: u32,
        usage: Usage,
    ) -> Result<Vec<RouteVariante>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variants: Vec<RouteVariante>,
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
        dbg!(&response);
        let out: Response = response.json().await?;
        Ok(out.variants)
    }

    pub async fn variants_by_stops(
        &self,
        stops: Vec<u32>,
        usage: Usage,
    ) -> Result<Vec<RouteVariante>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variants: Vec<RouteVariante>,
        }

        let response = self
            .client
            .get(format!(
                "{base}/variants.json?api-key={api_key}{usage}&stops={stops_formatted}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
                stops_formatted = {
                    let mut s = String::new();
                    for stop in stops {
                        s.push_str(&stop.to_string());
                        s.push(',');
                    }
                    s.pop();
                    s
                }
            ))
            .send()
            .await?;
        let out: Response = response.json().await?;
        Ok(out.variants)
    }
}

#[cfg(test)]
mod test {
    //use super::*;
    use crate::structs::*;

    #[test]
    fn variant_by_key() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let actual = rt
            .block_on(client.variant_by_key("17-1-G", Usage::Normal))
            .unwrap();
        let expected = RouteVariante {
            key: "17-1-G".to_string(),
            name: Some("McGregor to Garden City Centre".to_string()),
        };

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn variants_by_stop() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        rt.block_on(client.variants_by_stop(50254, Usage::Normal))
            .unwrap();
    }

    #[test]
    fn variants_by_stops() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        rt.block_on(client.variants_by_stops(vec![10652, 10907], Usage::Normal))
            .unwrap();
    }
}
