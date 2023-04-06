use reqwest::Error;
use serde::Deserialize;

use crate::structs::{routes::Variant, UrlParameter, Usage};

impl crate::TransitClient {
    pub async fn variant_by_key(&self, key: &str, usage: Usage) -> Result<Variant, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variant: Variant,
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

    pub async fn variants_by_stop(&self, stop: u32, usage: Usage) -> Result<Vec<Variant>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variants: Vec<Variant>,
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
    ) -> Result<Vec<Variant>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            variants: Vec<Variant>,
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
    use crate::structs::{routes::Variant, Usage};

    #[tokio::test]
    async fn variant_by_key() {
        let client = crate::testing_client();
        let actual = client
            .variant_by_key("17-1-G", Usage::Normal)
            .await
            .unwrap();
        let expected = Variant {
            key: "17-1-G".to_string(),
            name: Some("McGregor to Garden City Centre".to_string()),
        };

        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn variants_by_stop() {
        let client = crate::testing_client();
        client.variants_by_stop(50254, Usage::Normal).await.unwrap();
    }

    #[tokio::test]
    async fn variants_by_stops() {
        let client = crate::testing_client();
        client
            .variants_by_stops(vec![10652, 10907], Usage::Normal)
            .await
            .unwrap();
    }
}
