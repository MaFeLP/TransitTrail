//!
//! Holds functions to get locations (can be an [Address], a [Monument], or an [Intersection])
//! from the API.
//!
//! [Address]: crate::structs::common::Address
//! [Monument]: crate::structs::common::Monument
//! [Intersection]: crate::structs::common::Intersection
//!

use reqwest::Error;
use serde::Deserialize;

use crate::structs::{
    common::{GeoLocation, Location},
    UrlParameter, Usage,
};

impl crate::TransitClient {
    /// Get locations near a specified position.
    ///
    /// # Arguments
    ///
    /// * `position`: The geo location of the point to find locations near.
    /// * `distance`: The distance in metres from the given point which returned locations must
    ///   fall within. (default: `100`)
    /// * `max_results`: The number of locations to return -- closer locations will be prioritized.
    ///   (default: `5`)
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec<Location>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::structs::{common::GeoLocation, Usage};
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let locations = client.locations(
    ///         &GeoLocation {
    ///             latitude: 49.895,
    ///             longitude: -97.138,
    ///         },
    ///         None,
    ///         None,
    ///         Usage::Normal
    /// ).await.expect("Could not get locations");
    /// # });
    /// ```
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
                usage = UrlParameter::from(usage),
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
    use crate::structs::{common::*, Usage};

    #[tokio::test]
    async fn locations() {
        let client = crate::testing_client();
        let position = GeoLocation {
            latitude: 49.895,
            longitude: -97.138,
        };
        let main_street = Address {
            key: 133579,
            street_number: 333,
            street: Street {
                key: 2265,
                name: "Main Street".to_string(),
                street_type: Some(StreetType::Street),
                leg: None,
            },
            centre: GeoLocation {
                latitude: 49.89491,
                longitude: -97.13763,
            },
        };
        let actual = client
            .locations(&position, None, None, Usage::Normal)
            .await
            .unwrap();
        let expected = vec![
            Location::Monument(Monument {
                key: 4152,
                name: "MTS - Corporate Head Office".to_string(),
                categories: vec!["Services: Utilities".to_string()],
                address: main_street.clone(),
            }),
            Location::Monument(Monument {
                key: 4153,
                name: "Bank of Montreal Building".to_string(),
                categories: vec!["Office Buildings".to_string()],
                address: main_street.clone(),
            }),
            Location::Address(main_street),
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
                centre: GeoLocation {
                    latitude: 49.89467,
                    longitude: -97.13801,
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
                centre: GeoLocation {
                    latitude: 49.89533,
                    longitude: -97.13809,
                },
            }),
        ];

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}
