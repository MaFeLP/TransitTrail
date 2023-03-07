use crate::structs::{GeoLocation, Location, Usage};
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
    pub async fn locations(
        &self,
        position: &GeoLocation,
        distance: Option<f32>,
        max_results: Option<u32>,
        usage: Usage,
    ) -> Result<Vec<Location>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            locations: Vec<Location>,
        }

        let response = self
            .client
            .get(format!(
                "{base}locations.json?api-key={key}{usage}&lat={lat}&lon={long}&distance={distance}&max-results={max_results}",
                base = self.base_url,
                key = self.api_key,
                usage = usage.to_url_parameter(),
                lat = position.latitude,
                long = position.longitude,
                distance = distance.unwrap_or(100.0),
                max_results = max_results.unwrap_or(5),
            ))
            .send()
            .await?;
        let out: Response = response.json().await?;
        //let out: Response = serde_json::from_str(text.as_str()).unwrap();

        Ok(out.locations)
    }
}

#[cfg(test)]
mod test {
    //use super::*;
    use crate::structs::*;

    #[test]
    fn locations() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let position = GeoLocation {
            latitude: "49.895".to_string(),
            longitude: "-97.138".to_string(),
        };
        let actual = rt
            .block_on(client.locations(&position, None, None, Usage::Normal))
            .unwrap();
        let expected = vec![
            Location::Monument(Monument {
                key: 4152,
                name: "MTS - Corporate Head Office".to_string(),
                categories: vec!["Services: Utilities".to_string()],
                address: Address {
                    key: 133579,
                    street_number: 333,
                    street: Street {
                        key: 2265,
                        name: "Main Street".to_string(),
                        street_type: Some(StreetType::Street),
                        leg: None,
                    },
                    centre: GeographicLocation {
                        geographic: GeoLocation {
                            latitude: "49.89491".to_string(),
                            longitude: "-97.13763".to_string(),
                        },
                    },
                },
            }),
            Location::Monument(Monument {
                key: 4153,
                name: "Bank of Montreal Building".to_string(),
                categories: vec!["Office Buildings".to_string()],
                address: Address {
                    key: 133579,
                    street_number: 333,
                    street: Street {
                        key: 2265,
                        name: "Main Street".to_string(),
                        street_type: Some(StreetType::Street),
                        leg: None,
                    },
                    centre: GeographicLocation {
                        geographic: GeoLocation {
                            latitude: "49.89491".to_string(),
                            longitude: "-97.13763".to_string(),
                        },
                    },
                },
            }),
            Location::Address(Address {
                key: 133579,
                street_number: 333,
                street: Street {
                    key: 2265,
                    name: "Main Street".to_string(),
                    street_type: Some(StreetType::Street),
                    leg: None,
                },
                centre: GeographicLocation {
                    geographic: GeoLocation {
                        latitude: "49.89491".to_string(),
                        longitude: "-97.13763".to_string(),
                    },
                },
            }),
            Location::Intersection(Intersection {
                key: "41059:2265@2871".to_string(),
                street: Street {
                    key: 2265,
                    name: "Main Street".to_string(),
                    street_type: Some(StreetType::Street),
                    leg: None,
                },
                cross_street: Street {
                    key: 2871,
                    name: "Pioneer Avenue".to_string(),
                    street_type: Some(StreetType::Avenue),
                    leg: None,
                },
                centre: GeographicLocation {
                    geographic: GeoLocation {
                        latitude: "49.89467".to_string(),
                        longitude: "-97.13801".to_string(),
                    },
                },
            }),
            Location::Intersection(Intersection {
                key: "6007530:70002356@70002355".to_string(),
                street: Street {
                    key: 70002356,
                    name: "Walkway: Portage and Main".to_string(),
                    street_type: None,
                    leg: None,
                },
                cross_street: Street {
                    key: 70002355,
                    name: "Walkway: Winnipeg Square".to_string(),
                    street_type: None,
                    leg: None,
                },
                centre: GeographicLocation {
                    geographic: GeoLocation {
                        latitude: "49.89533".to_string(),
                        longitude: "-97.13809".to_string(),
                    },
                },
            }),
        ];

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
