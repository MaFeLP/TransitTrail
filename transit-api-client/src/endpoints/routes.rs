use reqwest::Error;
use serde::Deserialize;

use crate::structs::{routes::Route, UrlParameter, Usage};

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
            {Blue, Coverage, Customer, Regular, Route, Variant},
            badges::{ClassNames, Style},
        },
        Usage,
    };

    #[tokio::test]
    async fn normal_route() {
        let client = crate::testing_client();
        let actual = client.route(25, Usage::Normal).await.unwrap();
        let expected = Route::Regular(Regular {
            key: 25,
            number: 25,
            name: "Route 25 Ness Super Express".to_string(),
            customer_type: Customer::Regular,
            coverage: Coverage::SuperExpress,
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
                Variant {
                    key: "25-0-U".to_string(),
                    name: None,
                },
                Variant {
                    key: "25-1-D".to_string(),
                    name: None,
                },
            ]),
        });

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn normal_routes_by_stop() {
        let client = crate::testing_client();
        let actual = client.routes_by_stop(50254, Usage::Normal).await.unwrap();
        let expected = vec![
            Route::Regular(Regular {
                key: 57,
                number: 57,
                name: "Route 57 Southdale Express".to_string(),
                customer_type: Customer::Regular,
                coverage: Coverage::Express,
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
                    Variant {
                        key: "57-1-D".to_string(),
                        name: None,
                    },
                    Variant {
                        key: "57-0-S".to_string(),
                        name: None,
                    },
                ]),
            }),
            Route::Regular(Regular {
                key: 19,
                number: 19,
                name: "Route 19 Marion-Logan-Notre Dame".to_string(),
                customer_type: Customer::Regular,
                coverage: Coverage::Regular,
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
                    Variant {
                        key: "19-0-#".to_string(),
                        name: None,
                    },
                    Variant {
                        key: "19-1-N".to_string(),
                        name: None,
                    },
                    Variant {
                        key: "19-0-N".to_string(),
                        name: None,
                    },
                    Variant {
                        key: "19-0-E".to_string(),
                        name: None,
                    },
                    Variant {
                        key: "19-1-D".to_string(),
                        name: None,
                    },
                    Variant {
                        key: "19-1-A".to_string(),
                        name: None,
                    },
                    Variant {
                        key: "19-0-L".to_string(),
                        name: None,
                    },
                ]),
            }),
        ];

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn blue_route() {
        let client = crate::testing_client();
        let actual = client.route("BLUE", Usage::Normal).await.unwrap();
        let expected = Route::Blue(Blue {
            key: "BLUE".to_string(),
            number: "BLUE".to_string(),
            customer_type: Customer::Regular,
            coverage: Coverage::RapidTransit,
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
                Variant {
                    key: "BLUE-0-S".to_string(),
                    name: None,
                },
                Variant {
                    key: "BLUE-0-U".to_string(),
                    name: None,
                },
                Variant {
                    key: "BLUE-1-D".to_string(),
                    name: None,
                },
            ]),
        });

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
