use std::collections::HashMap;

use scraper::Html;

use reqwest::{Client, ClientBuilder};

mod utils;
use crate::utils::{map_to_reqwest_headers, Headers};

#[derive(Debug)]
pub struct Skrobak {
    client: Client,
}

impl Default for Skrobak {
    fn default() -> Self {
        let headers: Headers = HashMap::new();

        Self {
            client: ClientBuilder::default()
                .default_headers(map_to_reqwest_headers(&headers))
                .build()
                .unwrap(),
        }
    }
}

impl Skrobak {
    pub async fn parse_web_page_from_url(&self, url: &str) -> Result<Html, reqwest::Error> {
        let response = self.client.get(url).send().await?.text().await?;
        Ok(Html::parse_document(&response))
    }
}
