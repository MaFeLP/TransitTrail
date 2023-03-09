use crate::structs::{Street, UrlParameter, Usage};
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
    pub async fn street(&self, name: String, usage: Usage) -> Result<Vec<Street>, Error> {
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
    //use super::*;
    use crate::structs::*;

    #[test]
    fn main_street() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let actual = rt
            .block_on(client.street("Main Street".to_string(), Usage::Normal))
            .unwrap();
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

    #[test]
    fn portage() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let actual = rt
            .block_on(client.street("Portage Ave".to_string(), Usage::Normal))
            .unwrap();
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
