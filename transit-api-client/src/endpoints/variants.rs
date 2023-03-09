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
        let actual = rt
            .block_on(client.variants_by_stop(50254, Usage::Normal))
            .unwrap();
        let expected = vec![
            RouteVariante {
                key: "1-0".to_string(),
                name: Some("1 On-Request".to_string()),
            },
            RouteVariante {
                key: "W362-0".to_string(),
                name: Some("Employee bus".to_string()),
            },
            RouteVariante {
                key: "W322-0".to_string(),
                name: Some("Employee bus".to_string()),
            },
            RouteVariante {
                key: "W372-0".to_string(),
                name: Some("Employee bus".to_string()),
            },
            RouteVariante {
                key: "57-0-S".to_string(),
                name: Some("Southdale Express to Southdale".to_string()),
            },
            RouteVariante {
                key: "19-1-D".to_string(),
                name: Some("Marion-Logan-Notre Dame to Windsor Park via Drake".to_string()),
            },
            RouteVariante {
                key: "19-1-A".to_string(),
                name: Some("Marion-Logan-Notre Dame to Windsor Park via Autumnwood".to_string()),
            },
            RouteVariante {
                key: "W332-0".to_string(),
                name: Some("Employee bus".to_string()),
            },
        ];

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
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
        let actual = rt
            .block_on(client.variants_by_stops(vec![10652, 10907], Usage::Normal))
            .unwrap();
        let expected = vec![
            RouteVariante {
                key: "I304-0".to_string(),
                name: Some("Fort Rouge Garage".to_string()),
            },
            RouteVariante {
                key: "65-1-D".to_string(),
                name: Some("Grant Express to Downtown (City Hall)".to_string()),
            },
            RouteVariante {
                key: "S409-0-PM".to_string(),
                name: Some("to School Charter".to_string()),
            },
            RouteVariante {
                key: "38-0-T".to_string(),
                name: Some("Salter to Templeton & McPhillips".to_string()),
            },
            RouteVariante {
                key: "S425-0-PM".to_string(),
                name: Some("to From St.Paul's College (PM)".to_string()),
            },
            RouteVariante {
                key: "38-1-F".to_string(),
                name: Some("Salter to The Forks".to_string()),
            },
            RouteVariante {
                key: "66-1-D".to_string(),
                name: Some("Grant to Downtown (City Hall)".to_string()),
            },
        ];

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
