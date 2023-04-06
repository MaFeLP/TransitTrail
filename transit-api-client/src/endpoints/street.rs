use reqwest::Error;
use serde::Deserialize;

use crate::structs::{common::Street, UrlParameter, Usage};

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
    use crate::structs::{
        common::{Street, StreetLeg, StreetType},
        Usage,
    };

    #[tokio::test]
    async fn main_street() {
        let client = crate::testing_client();
        let actual = client
            .street("Main Street".to_string(), Usage::Normal)
            .await
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

    #[tokio::test]
    async fn portage() {
        let client = crate::testing_client();
        let actual = client
            .street("Portage Ave".to_string(), Usage::Normal)
            .await
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
