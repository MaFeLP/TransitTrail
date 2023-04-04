use crate::structs::{destinations::Destination, UrlParameter, Usage};
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
    pub async fn destinations(
        &self,
        route: String,
        usage: Usage,
    ) -> Result<Vec<Destination>, Error> {
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
        let out: Response = response.json().await?;

        Ok(out.destinations)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn destinations() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let actual = rt
            .block_on(client.destinations("16-1-K".to_string(), Usage::Normal))
            .unwrap();
        let expected = vec![
            Destination {
                key: 10,
                name: "City Hall".to_string(),
            },
            Destination {
                key: 164,
                name: "Kingston Row".to_string(),
            },
            Destination {
                key: 5,
                name: "Downtown".to_string(),
            },
        ];

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
