use crate::structs::{routes::Route, UrlParameter, Usage};
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
    pub async fn route<T: std::fmt::Display>(
        &self,
        route_number: T,
        usage: Usage,
    ) -> Result<Route, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            route: Route,
        }

        let response = self
            .client
            .get(format!(
                "{base}/routes/{route_number}.json?api-key={key}{usage}",
                base = self.base_url,
                key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        let out: Response = response.json().await?;
        Ok(out.route)
    }

    pub async fn routes_by_stop(
        &self,
        stop_number: u32,
        usage: Usage,
    ) -> Result<Vec<Route>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            routes: Vec<Route>,
        }

        let response = self
            .client
            .get(format!(
                "{base}/routes.json?api-key={key}{usage}&stop={stop_number}",
                base = self.base_url,
                key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        let out: Response = response.json().await?;
        Ok(out.routes)
    }
}

#[cfg(test)]
mod test {
    use crate::structs::{
        routes::{
            badges::{ClassNames, Style},
            {Route, RouteBlue, RouteCoverage, RouteCustomer, RouteRegular, RouteVariante},
        },
        Usage,
    };

    #[test]
    fn normal_route() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let actual = rt.block_on(client.route(25, Usage::Normal)).unwrap();
        let expected = Route::Regular(RouteRegular {
            key: 25,
            number: 25,
            name: "Route 25 Ness Super Express".to_string(),
            customer_type: RouteCustomer::Regular,
            coverage: RouteCoverage::SuperExpress,
            badge_label: 25,
            badge_style: Style {
                class_names: ClassNames {
                    class_name: vec!["badge-label".to_string(), "express".to_string()],
                },
                background_color: "#eed700".to_string(),
                border_color: "#cab700".to_string(),
                color: "#000000".to_string(),
            },
            variants: Some(vec![
                RouteVariante {
                    key: "25-0-U".to_string(),
                    name: None,
                },
                RouteVariante {
                    key: "25-1-D".to_string(),
                    name: None,
                },
            ]),
        });

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn normal_routes_by_stop() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let actual = rt
            .block_on(client.routes_by_stop(50254, Usage::Normal))
            .unwrap();
        let expected = vec![
            Route::Regular(RouteRegular {
                key: 57,
                number: 57,
                name: "Route 57 Southdale Express".to_string(),
                customer_type: RouteCustomer::Regular,
                coverage: RouteCoverage::Express,
                badge_label: 57,
                badge_style: Style {
                    class_names: ClassNames {
                        class_name: vec!["badge-label".to_string(), "express".to_string()],
                    },
                    background_color: "#eed700".to_string(),
                    border_color: "#cab700".to_string(),
                    color: "#000000".to_string(),
                },
                variants: Some(vec![
                    RouteVariante {
                        key: "57-1-D".to_string(),
                        name: None,
                    },
                    RouteVariante {
                        key: "57-0-S".to_string(),
                        name: None,
                    },
                ]),
            }),
            Route::Regular(RouteRegular {
                key: 19,
                number: 19,
                name: "Route 19 Marion-Logan-Notre Dame".to_string(),
                customer_type: RouteCustomer::Regular,
                coverage: RouteCoverage::Regular,
                badge_label: 19,
                badge_style: Style {
                    class_names: ClassNames {
                        class_name: vec!["badge-label".to_string(), "regular".to_string()],
                    },
                    background_color: "#ffffff".to_string(),
                    border_color: "#d9d9d9".to_string(),
                    color: "#000000".to_string(),
                },
                variants: Some(vec![
                    RouteVariante {
                        key: "19-0-#".to_string(),
                        name: None,
                    },
                    RouteVariante {
                        key: "19-1-N".to_string(),
                        name: None,
                    },
                    RouteVariante {
                        key: "19-0-N".to_string(),
                        name: None,
                    },
                    RouteVariante {
                        key: "19-0-E".to_string(),
                        name: None,
                    },
                    RouteVariante {
                        key: "19-1-D".to_string(),
                        name: None,
                    },
                    RouteVariante {
                        key: "19-1-A".to_string(),
                        name: None,
                    },
                    RouteVariante {
                        key: "19-0-L".to_string(),
                        name: None,
                    },
                ]),
            }),
        ];

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn blue_route() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        let actual = rt.block_on(client.route("BLUE", Usage::Normal)).unwrap();
        let expected = Route::Blue(RouteBlue {
            key: "BLUE".to_string(),
            number: "BLUE".to_string(),
            customer_type: RouteCustomer::Regular,
            coverage: RouteCoverage::RapidTransit,
            badge_label: "B".to_string(),
            badge_style: Style {
                class_names: ClassNames {
                    class_name: vec!["badge-label".to_string(), "rapid-transit".to_string()],
                },
                background_color: "#0060a9".to_string(),
                border_color: "#0060a9".to_string(),
                color: "#ffffff".to_string(),
            },
            variants: Some(vec![
                RouteVariante {
                    key: "BLUE-0-S".to_string(),
                    name: None,
                },
                RouteVariante {
                    key: "BLUE-0-U".to_string(),
                    name: None,
                },
                RouteVariante {
                    key: "BLUE-1-D".to_string(),
                    name: None,
                },
            ]),
        });

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
